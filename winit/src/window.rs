#[cfg(not(target_arch = "wasm32"))]
use glutin::ContextBuilder;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::{dpi::*, window::WindowId};

use femtovg::{renderer::OpenGl, Canvas, Color};

use crate::cursor::translate_cursor;
use vizia_core::{Context, Event, View, WindowDescription, WindowEvent};

pub struct Window {
    pub id: WindowId,
    pub canvas: Canvas<OpenGl>,

    #[cfg(not(target_arch = "wasm32"))]
    handle: glutin::WindowedContext<glutin::PossiblyCurrent>,
    #[cfg(target_arch = "wasm32")]
    handle: winit::window::Window,
}

impl Window {
    pub fn new(events_loop: &EventLoop<Event>, window_description: &WindowDescription) -> Self {
        #[allow(unused_mut)]
        let mut window_builder = WindowBuilder::new();

        //Windows COM doesn't play nicely with winit's drag and drop right now
        #[cfg(target_os = "windows")]
        let mut window_builder = {
            use winit::platform::windows::WindowBuilderExtWindows;
            window_builder.with_drag_and_drop(false)
        };

        // For wasm, create or look up the canvas element we're drawing on
        #[cfg(target_arch = "wasm32")]
        let canvas_element = {
            use wasm_bindgen::JsCast;

            let document = web_sys::window().unwrap().document().unwrap();

            if let Some(canvas_id) = &window_description.target_canvas {
                document.get_element_by_id(canvas_id).unwrap()
            } else {
                let element = document.create_element("canvas").unwrap();
                document.body().unwrap().insert_adjacent_element("afterbegin", &element).unwrap();
                element
            }
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap()
        };

        // Build the femtovg renderer, PART 1/2, web edition
        // these two parts have to be separated because they have conflicting requirements
        // on use of the referenced passed to window builder
        #[cfg(target_arch = "wasm32")]
        let renderer = OpenGl::new_from_html_canvas(&canvas_element).unwrap();

        // For wasm, tell winit about the above canvas
        #[cfg(target_arch = "wasm32")]
        let mut window_builder = {
            use winit::platform::web::WindowBuilderExtWebSys;
            window_builder.with_canvas(Some(canvas_element))
        };

        // Apply generic WindowBuilder properties
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
            .with_always_on_top(window_description.always_on_top)
            .with_resizable(window_description.resizable)
            .with_window_icon(window_description.icon.as_ref().map(|icon| {
                winit::window::Icon::from_rgba(
                    icon.clone(),
                    window_description.icon_width,
                    window_description.icon_height,
                )
                .unwrap()
            }));

        // Get the window handle. for glutin this is a ContextWrapper, and for web this is a
        // winit::window::Window
        #[cfg(not(target_arch = "wasm32"))]
        let handle = {
            let handle = ContextBuilder::new()
                .with_vsync(true)
                // .with_srgb(true)
                .build_windowed(window_builder, events_loop)
                .expect("Window context creation failed!");

            unsafe { handle.make_current().unwrap() }
        };
        #[cfg(target_arch = "wasm32")]
        let handle = window_builder.build(&events_loop).unwrap();

        // Build the femtovg renderer, PART 2/2, glutin edition
        #[cfg(not(target_arch = "wasm32"))]
        let renderer = OpenGl::new_from_glutin_context(&handle).expect("Cannot create renderer");

        // Build our result!
        let mut result = Window {
            #[cfg(not(target_arch = "wasm32"))]
            id: handle.window().id(),
            #[cfg(target_arch = "wasm32")]
            id: handle.id(),
            handle,
            canvas: Canvas::new(renderer).expect("Cannot create canvas"),
        };

        // Set some initial properties on our result canvas
        let dpi_factor = result.window().scale_factor();
        let size = result.window().inner_size();
        result.canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        result.canvas.clear_rect(
            0,
            0,
            size.width as u32,
            size.height as u32,
            Color::rgb(255, 80, 80),
        );

        result
    }

    pub fn window(&self) -> &winit::window::Window {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.handle.window()
        }
        #[cfg(target_arch = "wasm32")]
        &self.handle
    }

    pub fn resize(&self, size: PhysicalSize<u32>) {
        #[cfg(not(target_arch = "wasm32"))]
        self.handle.resize(size);
    }

    pub fn swap_buffers(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.handle.swap_buffers().expect("Failed to swap buffers");
    }
}

impl View for Window {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        //self.window_widget.on_event(state, entity, event);
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::GrabCursor(flag) => {
                    self.window().set_cursor_grab(*flag).expect("Failed to set cursor grab");
                }

                WindowEvent::SetCursorPosition(x, y) => {
                    self.window()
                        .set_cursor_position(winit::dpi::Position::Physical(PhysicalPosition::new(
                            *x as i32, *y as i32,
                        )))
                        .expect("Failed to set cursor position");
                }

                WindowEvent::SetCursor(cursor) => {
                    //println!("Set The Cursor: {:?}", cursor);
                    if let Some(icon) = translate_cursor(*cursor) {
                        self.window().set_cursor_visible(true);
                        self.window().set_cursor_icon(icon);
                    } else {
                        self.window().set_cursor_visible(false);
                    }
                }

                _ => {}
            }
        }
    }
}
