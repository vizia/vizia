use crate::context::AccessNode;
use crate::prelude::*;
use crate::systems::get_access_node;
use crate::{accessibility::IntoNode, context::AccessContext};
use std::any::Any;

use crate::events::ViewHandler;
use crate::state::ModelDataStore;
use accesskit::{NodeBuilder, TreeUpdate};
use femtovg::renderer::OpenGl;

/// The canvas we will be drawing to.
///
/// This type is part of the prelude.
pub type Canvas = femtovg::Canvas<OpenGl>;

/// A view is any object which can be displayed on the screen.
///
/// This trait is part of the prelude.
pub trait View: 'static + Sized {
    fn build<F>(self, cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        let id = cx.entity_manager.create();
        let current = cx.current();
        cx.tree.add(id, current).expect("Failed to add to tree");
        cx.cache.add(id).expect("Failed to add to cache");
        cx.style.add(id);
        cx.views.insert(id, Box::new(self));
        let parent_id = cx.tree.get_layout_parent(id).unwrap();
        let parent_node_id = parent_id.accesskit_id();
        let node_id = id.accesskit_id();
        let children =
            parent_id.child_iter(&cx.tree).map(|entity| entity.accesskit_id()).collect::<Vec<_>>();

        let mut access_context = AccessContext {
            current: id,
            tree: &cx.tree,
            cache: &cx.cache,
            style: &cx.style,
            text_context: &mut cx.text_context,
        };

        if let Some(mut parent_node) =
            get_access_node(&mut access_context, &mut cx.views, parent_id)
        {
            parent_node.node_builder.set_children(children);
            let parent_node = parent_node.node_builder.build(&mut cx.style.accesskit_node_classes);
            let node = NodeBuilder::default().build(&mut cx.style.accesskit_node_classes);
            cx.tree_updates.push(TreeUpdate {
                nodes: vec![(parent_node_id, parent_node), (node_id, node)],
                tree: None,
                focus: None,
            });
        }

        cx.data.insert(id, ModelDataStore::default()).expect("Failed to insert model data store");

        let handle = Handle { entity: id, p: Default::default(), cx };

        handle.cx.with_current(handle.entity, content);

        handle
    }

    /// The name of the view. This is used in css: to style every single one of a given view, you
    /// specify the element name.
    fn element(&self) -> Option<&'static str> {
        None
    }

    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {}

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        //Skip widgets with no width or no height
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let mut path = cx.build_path();

        cx.draw_shadows(canvas, &mut path);

        cx.draw_background(canvas, &mut path);

        cx.draw_gradients(canvas, &mut path);

        cx.draw_border(canvas, &mut path);

        cx.draw_inset_box_shadows(canvas, &mut path);

        cx.draw_outline(canvas);

        cx.draw_text_and_selection(canvas);
    }

    #[allow(unused_variables)]
    fn accessibility(&self, cx: &mut AccessContext, node: &mut AccessNode) {}
}

impl<T: View> ViewHandler for T
where
    T: std::marker::Sized + View + 'static,
{
    fn element(&self) -> Option<&'static str> {
        <T as View>::element(self)
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        <T as View>::event(self, cx, event);
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        <T as View>::draw(self, cx, canvas);
    }

    fn accessibility(&self, cx: &mut AccessContext, node: &mut AccessNode) {
        <T as View>::accessibility(self, cx, node);
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
