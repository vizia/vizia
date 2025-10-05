//! Views are used to visually present model data and to act as controls which, when interacted with, send events to mutate model data.
//!
//! # Example
//! The `Label` view is used to display a text string:
//!
//! ```no_run
//! # use vizia_core::prelude::*;
//! # use vizia_winit::application::Application;
//! Application::new(|cx|{
//!     Label::new(cx, "Hello World");
//! })
//! .run();
//! ```

use crate::accessibility::IntoNode;
use crate::prelude::*;
use crate::systems::get_access_node;
use std::any::{Any, TypeId};
mod handle;
pub use handle::Handle;
use hashbrown::HashMap;

use crate::events::ViewHandler;
use accesskit::{Node, TreeUpdate};

/// A view is any object which can be displayed on the screen.
///
/// # Creating a Custom View
///
/// To create a custom view, first define a struct with any view-specific state.
/// ```
/// # use vizia_core::prelude::*;
/// pub struct CustomView {
///     count: i32,
/// }
/// ```
///
/// Next, implement the constructor for the custom view. Typically, the constructor will take `&mut Context` as the first argument
/// and return a [`Handle`] to the view.
/// ```
/// # use vizia_core::prelude::*;
/// pub struct CustomView {
///     count: i32,
/// }
///
/// impl CustomView {
///     pub fn new(cx: &mut Context, count: i32) -> Handle<Self> {
///         Self {
///             count,
///         }.build(cx, |cx|{
///             // If we want the view to contain other views we can build those here.
///         })
///     }
/// }
///
/// # impl View for CustomView {}
/// ```
///
/// The `build` method above is provided by the `View` trait, which we must implement for any custom view.
/// ```
/// # use vizia_core::prelude::*;
/// pub struct CustomView {
///     count: i32,
/// }
///
/// impl CustomView {
///     pub fn new(cx: &mut Context, count: i32) -> Handle<Self> {
///         Self {
///             count,
///         }.build(cx, |cx|{
///             // If we want the view to contain other views we can build those here.
///         })
///     }
/// }
///
/// impl View for CustomView {
///
/// }
/// ```
///
/// The `View` trait contains methods, which can be optionally overridden, for assigning an element name, handling events, and performing custom drawing.
pub trait View: 'static + Sized {
    /// Builds the view into the tree and returns a handle which can be used to apply style and layout modifiers to the view.
    ///
    /// Typically this method is called within the constructor of a view, for example:
    /// ```
    /// # use vizia_core::prelude::*;
    /// pub struct CustomView{}
    ///
    /// impl CustomView {
    ///     pub fn new(cx: &mut Context) -> Handle<Self> {
    ///         Self{}.build(cx, |_|{})
    ///     }
    /// }
    /// # impl View for CustomView {}
    /// ```
    /// The `content` closure allows for a view to be built from other views. For example, a custom view could encapsulate a
    /// pair of labels:
    /// ```
    /// # use vizia_core::prelude::*;
    /// pub struct CustomView{}
    ///
    /// impl CustomView {
    ///     pub fn new(cx: &mut Context) -> Handle<Self> {
    ///         Self{}.build(cx, |cx|{
    ///             Label::new(cx, "Hello");
    ///             Label::new(cx, "World");
    ///         })
    ///     }
    /// }
    /// # impl View for CustomView {}
    /// ```
    fn build<F>(self, cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        let id = cx.entity_manager.create();
        let current = cx.current();
        cx.tree.add(id, current).expect("Failed to add to tree");
        cx.cache.add(id);
        cx.style.add(id);
        cx.needs_redraw(id);

        if let Some(element) = self.element() {
            cx.style.element.insert(id, fxhash::hash32(element));
        }

        // cx.views.insert(id, Box::new(self));

        let parent_id = cx.tree.get_layout_parent(id).unwrap();
        let parent_node_id = parent_id.accesskit_id();
        let node_id = id.accesskit_id();

        let mut access_context = AccessContext {
            current: id,
            tree: &cx.tree,
            cache: &cx.cache,
            style: &cx.style,
            text_context: &mut cx.text_context,
        };

        if let Some(parent_node) = get_access_node(&mut access_context, &mut cx.views, parent_id) {
            let parent_node = parent_node.node_builder;
            let node = Node::default();

            cx.tree_updates.push(Some(TreeUpdate {
                nodes: vec![(parent_node_id, parent_node), (node_id, node)],
                tree: None,
                focus: cx.focused.accesskit_id(),
            }));
        }

        cx.models.insert(id, HashMap::default());
        cx.stores.insert(id, HashMap::default());

        let mut handle = Handle { current: id, entity: id, p: Default::default(), cx };

        handle.cx.with_current(handle.entity, content);

        let s = handle.cx.with_current(handle.entity, |cx| self.on_build(cx));

        handle.cx.views.insert(id, Box::new(s));

        handle
    }

    fn on_build(self, cx: &mut Context) -> Self {
        self
    }

    /// Specifies a name for the view type which can be used as an element selector in css.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// pub struct CustomView{}
    ///
    /// impl CustomView {
    ///     pub fn new(cx: &mut Context) -> Handle<Self> {
    ///         Self{}.build(cx, |_|{})
    ///     }
    /// }
    ///
    /// impl View for CustomView {
    ///     fn element(&self) -> Option<&'static str> {
    ///         Some("custom_view")
    ///     }
    /// }
    /// ```
    /// Then in css:
    /// ```css
    /// custom_view {
    ///     background-color: red;
    /// }
    /// ```
    fn element(&self) -> Option<&'static str> {
        None
    }

    /// Handles any events received by the view.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// pub struct CustomView{}
    ///
    /// impl CustomView {
    ///     pub fn new(cx: &mut Context) -> Handle<Self> {
    ///         Self{}.build(cx, |_|{})
    ///     }
    /// }
    ///
    /// impl View for CustomView {
    ///     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    ///         event.map(|window_event, meta| match window_event{
    ///             WindowEvent::MouseDown(_) => {
    ///                 if meta.target == cx.current() {
    ///                     // Emit a `WindowClose` event when this view is clicked on.
    ///                     cx.emit(WindowEvent::WindowClose);
    ///                 }
    ///             }
    ///
    ///             _=> {}
    ///         });
    ///     }
    /// }
    /// ```
    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {}

    /// Provides custom drawing for the view.
    ///
    /// Usually the look of a view is determined by the style and layout properties of the view. However, the `draw` method of
    /// the `View` trait can be used to provide completely custom drawing for the view. The properties of the view can be accessed
    /// through the provided [`DrawContext`] and the provided [`Canvas`] can be used to draw custom paths.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_core::vg;
    /// pub struct CustomView{}
    ///
    /// impl CustomView {
    ///     pub fn new(cx: &mut Context) -> Handle<Self> {
    ///         Self{}.build(cx, |_|{})
    ///     }
    /// }
    ///
    /// impl View for CustomView {
    ///     fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
    ///         // Get the bounding box of the current view.
    ///         let bounds = cx.bounds();
    ///
    ///         // Create a new `Path` from the `vg` module.
    ///         let mut path = vg::Path::new();
    ///         // Add a rectangle to the path with the dimensions of the view bounds.
    ///         path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
    ///         // Fill the path onto the canvas with a red color.
    ///         canvas.fill_path(&mut path, &vg::Paint::color(Color::red().into()));
    ///     }
    /// }
    /// ```
    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let bounds = cx.bounds();

        //Skip widgets with no width or no height
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        cx.draw_background(canvas);
        cx.draw_shadows(canvas);

        cx.draw_border(canvas);

        cx.draw_outline(canvas);

        cx.draw_text(canvas);
    }

    #[allow(unused_variables)]
    /// Provides a way to configure the accessibility features of a view.
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

    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
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

    fn id(&self) -> std::any::TypeId {
        TypeId::of::<T>()
    }
}
