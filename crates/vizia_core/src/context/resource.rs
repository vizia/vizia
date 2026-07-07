use hashbrown::{HashSet, hash_map::Entry};

use vizia_storage::Tree;

use crate::{
    entity::Entity,
    resource::{ImageOrSvg, ImageRetentionPolicy, ResourceManager, StoredImage},
    style::Style,
};

use super::{Context, ContextProxy, EventProxy};

#[cfg(feature = "tokio")]
use std::sync::Arc;

/// A context used when loading images.
pub struct ResourceContext<'a> {
    pub(crate) current: Entity,
    pub(crate) event_proxy: &'a Option<Box<dyn EventProxy>>,
    pub(crate) resource_manager: &'a mut ResourceManager,
    pub(crate) style: &'a mut Style,
    pub(crate) tree: &'a Tree<Entity>,
    #[cfg(feature = "tokio")]
    pub(crate) task_runtime: Arc<tokio::runtime::Runtime>,
    #[cfg(feature = "tokio")]
    pub(crate) named_tasks: crate::context::NamedTaskMap,
}

impl<'a> ResourceContext<'a> {
    /// Creates a new [ResourceContext].
    pub(crate) fn new(cx: &'a mut Context) -> Self {
        Self {
            current: cx.current,
            event_proxy: &cx.event_proxy,
            resource_manager: &mut cx.resource_manager,
            style: &mut cx.style,
            tree: &cx.tree,
            #[cfg(feature = "tokio")]
            task_runtime: cx.task_runtime.clone(),
            #[cfg(feature = "tokio")]
            named_tasks: cx.named_tasks.clone(),
        }
    }

    /// Executes the given closure in a spawned thread.
    pub fn spawn<F>(&self, target: F)
    where
        F: 'static + Send + FnOnce(&mut ContextProxy),
    {
        let mut cxp = ContextProxy {
            current: self.current,
            event_proxy: self.event_proxy.as_ref().map(|p| p.make_clone()),
        };

        std::thread::spawn(move || target(&mut cxp));
    }

    /// Submits a configured `TaskBuilder` for asynchronous execution (requires `tokio` feature).
    #[cfg(feature = "tokio")]
    pub fn spawn_task<T, E>(
        &self,
        task: crate::context::TaskBuilder<T, E>,
    ) -> crate::context::TaskHandle
    where
        T: Send + 'static,
        E: Send + 'static,
    {
        task.add_to_resource_context(self)
    }

    /// Loads the provided image into the resource manager.
    pub fn load_image(
        &mut self,
        path: String,
        image: skia_safe::Image,
        policy: ImageRetentionPolicy,
    ) {
        let id = if let Some(image_id) = self.resource_manager.image_ids.get(&path) {
            *image_id
        } else {
            let id = self.resource_manager.image_id_manager.create();
            self.resource_manager.image_ids.insert(path.clone(), id);
            id
        };

        match self.resource_manager.images.entry(id) {
            Entry::Occupied(mut occ) => {
                occ.get_mut().image = ImageOrSvg::Image(image);
                occ.get_mut().dirty = true;
                occ.get_mut().retention_policy = policy;
            }
            Entry::Vacant(vac) => {
                vac.insert(StoredImage {
                    image: ImageOrSvg::Image(image),
                    retention_policy: policy,
                    used: true,
                    dirty: false,
                    observers: HashSet::new(),
                });
            }
        }
        // Relayout only the entities that display this image (its observers) plus the current
        // entity, rather than forcing a full tree relayout.
        let observers: Vec<Entity> = self
            .resource_manager
            .images
            .get(&id)
            .map(|img| img.observers.iter().copied().collect())
            .unwrap_or_default();
        for observer in observers {
            self.style.needs_relayout(observer);
        }
        self.style.needs_relayout(self.current);
    }
}
