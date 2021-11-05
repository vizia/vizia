
use glutin::{dpi::*, window::WindowId};
use glutin::event_loop::EventLoop;
use glutin::window::{CursorIcon, WindowBuilder};
use glutin::ContextBuilder;

use femtovg::{renderer::OpenGl, Canvas, Color};

use tuix_core::{Entity, State, Widget, WindowDescription, WindowWidget, entity};

pub struct Window {
    pub id: WindowId,
    pub handle: glutin::WindowedContext<glutin::PossiblyCurrent>,
    pub canvas: Canvas<OpenGl>,
    pub window_widget: WindowWidget,
}

impl Window {
    pub fn new(events_loop: &EventLoop<()>, window_description: &WindowDescription) -> Self {
        //Windows COM doesn't play nicely with winit's drag and drop right now
        #[cfg(target_os = "windows")]
        let mut window_builder = {
            use glutin::platform::windows::WindowBuilderExtWindows;
            WindowBuilder::new().with_drag_and_drop(false)
        };
        #[cfg(not(target_os = "windows"))]
        let mut window_builder = WindowBuilder::new();

        window_builder = window_builder
            .with_title(&window_description.title)
            .with_inner_size(PhysicalSize::new(
                window_description.inner_size.width,
                window_description.inner_size.height,
            ))
            .with_min_inner_size(PhysicalSize::new(
                window_description.min_inner_size.width,
                window_description.min_inner_size.height,
            ))
            .with_window_icon(if let Some(icon) = &window_description.icon {
                Some(
                    glutin::window::Icon::from_rgba(
                        icon.clone(),
                        window_description.icon_width,
                        window_description.icon_height,
                    )
                    .unwrap(),
                )
            } else {
                None
            });

        let handle = ContextBuilder::new()
            .with_vsync(true)
            // .with_srgb(true)
            .build_windowed(window_builder, &events_loop)
            .expect("Window context creation failed!");

        let handle = unsafe { handle.make_current().unwrap() };

        let renderer = OpenGl::new(|s| handle.context().get_proc_address(s) as *const _)
            .expect("Cannot create renderer");
        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        let dpi_factor = handle.window().scale_factor();
        let size = handle.window().inner_size();

        canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        canvas.clear_rect(
            0,
            0,
            size.width as u32,
            size.height as u32,
            Color::rgb(255, 80, 80),
        );

        // let height = size.height as f32;
        // let width = size.width as f32;

        Window { 
            id: handle.window().id(),
            handle, 
            canvas,
            window_widget: WindowWidget::new(),
        }
    }
}

impl Widget for Window {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
    
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut tuix_core::Event) {
        self.window_widget.on_event(state, entity, event);

        if let Some(window_event) = event.message.downcast() {
            match window_event {

                tuix_core::WindowEvent::GrabCursor(flag) => {
                    self.handle.window().set_cursor_grab(*flag);
                }

                tuix_core::WindowEvent::SetCursorPosition(x, y) => {
                    self.handle.window().set_cursor_position(Position::Physical(PhysicalPosition::new(*x as i32, *y as i32)));
                }

                tuix_core::WindowEvent::SetCursor(cursor) => {
                    match *cursor {
                        tuix_core::CursorIcon::Default => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Default);
                        }

                        tuix_core::CursorIcon::Crosshair => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Crosshair);
                        }

                        tuix_core::CursorIcon::Hand => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Hand);
                        }

                        tuix_core::CursorIcon::Arrow => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Arrow);
                        }

                        tuix_core::CursorIcon::Move => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Move);
                        }

                        tuix_core::CursorIcon::Text => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Text);
                        }

                        tuix_core::CursorIcon::Wait => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Wait);
                        }

                        tuix_core::CursorIcon::Help => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Help);
                        }

                        tuix_core::CursorIcon::Progress => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Progress);
                        }

                        tuix_core::CursorIcon::NotAllowed => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::NotAllowed);
                        }

                        tuix_core::CursorIcon::ContextMenu => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::ContextMenu);
                        }

                        tuix_core::CursorIcon::Cell => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Cell);
                        }

                        tuix_core::CursorIcon::VerticalText => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::VerticalText);
                        }

                        tuix_core::CursorIcon::Alias => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Alias);
                        }

                        tuix_core::CursorIcon::Copy => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Copy);
                        }

                        tuix_core::CursorIcon::NoDrop => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::NoDrop);
                        }

                        tuix_core::CursorIcon::Grab => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Grab);
                        }

                        tuix_core::CursorIcon::Grabbing => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::Grabbing);
                        }

                        tuix_core::CursorIcon::AllScroll => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::AllScroll);
                        }

                        tuix_core::CursorIcon::ZoomIn => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::ZoomIn);
                        }

                        tuix_core::CursorIcon::ZoomOut => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::ZoomOut);
                        }

                        tuix_core::CursorIcon::EResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::EResize);
                        }

                        tuix_core::CursorIcon::NResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::NResize);
                        }

                        tuix_core::CursorIcon::NeResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::NeResize);
                        }

                        tuix_core::CursorIcon::NwResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::NwResize);
                        }

                        tuix_core::CursorIcon::SResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::SResize);
                        }

                        tuix_core::CursorIcon::SeResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::SeResize);
                        }

                        tuix_core::CursorIcon::SwResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::SwResize);
                        }

                        tuix_core::CursorIcon::WResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::WResize);
                        }

                        tuix_core::CursorIcon::EwResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::EwResize);
                        }

                        tuix_core::CursorIcon::NsResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::NsResize);
                        }

                        tuix_core::CursorIcon::NeswResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::NeswResize);
                        }

                        tuix_core::CursorIcon::NwseResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::NwseResize);
                        }

                        tuix_core::CursorIcon::ColResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::ColResize);
                        }

                        tuix_core::CursorIcon::RowResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(CursorIcon::RowResize);
                        }

                        tuix_core::CursorIcon::None => {
                            self.handle.window().set_cursor_visible(false);
                        }
                    }
                }

                _=> {}
            }
        }
    }
}
