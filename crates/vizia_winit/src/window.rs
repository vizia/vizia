use std::sync::Arc;

use hashbrown::HashMap;

use crate::convert::cursor_icon_to_cursor_icon;
use crate::window_modifiers::WindowModifiers;

use vizia_core::{context::TreeProps, prelude::*};
use vizia_window::AnchorTarget;

use winit::{
    dpi::*,
    window::{CursorGrabMode, CursorIcon, CustomCursor, WindowLevel},
};

#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;

type WindowCallback = Option<Box<dyn Fn(&mut EventContext)>>;

pub struct Window {
    pub window: Option<Arc<winit::window::Window>>,
    pub on_close: WindowCallback,
    pub on_create: WindowCallback,
    pub should_close: bool,
    pub(crate) custom_cursors: Arc<HashMap<CursorIcon, CustomCursor>>,
}

impl Window {
    fn window(&self) -> &winit::window::Window {
        self.window.as_ref().unwrap()
    }

    pub fn new(cx: &mut Context, content: impl 'static + Fn(&mut Context)) -> Handle<Self> {
        Self {
            window: None,
            on_close: None,
            on_create: None,
            should_close: false,
            custom_cursors: Default::default(),
        }
        .build(cx, |cx| {
            cx.windows.insert(
                cx.current(),
                WindowState { content: Some(Arc::new(content)), ..Default::default() },
            );
            cx.tree.set_window(cx.current(), true);
        })
    }

    pub fn popup(
        cx: &mut Context,
        is_modal: bool,
        content: impl 'static + Fn(&mut Context),
    ) -> Handle<Self> {
        Self {
            window: None,
            on_close: None,
            on_create: None,
            should_close: false,
            custom_cursors: Default::default(),
        }
        .build(cx, |cx| {
            let parent_window = cx.parent_window();
            if is_modal {
                cx.emit_to(parent_window, WindowEvent::SetEnabled(false));
            }

            cx.windows.insert(
                cx.current(),
                WindowState {
                    owner: Some(parent_window),
                    is_modal: true,
                    content: Some(Arc::new(content)),
                    ..Default::default()
                },
            );
            cx.tree.set_window(cx.current(), true);
        })
        .anchor_target(AnchorTarget::Window)
        .lock_focus_to_within()
    }
}

impl View for Window {
    fn element(&self) -> Option<&'static str> {
        Some("window")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::Destroyed => {
                let parent_window = cx.parent_window();
                cx.emit_to(parent_window, WindowEvent::SetEnabled(true));
            }

            WindowEvent::GrabCursor(flag) => {
                let grab_mode = if *flag { CursorGrabMode::Locked } else { CursorGrabMode::None };
                self.window().set_cursor_grab(grab_mode).expect("Failed to set cursor grab");
            }

            WindowEvent::SetCursorPosition(x, y) => {
                self.window()
                    .set_cursor_position(winit::dpi::Position::Physical(PhysicalPosition::new(
                        *x as i32, *y as i32,
                    )))
                    .expect("Failed to set cursor position");
            }

            WindowEvent::SetCursor(cursor) => {
                let Some(icon) = cursor_icon_to_cursor_icon(*cursor) else {
                    self.window().set_cursor_visible(false);
                    return;
                };

                if let Some(custom_icon) = self.custom_cursors.get(&icon) {
                    self.window().set_cursor(custom_icon.clone());
                } else {
                    self.window().set_cursor(icon);
                }

                self.window().set_cursor_visible(true);
            }

            WindowEvent::SetTitle(title) => {
                self.window().set_title(title);
            }

            WindowEvent::SetSize(size) => {
                let _ = self.window().request_inner_size(LogicalSize::new(size.width, size.height));
            }

            WindowEvent::SetMinSize(size) => {
                self.window()
                    .set_min_inner_size(size.map(|size| LogicalSize::new(size.width, size.height)));
            }

            WindowEvent::SetMaxSize(size) => {
                self.window()
                    .set_max_inner_size(size.map(|size| LogicalSize::new(size.width, size.height)));
            }

            WindowEvent::SetPosition(pos) => {
                self.window().set_outer_position(LogicalPosition::new(pos.x, pos.y));
                meta.consume();
            }

            WindowEvent::SetResizable(flag) => {
                self.window().set_resizable(*flag);
            }

            WindowEvent::SetMinimized(flag) => {
                self.window().set_minimized(*flag);
            }

            WindowEvent::SetMaximized(flag) => {
                self.window().set_maximized(*flag);
            }

            WindowEvent::SetVisible(flag) => {
                self.window().set_visible(*flag);

                meta.consume();
            }

            WindowEvent::SetDecorations(flag) => {
                self.window().set_decorations(*flag);
            }

            WindowEvent::ReloadStyles => {
                cx.reload_styles().unwrap();
            }

            WindowEvent::WindowClose => {
                self.should_close = true;

                cx.close_window();

                if let Some(callback) = &self.on_close {
                    callback(cx);
                }

                meta.consume();
            }

            WindowEvent::FocusNext => {
                cx.focus_next();
            }

            WindowEvent::FocusPrev => {
                cx.focus_prev();
            }

            WindowEvent::Redraw => {
                self.window().request_redraw();
            }

            #[allow(unused_variables)]
            WindowEvent::SetEnabled(flag) => {
                #[cfg(target_os = "windows")]
                self.window().set_enable(*flag);

                self.window().focus_window();
            }

            WindowEvent::DragWindow => {
                self.window().drag_window().expect("Failed to init drag window");
                meta.consume();
            }

            WindowEvent::SetAlwaysOnTop(flag) => {
                self.window().set_window_level(if *flag {
                    WindowLevel::AlwaysOnTop
                } else {
                    WindowLevel::Normal
                });
            }

            _ => {}
        })
    }
}

impl WindowModifiers for Handle<'_, Window> {
    fn on_close(self, callback: impl Fn(&mut EventContext) + 'static) -> Self {
        self.modify(|window| window.on_close = Some(Box::new(callback)))
    }

    fn on_create(self, callback: impl Fn(&mut EventContext) + 'static) -> Self {
        self.modify(|window| window.on_create = Some(Box::new(callback)))
    }

    fn title<T: ToString>(mut self, title: impl Res<T>) -> Self {
        let entity = self.entity();
        let title = title.get(&self).to_string();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.title = title;
        }

        self
    }

    fn inner_size<S: Into<WindowSize>>(mut self, size: impl Res<S>) -> Self {
        let entity = self.entity();
        let size = size.get(&self).into();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.inner_size = size;
        }

        self
    }

    fn min_inner_size<S: Into<WindowSize>>(mut self, size: impl Res<Option<S>>) -> Self {
        let entity = self.entity();
        let size = size.get(&self).map(|size| size.into());
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.min_inner_size = size;
        }

        self
    }

    fn max_inner_size<S: Into<WindowSize>>(mut self, size: impl Res<Option<S>>) -> Self {
        let entity = self.entity();
        let size = size.get(&self).map(|size| size.into());
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.max_inner_size = size;
        }

        self
    }

    fn position<P: Into<vizia_window::WindowPosition>>(mut self, position: impl Res<P>) -> Self {
        let entity = self.entity();
        let pos = Some(position.get(&self).into());
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.position = pos;
        }

        self
    }

    fn offset<P: Into<vizia_window::WindowPosition>>(mut self, offset: impl Res<P>) -> Self {
        let entity = self.entity();
        let offset = Some(offset.get(&self).into());
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.offset = offset;
        }

        self
    }

    fn anchor<P: Into<vizia_window::Anchor>>(mut self, anchor: impl Res<P>) -> Self {
        let entity = self.entity();
        let anchor = Some(anchor.get(&self).into());
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.anchor = anchor;
        }

        self
    }

    fn anchor_target<P: Into<vizia_window::AnchorTarget>>(
        mut self,
        anchor_target: impl Res<P>,
    ) -> Self {
        let entity = self.entity();
        let anchor_target = Some(anchor_target.get(&self).into());
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.anchor_target = anchor_target;
        }

        self
    }

    fn parent_anchor<P: Into<Anchor>>(mut self, parent_anchor: impl Res<P>) -> Self {
        let entity = self.entity();
        let parent_anchor = Some(parent_anchor.get(&self).into());
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.parent_anchor = parent_anchor;
        }

        self
    }

    fn resizable(mut self, flag: impl Res<bool>) -> Self {
        let entity = self.entity();
        let flag = flag.get(&self);
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.resizable = flag;
        }

        self
    }

    fn minimized(mut self, flag: impl Res<bool>) -> Self {
        let entity = self.entity();
        let flag = flag.get(&self);
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.minimized = flag;
        }

        self
    }

    fn maximized(mut self, flag: impl Res<bool>) -> Self {
        let entity = self.entity();
        let flag = flag.get(&self);
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.maximized = flag;
        }

        self
    }

    fn visible(mut self, flag: impl Res<bool>) -> Self {
        let entity = self.entity();
        let flag = flag.get(&self);
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.visible = flag
        }

        self
    }

    fn transparent(mut self, flag: bool) -> Self {
        let entity = self.entity();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.transparent = flag
        }

        self
    }

    fn decorations(mut self, flag: bool) -> Self {
        let entity = self.entity();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.decorations = flag
        }

        self
    }

    fn always_on_top(mut self, flag: bool) -> Self {
        let entity = self.entity();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.always_on_top = flag
        }

        self
    }

    fn vsync(mut self, flag: bool) -> Self {
        let entity = self.entity();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.vsync = flag
        }

        self
    }

    fn icon(mut self, width: u32, height: u32, image: Vec<u8>) -> Self {
        let entity = self.entity();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.icon = Some(image);
            win_state.window_description.icon_width = width;
            win_state.window_description.icon_height = height;
        }

        self
    }

    fn enabled_window_buttons(mut self, window_buttons: WindowButtons) -> Self {
        let entity = self.entity();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.enabled_window_buttons = window_buttons;
        }

        self
    }
}
