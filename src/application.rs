use std::collections::{HashMap, VecDeque};

use femtovg::{Align, Baseline, Canvas, Paint, Path, renderer::OpenGl};
use glutin::{ContextBuilder, event_loop::{self, ControlFlow, EventLoop}, window::WindowBuilder};
use morphorm::Cache;

use crate::{CachedData, Color, Context, Entity, Event, EventManager, Handle, IdManager, MouseButton, MouseButtonState, MouseState, Propagation, Style, Tree, WindowEvent, apply_hover, style};

static FONT: &[u8] = include_bytes!("Roboto-Regular.ttf");

pub struct Application {
    context: Context,
}

impl Application {
    pub fn new<F>(func: F) -> Self
    where F: FnOnce(&mut Context)
    {

        let mut cache = CachedData::default();
        cache.add(Entity::root());

        let mut context = Context {
            entity_manager: IdManager::new(),
            tree: Tree::new(),
            current: Entity::root(),
            count: 0,
            views: HashMap::new(),
            state: HashMap::new(),  
            data: HashMap::new(),
            style: Style::default(),
            cache,
            event_queue: VecDeque::new(),
            mouse: MouseState::default(),
            hovered: Entity::root(),
            state_count: 0,
        };

        context.entity_manager.create();

        (func)(&mut context);

        Self {
            context,
        }
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.context.style.background_color.insert(Entity::root(), color);

        self
    }

    pub fn run(self) {

        let mut context = self.context;
        
        let event_loop = EventLoop::new();
        
        let handle = ContextBuilder::new()
            .build_windowed(WindowBuilder::new(), &event_loop)
            .expect("Failed to build windowed context");

        let handle = unsafe { handle.make_current().unwrap() };

        let renderer = OpenGl::new(|s| handle.context().get_proc_address(s) as *const _)
            .expect("Cannot create renderer");
        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        let font = canvas.add_font_mem(FONT).expect("Failed to load font");

        let dpi_factor = handle.window().scale_factor();
        let size = handle.window().inner_size();

        let clear_color = context.style.background_color.get(Entity::root()).cloned().unwrap_or_default();

        canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        canvas.clear_rect(
            0,
            0,
            size.width as u32,
            size.height as u32,
            clear_color.into(),
        );

        context
            .cache
            .set_width(Entity::root(), 800.0);
        context
            .cache
            .set_height(Entity::root(), 600.0);

        let mut event_manager = EventManager::new();

        event_loop.run(move |event, _, control_flow|{
            *control_flow = ControlFlow::Wait;

            match event {
                glutin::event::Event::MainEventsCleared => {

                    while !context.event_queue.is_empty() {
                        event_manager.flush_events(&mut context);
                    }

                    // Process VIZIA events here
                    morphorm::layout(&mut context.cache, &context.tree, &context.style);

                    handle.window().request_redraw();
                }

                glutin::event::Event::RedrawRequested(_) => {
                    // Redraw here
                    //println!("Redraw");
                    let clear_color = context.style.background_color.get(Entity::root()).cloned().unwrap_or(Color::white());
                    canvas.clear_rect(
                        0,
                        0,
                        size.width as u32,
                        size.height as u32,
                        clear_color.into(),
                    );
                    for entity in context.tree.clone().into_iter() {
                        //println!("{}", debug(&mut context, entity));
                        let bounds = context.cache.get_bounds(entity);
                        let mut path = Path::new();
                        path.rect(bounds.x, bounds.y, bounds.w, bounds.h);

                        let background_color: femtovg::Color = context.style.background_color.get(entity).cloned().unwrap_or_default().into();
                        canvas.fill_path(&mut path, Paint::color(background_color));
                        
                        if let Some(text) = context.style.text.get(entity) {
                            let mut paint = Paint::color(femtovg::Color::black());
                            paint.set_font(&[font]);
                            paint.set_text_align(Align::Center);
                            paint.set_text_baseline(Baseline::Middle);
                            canvas.fill_text(bounds.x + bounds.w / 2.0, bounds.y + bounds.h / 2.0, text, paint);
                        }
                    }

                    canvas.flush();
                    handle.swap_buffers().expect("Failed to swap buffers");
                }

                glutin::event::Event::WindowEvent {
                    window_id: _,
                    event,
                } => {
                    match event {
                        glutin::event::WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }

                        glutin::event::WindowEvent::CursorMoved {
                            device_id,
                            position,
                            modifiers
                        } => {

                            context.mouse.cursorx = position.x as f32;
                            context.mouse.cursory = position.y as f32;

                            apply_hover(&mut context);
                        }

                        glutin::event::WindowEvent::MouseInput {
                            device_id,
                            button,
                            state,
                            modifiers,
                        } => {
                            let button = match button {
                                glutin::event::MouseButton::Left => MouseButton::Left,
                                glutin::event::MouseButton::Right => MouseButton::Right,
                                glutin::event::MouseButton::Middle => MouseButton::Middle,
                                glutin::event::MouseButton::Other(val) => MouseButton::Other(val),
                            };

                            let state = match state {
                                glutin::event::ElementState::Pressed => MouseButtonState::Pressed,
                                glutin::event::ElementState::Released => MouseButtonState::Released,
                            };

                            match state {
                                MouseButtonState::Pressed => {
                                    context.event_queue.push_back(Event::new(WindowEvent::MouseDown(button)).target(context.hovered).propagate(Propagation::Direct));
                                }

                                MouseButtonState::Released => {
                                    context.event_queue.push_back(Event::new(WindowEvent::MouseUp(button)).target(context.hovered).propagate(Propagation::Direct));
                                }
                            }
                        }

                        _=> {}
                    }
                }

                _=> {}
            }
        });
    }
}

fn debug(cx: &mut Context, entity: Entity) -> String {
    if let Some(view) = cx.views.get(&entity) {
        view.debug(entity)
    } else {
        "None".to_string()
    }
}