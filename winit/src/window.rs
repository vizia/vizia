use glutin::event_loop::EventLoopWindowTarget;
#[cfg(not(target_arch = "wasm32"))]
use glutin::ContextBuilder;
use winit::window::WindowBuilder;
use winit::{dpi::*, window::WindowId};

use femtovg::{renderer::OpenGl, Canvas, Color};

use crate::cursor::translate_cursor;
use vizia_core::{Context, Event, Handle, View, WindowDescription, WindowEvent};

pub struct Window {
    pub id: Option<WindowId>,
    pub canvas: Option<Canvas<OpenGl>>,

    #[cfg(not(target_arch = "wasm32"))]
    pub handle: Option<glutin::WindowedContext<glutin::PossiblyCurrent>>,
    #[cfg(target_arch = "wasm32")]
    pub handle: winit::window::Window,
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

        // Build our result!
        let mut result = Window {
            id: handle.id(),
            handle,
            canvas: Canvas::new(renderer).expect("Cannot create canvas"),
        };

        setup_canvas(&mut result);
        result
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
    pub fn new(
        events_loop: &EventLoopWindowTarget<Event>,
        window_description: &WindowDescription,
    ) -> Self {
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

        // Build our result!
        let mut result = Window {
            id: Some(handle.window().id()),
            handle: Some(handle),
            canvas: Some(Canvas::new(renderer).expect("Cannot create canvas")),
        };

        setup_canvas(&mut result);

        result
    }

    pub fn new2(
        cx: &mut Context,
        window_description: WindowDescription,
        content: impl Fn(&mut Context),
    ) -> Handle<Self> {
        let handle = Self { id: None, handle: None, canvas: None }.build(cx, content);

        handle.cx.sub_windows.insert(handle.entity, window_description);

        handle
    }

    pub fn make_current(&mut self) {
        let old_handle = self.handle.take().unwrap();
        self.handle = Some(unsafe { old_handle.make_current().unwrap() });
    }

    pub fn window(&self) -> Option<&winit::window::Window> {
        if let Some(handle) = self.handle.as_ref() {
            Some(handle.window())
        } else {
            None
        }
    }

    pub fn resize(&self, size: PhysicalSize<u32>) {
        if let Some(handle) = self.handle.as_ref() {
            handle.resize(size);
        }
    }

    pub fn swap_buffers(&self) {
        if let Some(handle) = self.handle.as_ref() {
            handle.swap_buffers().expect("Failed to swap buffers");
        }
    }
}

impl View for Window {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        //self.window_widget.on_event(state, entity, event);
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::GrabCursor(flag) => {
                    self.window()
                        .unwrap()
                        .set_cursor_grab(*flag)
                        .expect("Failed to set cursor grab");
                }

                WindowEvent::SetCursorPosition(x, y) => {
                    self.window()
                        .unwrap()
                        .set_cursor_position(winit::dpi::Position::Physical(PhysicalPosition::new(
                            *x as i32, *y as i32,
                        )))
                        .expect("Failed to set cursor position");
                }

                WindowEvent::SetCursor(cursor) => {
                    //println!("Set The Cursor: {:?}", cursor);
                    if let Some(icon) = translate_cursor(*cursor) {
                        self.window().unwrap().set_cursor_visible(true);
                        self.window().unwrap().set_cursor_icon(icon);
                    } else {
                        self.window().unwrap().set_cursor_visible(false);
                    }
                }

                _ => {}
            }
        }
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
    let dpi_factor = result.window().unwrap().scale_factor();
    let size = result.window().unwrap().inner_size();
    result.canvas.as_mut().unwrap().set_size(
        size.width as u32,
        size.height as u32,
        dpi_factor as f32,
    );
    result.canvas.as_mut().unwrap().clear_rect(
        0,
        0,
        size.width as u32,
        size.height as u32,
        Color::rgb(255, 80, 80),
    );
}
