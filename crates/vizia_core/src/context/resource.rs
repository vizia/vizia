use hashbrown::{hash_map::Entry, HashSet};

use skia_safe::Surface;
use vizia_storage::Tree;

use crate::{
    entity::Entity,
    resource::{ImageRetentionPolicy, ResourceManager, StoredImage},
    style::Style,
};

use super::{Context, ContextProxy, EventProxy};
use hashbrown::HashMap;

/// A context used when loading images.
pub struct ResourceContext<'a> {
    pub(crate) current: Entity,
    pub(crate) event_proxy: &'a Option<Box<dyn EventProxy>>,
    pub(crate) resource_manager: &'a mut ResourceManager,
    // pub(crate) canvases: &'a mut HashMap<Entity, Surface>,
    pub(crate) style: &'a mut Style,
    pub(crate) tree: &'a Tree<Entity>,
}

impl<'a> ResourceContext<'a> {
    pub fn new(cx: &'a mut Context) -> Self {
        Self {
            current: cx.current,
            event_proxy: &cx.event_proxy,
            resource_manager: &mut cx.resource_manager,
            // canvases: &mut cx.canvases,
            style: &mut cx.style,
            tree: &cx.tree,
        }
    }

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

    pub fn load_image(
        &mut self,
        path: String,
        image: skia_safe::Image,
        policy: ImageRetentionPolicy,
    ) {
        let id = if let Some(image_id) = self.resource_manager.image_ids.get(&path) {
            *image_id
        } else {
            self.resource_manager.image_id_manager.create()
        };

        match self.resource_manager.images.entry(id) {
            Entry::Occupied(mut occ) => {
                occ.get_mut().image = image;
                occ.get_mut().dirty = true;
                occ.get_mut().retention_policy = policy;
            }
            Entry::Vacant(vac) => {
                vac.insert(StoredImage {
                    image,
                    retention_policy: policy,
                    used: true,
                    dirty: false,
                    observers: HashSet::new(),
                });
            }
        }
        self.style.needs_relayout();
    }
}
