use crate::layout::cache::GeoChanged;
use crate::prelude::*;
use crate::style::Abilities;
use std::any::TypeId;

#[derive(Lens)]
pub struct TooltipModel {
    pub tooltip_visible: bool,
}

pub enum TooltipEvent {
    ShowTooltip,
    HideTooltip,
}

impl Model for TooltipModel {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|tooltip_event, _| match tooltip_event {
            TooltipEvent::ShowTooltip => {
                self.tooltip_visible = true;
            }

            TooltipEvent::HideTooltip => {
                self.tooltip_visible = false;
            }
        })
    }
}

pub(crate) struct ActionsModel {
    pub(crate) on_press: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    pub(crate) on_press_down: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    pub(crate) on_double_click: Option<Box<dyn Fn(&mut EventContext, MouseButton) + Send + Sync>>,
    pub(crate) on_hover: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    pub(crate) on_hover_out: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    pub(crate) on_over: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    pub(crate) on_over_out: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    pub(crate) on_mouse_move: Option<Box<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>,
    pub(crate) on_mouse_down: Option<Box<dyn Fn(&mut EventContext, MouseButton) + Send + Sync>>,
    pub(crate) on_mouse_up: Option<Box<dyn Fn(&mut EventContext, MouseButton) + Send + Sync>>,
    pub(crate) on_focus_in: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    pub(crate) on_focus_out: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    pub(crate) on_geo_changed: Option<Box<dyn Fn(&mut EventContext, GeoChanged) + Send + Sync>>,
    pub(crate) on_drag_start: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    pub(crate) on_drop: Option<Box<dyn Fn(&mut EventContext, DropData) + Send + Sync>>,
}

impl ActionsModel {
    pub(crate) fn new() -> Self {
        Self {
            on_press: None,
            on_press_down: None,
            on_double_click: None,
            on_hover: None,
            on_hover_out: None,
            on_over: None,
            on_over_out: None,
            on_mouse_move: None,
            on_mouse_down: None,
            on_mouse_up: None,
            on_focus_in: None,
            on_focus_out: None,
            on_geo_changed: None,
            on_drag_start: None,
            on_drop: None,
        }
    }
}

impl Model for ActionsModel {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        if let Some(actions_event) = event.take() {
            match actions_event {
                ActionsEvent::OnPress(on_press) => {
                    self.on_press = Some(on_press);
                }

                ActionsEvent::OnPressDown(on_press_down) => {
                    self.on_press_down = Some(on_press_down);
                }

                ActionsEvent::OnDoubleClick(on_double_click) => {
                    self.on_double_click = Some(on_double_click);
                }

                ActionsEvent::OnHover(on_hover) => {
                    self.on_hover = Some(on_hover);
                }

                ActionsEvent::OnHoverOut(on_hover_out) => {
                    self.on_hover_out = Some(on_hover_out);
                }

                ActionsEvent::OnOver(on_over) => {
                    self.on_over = Some(on_over);
                }

                ActionsEvent::OnOverOut(on_over_out) => {
                    self.on_over_out = Some(on_over_out);
                }

                ActionsEvent::OnMouseMove(on_move) => {
                    self.on_mouse_move = Some(on_move);
                }

                ActionsEvent::OnMouseDown(on_mouse_down) => {
                    self.on_mouse_down = Some(on_mouse_down);
                }

                ActionsEvent::OnMouseUp(on_mouse_up) => {
                    self.on_mouse_up = Some(on_mouse_up);
                }

                ActionsEvent::OnFocusIn(on_focus_in) => {
                    self.on_focus_in = Some(on_focus_in);
                }

                ActionsEvent::OnFocusOut(on_focus_out) => {
                    self.on_focus_out = Some(on_focus_out);
                }

                ActionsEvent::OnGeoChanged(on_geo_changed) => {
                    self.on_geo_changed = Some(on_geo_changed);
                    cx.cache.set_bounds(cx.current, BoundingBox::default());
                    cx.needs_relayout();
                }

                ActionsEvent::OnDragStart(on_drag_start) => {
                    self.on_drag_start = Some(on_drag_start);
                }

                ActionsEvent::OnDrop(on_drop) => {
                    self.on_drop = Some(on_drop);
                }
            }
        }

        event.map(|window_event, meta| match window_event {
            WindowEvent::Press { mouse } => {
                let over = if *mouse { cx.hovered() } else { cx.focused() };
                if cx.current() != over && !over.is_descendant_of(cx.tree, cx.current()) {
                    return;
                }

                if !cx.is_disabled() && cx.current == meta.target {
                    if let Some(action) = &self.on_press {
                        (action)(cx);
                    }
                }
            }

            WindowEvent::PressDown { mouse } => {
                let over = if *mouse { cx.hovered() } else { cx.focused() };
                if cx.current() != over && !over.is_descendant_of(cx.tree, cx.current()) {
                    return;
                }
                if !cx.is_disabled() && cx.current == meta.target {
                    if let Some(action) = &self.on_press_down {
                        (action)(cx);
                    }
                }
                if cx.is_draggable() {
                    if let Some(action) = &self.on_drag_start {
                        (action)(cx);
                    }
                }
            }

            WindowEvent::MouseDoubleClick(button) => {
                if meta.target == cx.current && !cx.is_disabled() {
                    if let Some(action) = &self.on_double_click {
                        (action)(cx, *button);
                    }
                }
            }

            WindowEvent::MouseEnter => {
                if meta.target == cx.current() {
                    if let Some(action) = &self.on_hover {
                        (action)(cx);
                    }
                }
            }

            WindowEvent::MouseLeave => {
                if meta.target == cx.current() {
                    if let Some(action) = &self.on_hover_out {
                        (action)(cx);
                    }
                }
            }

            WindowEvent::MouseOver => {
                if let Some(action) = &self.on_over {
                    (action)(cx);
                }
            }

            WindowEvent::MouseOut => {
                // if meta.target == cx.current() {
                if let Some(action) = &self.on_over_out {
                    (action)(cx);
                }
                // }
            }

            WindowEvent::MouseMove(x, y) => {
                if let Some(action) = &self.on_mouse_move {
                    (action)(cx, *x, *y);
                }
                if cx.mouse.left.state == MouseButtonState::Released {
                    if let Some(drop_data) = cx.drop_data.take() {
                        if let Some(action) = &self.on_drop {
                            (action)(cx, drop_data);
                        }
                    }
                }
            }

            WindowEvent::MouseDown(mouse_button) => {
                if let Some(action) = &self.on_mouse_down {
                    (action)(cx, *mouse_button);
                }
            }

            WindowEvent::MouseUp(mouse_button) => {
                if let Some(action) = &self.on_mouse_up {
                    (action)(cx, *mouse_button);
                }
                if let Some(drop_data) = cx.drop_data.take() {
                    if let Some(action) = &self.on_drop {
                        (action)(cx, drop_data);
                    }
                }
            }

            WindowEvent::FocusIn => {
                if let Some(action) = &self.on_focus_in {
                    (action)(cx);
                }
            }

            WindowEvent::FocusOut => {
                if let Some(action) = &self.on_focus_out {
                    (action)(cx);
                }
            }

            WindowEvent::GeometryChanged(geo) => {
                if meta.target == cx.current() {
                    if let Some(action) = &self.on_geo_changed {
                        (action)(cx, *geo);
                    }
                }
            }

            _ => {}
        });
    }
}

pub(crate) enum ActionsEvent {
    OnPress(Box<dyn Fn(&mut EventContext) + Send + Sync>),
    OnPressDown(Box<dyn Fn(&mut EventContext) + Send + Sync>),
    OnDoubleClick(Box<dyn Fn(&mut EventContext, MouseButton) + Send + Sync>),
    OnHover(Box<dyn Fn(&mut EventContext) + Send + Sync>),
    OnHoverOut(Box<dyn Fn(&mut EventContext) + Send + Sync>),
    OnOver(Box<dyn Fn(&mut EventContext) + Send + Sync>),
    OnOverOut(Box<dyn Fn(&mut EventContext) + Send + Sync>),
    OnMouseMove(Box<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>),
    OnMouseDown(Box<dyn Fn(&mut EventContext, MouseButton) + Send + Sync>),
    OnMouseUp(Box<dyn Fn(&mut EventContext, MouseButton) + Send + Sync>),
    OnFocusIn(Box<dyn Fn(&mut EventContext) + Send + Sync>),
    OnFocusOut(Box<dyn Fn(&mut EventContext) + Send + Sync>),
    OnGeoChanged(Box<dyn Fn(&mut EventContext, GeoChanged) + Send + Sync>),
    OnDragStart(Box<dyn Fn(&mut EventContext) + Send + Sync>),
    OnDrop(Box<dyn Fn(&mut EventContext, DropData) + Send + Sync>),
}

/// Modifiers which add an action callback to a view.
pub trait ActionModifiers<V> {
    /// Adds a callback which is performed when the the view receives the [`Press`](crate::prelude::WindowEvent::Press) event.
    /// By default a view receives the [`Press`](crate::prelude::WindowEvent::Press) event when the left mouse button is pressed and then released on the view,
    /// or when the space or enter keys are pressed and then released while the view is focused.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// Element::new(cx).on_press(|_| println!("View was pressed!"));
    /// ```
    fn on_press<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync;

    /// Adds a callback which is performed when the the view receives the [`PressDown`](crate::prelude::WindowEvent::PressDown) event.
    // By default a view receives the [`PressDown`](crate::prelude::WindowEvent::PressDown) event when the left mouse button is pressed on the view,
    /// or when the space or enter keys are pressed while the view is focused.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// Element::new(cx).on_press_down(|_| println!("View was pressed down!"));
    /// ```
    fn on_press_down<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync;

    /// Adds a callback which is performed when the the view receives the [`MouseDoubleClick`](crate::prelude::WindowEvent::MouseDoubleClick) event.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// Element::new(cx).on_double_click(|_, _button| println!("View was double clicked on!"));
    /// ```
    fn on_double_click<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, MouseButton) + Send + Sync;

    /// Adds a callback which is performed when the mouse pointer moves over a view.
    /// This callback is not triggered when the mouse pointer moves over an overlapping child of the view.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// Element::new(cx).on_hover(|_| println!("Mouse cursor entered the view!"));
    /// ```
    fn on_hover<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync;

    /// Adds a callback which is performed when the mouse pointer moves away from a view.
    /// This callback is not triggered when the mouse pointer moves away from an overlapping child of the view.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// Element::new(cx).on_hover_out(|_| println!("Mouse cursor left the view!"));
    /// ```
    fn on_hover_out<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync;

    /// Adds a callback which is performed when the mouse pointer moves over the bounds of a view,
    /// including any overlapping children.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// Element::new(cx).on_over(|_| println!("Mouse cursor entered the view bounds!"));
    /// ```
    fn on_over<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync;

    /// Adds a callback which is performed when the mouse pointer moves away from the bounds of a view,
    /// including any overlapping children.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// Element::new(cx).on_over_out(|_| println!("Mouse cursor left the view bounds!"));
    /// ```
    fn on_over_out<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync;

    /// Adds a callback which is performed when the mouse pointer moves within the bounds of a view.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// Element::new(cx).on_mouse_move(|_, x, y| println!("Cursor moving: {} {}", x, y));
    /// ```
    fn on_mouse_move<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, f32, f32) + Send + Sync;

    /// Adds a callback which is performed when a mouse button is pressed on the view.
    /// Unlike the `on_press` callback, this callback is triggered for all mouse buttons and not for any keyboard keys.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// Element::new(cx).on_mouse_down(|_, button| println!("Mouse button, {:?}, was pressed!", button));
    /// ```
    fn on_mouse_down<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, MouseButton) + Send + Sync;

    /// Adds a callback which is performed when a mouse button is released on the view.
    /// Unlike the `on_release` callback, this callback is triggered for all mouse buttons and not for any keyboard keys.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// Element::new(cx).on_mouse_up(|_, button| println!("Mouse button, {:?}, was released!", button));
    /// ```
    fn on_mouse_up<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, MouseButton) + Send + Sync;

    /// Adds a callback which is performed when the view gains keyboard focus.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// Element::new(cx).on_focus_in(|_| println!("View gained keyboard focus!"));
    /// ```
    fn on_focus_in<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync;

    /// Adds a callback which is performed when the view loses keyboard focus.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// Element::new(cx).on_focus_out(|_| println!("View lost keyboard focus!"));
    /// ```
    fn on_focus_out<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync;

    /// Adds a callback which is performed when the the view changes size or position after layout.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// Element::new(cx).on_geo_changed(|_, _| println!("View geometry changed!"));
    /// ```
    fn on_geo_changed<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, GeoChanged) + Send + Sync;

    fn tooltip<C: FnOnce(&mut Context)>(self, content: C) -> Self;

    fn on_drag<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync;

    fn on_drop<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, DropData) + Send + Sync;
}

// If the entity doesn't have an `ActionsModel` then add one to the entity
fn build_action_model(cx: &mut Context, entity: Entity) {
    if cx
        .data
        .get(&entity)
        .and_then(|model_data_store| model_data_store.models.get(&TypeId::of::<ActionsModel>()))
        .is_none()
    {
        cx.with_current(entity, |cx| {
            ActionsModel::new().build(cx);
        });
    }
}

fn build_tooltip_model(cx: &mut Context, entity: Entity) {
    if cx
        .data
        .get(&entity)
        .and_then(|model_data_store| model_data_store.models.get(&TypeId::of::<TooltipModel>()))
        .is_none()
    {
        cx.with_current(entity, |cx| {
            TooltipModel { tooltip_visible: false }.build(cx);
        });
    }
}

impl<'a, V: View> ActionModifiers<V> for Handle<'a, V> {
    fn tooltip<C: FnOnce(&mut Context)>(self, content: C) -> Self {
        let entity = self.entity();

        build_tooltip_model(self.cx, entity);

        let s = self
            .on_hover(|cx| cx.emit(TooltipEvent::ShowTooltip))
            .on_hover_out(|cx| cx.emit(TooltipEvent::HideTooltip));

        s.cx.with_current(entity, |cx| {
            Tooltip::new(cx, content).toggle_class("vis", TooltipModel::tooltip_visible);
        });

        s
    }

    fn on_press<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnPress(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_press_down<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnPressDown(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_double_click<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, MouseButton) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnDoubleClick(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_hover<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnHover(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_hover_out<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnHoverOut(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_over<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnOver(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_over_out<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnOverOut(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_mouse_move<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, f32, f32) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnMouseMove(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_mouse_down<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, MouseButton) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnMouseDown(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_mouse_up<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, MouseButton) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnMouseUp(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_focus_in<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnFocusIn(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_focus_out<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnFocusOut(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_geo_changed<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, GeoChanged) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnGeoChanged(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_drag<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        if let Some(abilities) = self.cx.style.abilities.get_mut(self.entity) {
            abilities.set(Abilities::DRAGGABLE, true);
        }

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnDragStart(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_drop<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, DropData) + Send + Sync,
    {
        build_action_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(ActionsEvent::OnDrop(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }
}
