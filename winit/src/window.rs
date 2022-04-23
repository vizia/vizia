use crate::convert::cursor_icon_to_cursor_icon;
use femtovg::{renderer::OpenGl, Canvas, Color};
#[cfg(not(target_arch = "wasm32"))]
use glutin::ContextBuilder;
use vizia_core::{Context, Event, View, WindowDescription, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::{dpi::*, window::WindowId};

pub struct Window {
    pub id: WindowId,
    pub canvas: Canvas<OpenGl>,

    #[cfg(not(target_arch = "wasm32"))]
    handle: glutin::WindowedContext<glutin::PossiblyCurrent>,
    #[cfg(target_arch = "wasm32")]
    handle: winit::window::Window,
}

#[cfg(target_arch = "wasm32")]
impl Window {
    pub fn new(events_loop: &EventLoop<Event>, window_description: &WindowDescription) -> Self {
        let window_builder = WindowBuilder::new();

        // For wasm, create or look up the canvas element we're drawing on
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

        // Build the femtovg renderer
        let renderer = OpenGl::new_from_html_canvas(&canvas_element).unwrap();

        // tell winit about the above canvas
        let window_builder = {
            use winit::platform::web::WindowBuilderExtWebSys;
            window_builder.with_canvas(Some(canvas_element))
        };

        // Apply generic WindowBuilder properties
        let window_builder = apply_window_description(window_builder, &window_description);

        // Get the window handle. this is a winit::window::Window
        let handle = window_builder.build(&events_loop).unwrap();

        // Build our window
        let mut window = Window {
            id: handle.id(),
            handle,
            canvas: Canvas::new(renderer).expect("Cannot create canvas"),
        };

        setup_canvas(&mut window);
        window
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.handle
    }

    pub fn resize(&self, _size: PhysicalSize<u32>) {
        // TODO?
    }

    pub fn swap_buffers(&self) {
        // Intentional no-op
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Window {
    pub fn new(events_loop: &EventLoop<Event>, window_description: &WindowDescription) -> Self {
        let window_builder = WindowBuilder::new();

        //Windows COM doesn't play nicely with winit's drag and drop right now
        #[cfg(target_os = "windows")]
        let window_builder = {
            use winit::platform::windows::WindowBuilderExtWindows;
            window_builder.with_drag_and_drop(false)
        };

        // Apply generic WindowBuilder properties
        let window_builder = apply_window_description(window_builder, &window_description);

        // Get the window handle. this is a ContextWrapper
        let handle = {
            let handle = ContextBuilder::new()
                .with_vsync(window_description.vsync)
                // .with_srgb(true)
                .build_windowed(window_builder, &events_loop)
                .expect("Window context creation failed!");

            unsafe { handle.make_current().unwrap() }
        };

        // Build the femtovg renderer
        let renderer = OpenGl::new_from_glutin_context(&handle).expect("Cannot create renderer");

        // Build our window
        let mut window = Window {
            id: handle.window().id(),
            handle,
            canvas: Canvas::new(renderer).expect("Cannot create canvas"),
        };

        setup_canvas(&mut window);
        window
    }

    pub fn window(&self) -> &winit::window::Window {
        self.handle.window()
    }

    pub fn resize(&self, size: PhysicalSize<u32>) {
        self.handle.resize(size);
    }

    pub fn swap_buffers(&self) {
        self.handle.swap_buffers().expect("Failed to swap buffers");
    }
}

impl View for Window {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        event.map(|window_event, _| match window_event {
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
                if let Some(icon) = cursor_icon_to_cursor_icon(*cursor) {
                    self.window().set_cursor_visible(true);
                    self.window().set_cursor_icon(icon);
                } else {
                    self.window().set_cursor_visible(false);
                }
            }

            _ => {}
        })
    }
}

fn apply_window_description(
    builder: WindowBuilder,
    description: &WindowDescription,
) -> WindowBuilder {
    builder
        .with_title(&description.title)
        .with_inner_size(LogicalSize::new(
            description.inner_size.width,
            description.inner_size.height,
        ))
        .with_min_inner_size(LogicalSize::new(
            description.min_inner_size.width,
            description.min_inner_size.height,
        ))
        .with_always_on_top(description.always_on_top)
        .with_resizable(description.resizable)
        .with_window_icon(if let Some(icon) = &description.icon {
            Some(
                winit::window::Icon::from_rgba(
                    icon.clone(),
                    description.icon_width,
                    description.icon_height,
                )
                .unwrap(),
            )
        } else {
            None
        })
}

fn setup_canvas(result: &mut Window) {
    // Set some initial properties on our result canvas
    let size = result.window().inner_size();
    result.canvas.set_size(size.width as u32, size.height as u32, 1.0);
    result.canvas.clear_rect(0, 0, size.width as u32, size.height as u32, Color::rgb(255, 80, 80));
}
