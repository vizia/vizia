use crate::prelude::*;
use std::any::TypeId;

pub(crate) struct DragModel {
    pub(crate) on_drag_start: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    pub(crate) on_drag_enter: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    pub(crate) on_drag_leave: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    pub(crate) on_drag_move: Option<Box<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>,
    pub(crate) on_drop: Option<Box<dyn Fn(&mut EventContext, DropData) + Send + Sync>>,
    pub(crate) dragging: Signal<bool>,
}

impl DragModel {
    pub(crate) fn new() -> Self {
        Self {
            on_drag_start: None,
            on_drag_enter: None,
            on_drag_leave: None,
            on_drag_move: None,
            on_drop: None,
            dragging: Signal::new(false),
        }
    }
}

pub(crate) enum DragEvent {
    OnDragStart(Box<dyn Fn(&mut EventContext) + Send + Sync>),
    OnDragEnter(Box<dyn Fn(&mut EventContext) + Send + Sync>),
    OnDragLeave(Box<dyn Fn(&mut EventContext) + Send + Sync>),
    OnDragMove(Box<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>),
    OnDrop(Box<dyn Fn(&mut EventContext, DropData) + Send + Sync>),
}

fn reached_drag_distance_threshold(cx: &EventContext) -> bool {
    let (press_x, press_y) = cx.mouse.left.pos_down;
    let delta_x = cx.mouse.cursor_x - press_x;
    let delta_y = cx.mouse.cursor_y - press_y;
    let drag_distance = cx.environment().drag_distance.get() as f32;

    delta_x * delta_x + delta_y * delta_y >= drag_distance * drag_distance
}

impl Model for DragModel {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|drag_event, _| match drag_event {
            DragEvent::OnDragStart(on_drag_start) => {
                self.on_drag_start = Some(on_drag_start);
            }

            DragEvent::OnDragEnter(on_drag_enter) => {
                self.on_drag_enter = Some(on_drag_enter);
            }

            DragEvent::OnDragLeave(on_drag_leave) => {
                self.on_drag_leave = Some(on_drag_leave);
            }

            DragEvent::OnDragMove(on_drag_move) => {
                self.on_drag_move = Some(on_drag_move);
            }

            DragEvent::OnDrop(on_drop) => {
                self.on_drop = Some(on_drop);
            }
        });

        event.map(|window_event, meta| match window_event {
            WindowEvent::DragEnter => {
                if meta.target == cx.current() {
                    if let Some(action) = &self.on_drag_enter {
                        (action)(cx);
                    }
                }
            }

            WindowEvent::DragLeave => {
                if meta.target == cx.current() {
                    if let Some(action) = &self.on_drag_leave {
                        (action)(cx);
                    }
                }
            }

            WindowEvent::DragMove(x, y) => {
                if let Some(action) = &self.on_drag_move {
                    (action)(cx, *x, *y);
                }
            }

            WindowEvent::MouseOut => {
                if cx.mouse.left.state == MouseButtonState::Pressed
                    && cx.mouse.left.pressed == cx.current()
                    && cx.is_draggable()
                    && reached_drag_distance_threshold(cx)
                    && !cx.has_drop_data()
                {
                    if let Some(action) = &self.on_drag_start {
                        (action)(cx);
                    }

                    if cx.has_drop_data() {
                        cx.capture();

                        self.dragging.set(true);
                    }
                }
            }

            WindowEvent::MouseMove(_, _) => {
                if cx.mouse.left.state == MouseButtonState::Pressed
                    && cx.mouse.left.pressed == cx.current()
                    && cx.is_draggable()
                    && reached_drag_distance_threshold(cx)
                    && !cx.has_drop_data()
                {
                    if let Some(action) = &self.on_drag_start {
                        (action)(cx);
                    }

                    if cx.has_drop_data() {
                        cx.capture();

                        self.dragging.set(true);
                    }
                }

                if cx.mouse.left.state == MouseButtonState::Released {
                    if let Some(action) = &self.on_drop {
                        if let Some(drop_data) = cx.drop_data.take() {
                            (action)(cx, drop_data);
                        }
                    }
                    self.dragging.set(false);
                }
            }

            WindowEvent::MouseUp(_) => {
                self.dragging.set(false);
                if let Some(action) = &self.on_drop {
                    if let Some(drop_data) = cx.drop_data.take() {
                        (action)(cx, drop_data);
                    }
                }
            }

            _ => {}
        });
    }
}

fn build_drag_model(cx: &mut Context, entity: Entity) {
    if cx.models.get(&entity).and_then(|models| models.get(&TypeId::of::<DragModel>())).is_none() {
        cx.with_current(entity, |cx| {
            DragModel::new().build(cx);
        });
    }
}

/// Modifiers which add drag-and-drop callbacks to a view.
pub trait DragModifiers<V> {
    /// Adds a callback which is performed when the view begins to be dragged.
    ///
    /// The callback should call [`set_drop_data`](EventContext::set_drop_data) to supply the data
    /// that will be delivered to the drop target.
    fn on_drag<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync;

    /// Adds a callback which is performed when the cursor enters this view while carrying drag data.
    fn on_drag_enter<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync;

    /// Adds a callback which is performed when the cursor leaves this view while carrying drag data.
    fn on_drag_leave<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync;

    /// Adds a callback which is performed when the cursor moves over this view while carrying drag data.
    fn on_drag_move<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, f32, f32) + Send + Sync;

    /// Adds a callback which is performed when data is dropped on the view during a drag-and-drop operation.
    fn on_drop<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, DropData) + Send + Sync;

    /// Adds a view that is rendered under the mouse cursor while this view is being dragged.
    fn on_drag_view<C: Fn(&mut Context) -> Handle<'_, T> + 'static, T: View>(
        self,
        content: C,
    ) -> Self;
}

impl<V: View> DragModifiers<V> for Handle<'_, V> {
    fn on_drag<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        build_drag_model(self.cx, self.entity);

        if let Some(abilities) = self.cx.style.abilities.get_mut(self.entity) {
            abilities.set(Abilities::DRAGGABLE, true);
        }

        self.cx.emit_custom(
            Event::new(DragEvent::OnDragStart(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_drag_enter<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        build_drag_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(DragEvent::OnDragEnter(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_drag_leave<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        build_drag_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(DragEvent::OnDragLeave(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_drag_move<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, f32, f32) + Send + Sync,
    {
        build_drag_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(DragEvent::OnDragMove(Box::new(action)))
                .target(self.entity)
                .origin(self.entity),
        );

        self
    }

    fn on_drop<F>(self, action: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, DropData) + Send + Sync,
    {
        build_drag_model(self.cx, self.entity);

        self.cx.emit_custom(
            Event::new(DragEvent::OnDrop(Box::new(action))).target(self.entity).origin(self.entity),
        );

        self
    }

    fn on_drag_view<C: Fn(&mut Context) -> Handle<'_, T> + 'static, T: View>(
        self,
        content: C,
    ) -> Self {
        build_drag_model(self.cx, self.entity);

        if let Some(abilities) = self.cx.style.abilities.get_mut(self.entity) {
            abilities.set(Abilities::DRAGGABLE, true);
        }

        let source_entity = self.entity;

        self.cx.with_current(source_entity, move |cx| {
            let is_dragging = cx.data::<DragModel>().dragging;
            Binding::new(cx, is_dragging, move |cx| {
                if is_dragging.get() {
                    let window_entity = cx.parent_window();
                    let drag_entity = cx.with_current(window_entity, |cx| {
                        (content)(cx)
                            .position_type(PositionType::Absolute)
                            .left(Pixels(0.0))
                            .top(Pixels(0.0))
                            .display(Display::None)
                            .pointer_events(PointerEvents::None)
                            .z_index(10_000)
                            .entity()
                    });

                    let mut ex = EventContext::new(cx);
                    ex.set_active_drag_view(Some(drag_entity));
                }
            });
        });

        self
    }
}
