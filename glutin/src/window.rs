use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::{dpi::*, window::WindowId};

use femtovg::{renderer::OpenGl, Canvas, Color};

use vizia_core::{Context, CursorIcon, Event, View, WindowDescription, WindowEvent};

pub struct Window {
    pub id: WindowId,
    pub handle: glutin::WindowedContext<glutin::PossiblyCurrent>,
    pub canvas: Canvas<OpenGl>,
    //pub window_widget: WindowWidget,
}

impl Window {
    pub fn new(events_loop: &EventLoop<Event>, window_description: &WindowDescription) -> Self {
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
        canvas.clear_rect(0, 0, size.width as u32, size.height as u32, Color::rgb(255, 80, 80));

        // let height = size.height as f32;
        // let width = size.width as f32;

        Window {
            id: handle.window().id(),
            handle,
            canvas,
            //window_widget: WindowWidget::new(),
        }
    }
}

impl View for Window {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        //self.window_widget.on_event(state, entity, event);
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::GrabCursor(flag) => {
                    self.handle.window().set_cursor_grab(*flag).expect("Failed to set cursor grab");
                }

                WindowEvent::SetCursorPosition(x, y) => {
                    self.handle
                        .window()
                        .set_cursor_position(glutin::dpi::Position::Physical(
                            PhysicalPosition::new(*x as i32, *y as i32),
                        ))
                        .expect("Failed to set cursor position");
                }

                WindowEvent::SetCursor(cursor) => {
                    //println!("Set The Cursor: {:?}", cursor);
                    match *cursor {
                        CursorIcon::Default => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::Default);
                        }

                        CursorIcon::Crosshair => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::Crosshair);
                        }

                        CursorIcon::Hand => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(glutin::window::CursorIcon::Hand);
                        }

                        CursorIcon::Arrow => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(glutin::window::CursorIcon::Arrow);
                        }

                        CursorIcon::Move => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(glutin::window::CursorIcon::Move);
                        }

                        CursorIcon::Text => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(glutin::window::CursorIcon::Text);
                        }

                        CursorIcon::Wait => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(glutin::window::CursorIcon::Wait);
                        }

                        CursorIcon::Help => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(glutin::window::CursorIcon::Help);
                        }

                        CursorIcon::Progress => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::Progress);
                        }

                        CursorIcon::NotAllowed => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::NotAllowed);
                        }

                        CursorIcon::ContextMenu => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::ContextMenu);
                        }

                        CursorIcon::Cell => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(glutin::window::CursorIcon::Cell);
                        }

                        CursorIcon::VerticalText => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::VerticalText);
                        }

                        CursorIcon::Alias => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(glutin::window::CursorIcon::Alias);
                        }

                        CursorIcon::Copy => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(glutin::window::CursorIcon::Copy);
                        }

                        CursorIcon::NoDrop => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::NoDrop);
                        }

                        CursorIcon::Grab => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle.window().set_cursor_icon(glutin::window::CursorIcon::Grab);
                        }

                        CursorIcon::Grabbing => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::Grabbing);
                        }

                        CursorIcon::AllScroll => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::AllScroll);
                        }

                        CursorIcon::ZoomIn => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::ZoomIn);
                        }

                        CursorIcon::ZoomOut => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::ZoomOut);
                        }

                        CursorIcon::EResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::EResize);
                        }

                        CursorIcon::NResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::NResize);
                        }

                        CursorIcon::NeResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::NeResize);
                        }

                        CursorIcon::NwResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::NwResize);
                        }

                        CursorIcon::SResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::SResize);
                        }

                        CursorIcon::SeResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::SeResize);
                        }

                        CursorIcon::SwResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::SwResize);
                        }

                        CursorIcon::WResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::WResize);
                        }

                        CursorIcon::EwResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::EwResize);
                        }

                        CursorIcon::NsResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::NsResize);
                        }

                        CursorIcon::NeswResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::NeswResize);
                        }

                        CursorIcon::NwseResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::NwseResize);
                        }

                        CursorIcon::ColResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::ColResize);
                        }

                        CursorIcon::RowResize => {
                            self.handle.window().set_cursor_visible(true);
                            self.handle
                                .window()
                                .set_cursor_icon(glutin::window::CursorIcon::RowResize);
                        }

                        CursorIcon::None => {
                            self.handle.window().set_cursor_visible(false);
                        }
                    }
                }

                _ => {}
            }
        }
    }
}
