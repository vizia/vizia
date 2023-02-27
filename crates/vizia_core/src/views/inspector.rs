use femtovg::{Path, Paint, Align, Baseline, FontId};
use morphorm::{PositionType, Units, LayoutType};

use crate::{Context, View, Handle, VStack, HStack, ZStack, Label, Units::*, Color, Model, Entity, TreeExt, List, Lens, Actions, Binding, Overflow, FontOrId, PropSet, context, Canvas, WindowEvent, MouseButton, Display};


#[derive(Lens)]
pub struct InspectorData {
    canvas: Entity,
    overlay: Entity,
    selected: Entity,

    sub_tree: Vec<Entity>,
}

impl InspectorData {
    fn build_tree(&mut self, cx: &Context, parent: Entity) {
        println!("Node: {} {:?}", parent, cx.cache.get_display(parent));
        if parent != self.canvas && parent != self.overlay && 
            cx.cache.get_display(parent) != Display::None 
            && cx.style.display.get(parent).cloned().unwrap_or_default() != Display::None    
        {
            self.sub_tree.push(parent);
        }
        for child in parent.child_iter(&cx.tree) {
            self.build_tree(cx, child);
        }
    }
}

#[derive(Debug)]
pub enum InspectorEvent {
    SetRoot(Entity),
    SetOverlay(Entity),
    SetSelected(Entity),
}

impl Model for InspectorData {
    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(inspector_event) = event.message.downcast() {
            match inspector_event {

                InspectorEvent::SetRoot(root) => {
                    self.canvas = *root;
                    self.build_tree(cx, self.canvas);
                }

                InspectorEvent::SetOverlay(overlay) => {
                    self.overlay = *overlay;
                }

                InspectorEvent::SetSelected(selected) => {
                    if selected.is_descendant_of(&cx.tree, self.canvas) && *selected != self.overlay {
                        self.selected = *selected;
                    }
                }
            }
        }
    }
}

pub struct Inspector {

}

impl Inspector {
    pub fn new<F>(cx: &mut Context, builder: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        Self {
            
        }.build2(cx, move |cx|{

            if cx.data::<InspectorData>().is_none() {
                InspectorData {
                    canvas: Entity::null(),
                    selected: Entity::null(),
                    overlay: Entity::null(),
                    sub_tree: Vec::new(),
                }.build(cx);
            }

            HStack::new(cx, move |cx|{
                VStack::new(cx, move |cx|{
                    (builder)(cx);
                    Overlay::new(cx).z_order(100).overflow(Overflow::Visible).position_type(PositionType::SelfDirected);
                    cx.emit(InspectorEvent::SetRoot(cx.current));
                }).background_color(Color::rgb(200,200,200)).space(Pixels(50.0)).size(Stretch(1.0));


                List::new(cx, InspectorData::sub_tree, |cx, item|{
                    Binding::new(cx, InspectorData::selected, move |cx, selected|{
                        let entity = *item.get(cx);
                        let selected_color = if *selected.get(cx) == entity {Color::blue()} else {Color::rgb(150,150,150)};
                        Label::new(cx, &item.get(cx).to_string())
                            .width(Stretch(1.0))
                            .height(Pixels(30.0))
                            .background_color(selected_color)
                            .on_press(move |cx| cx.emit(InspectorEvent::SetSelected(entity)));
                    });
                }).width(Pixels(300.0)).height(Stretch(1.0)).background_color(Color::rgb(150, 150, 150));

                // VStack::new(cx, |cx|{
                    
                // }).width(Pixels(300.0)).background_color(Color::rgb(150, 150, 150));
            });

        })
    }
}

impl View for Inspector {
    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    cx.emit(InspectorEvent::SetSelected(cx.hovered));
                }

                _=> {}
            }
        }
    }
}

pub struct Overlay {

}

impl Overlay {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {

        }.build2(cx, |cx|{
            cx.emit(InspectorEvent::SetOverlay(cx.current));
        }).hoverable(false)
    }
}

impl View for Overlay {

    fn draw(&self, cx: &mut Context, canvas: &mut crate::Canvas) {
        // println!("current: {:?}", cx.cache.get_bounds(cx.current));
        // let overlay_bounds = cx.cache.get_bounds(cx.current);
        // let mut path = Path::new();
        // path.rect(overlay_bounds.x + 1.0, overlay_bounds.y + 1.0, overlay_bounds.w - 2.0, overlay_bounds.h - 2.0);
        // let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
        // paint.set_line_width(2.0);
        // canvas.stroke_path(&mut path, paint);
        

        if let Some(inspector_data) = cx.data::<InspectorData>() {
            if inspector_data.selected != Entity::null() {

                let parent = inspector_data.selected.get_parent(cx).unwrap();

                let default_font = cx
                    .resource_manager
                    .fonts
                    .get(&cx.style.default_font)
                    .and_then(|font| match font {
                        FontOrId::Id(id) => Some(id),
                        _ => None,
                    })
                    .expect("Failed to find default font");

                let font_id = cx
                    .resource_manager
                    .fonts
                    .get("roboto-bold")
                    .and_then(|font| match font {
                        FontOrId::Id(id) => Some(id),
                        _ => None,
                    })
                    .unwrap_or(default_font);
                
                // Draw outline
                let bounds = cx.cache.get_bounds(inspector_data.selected);
                let mut path = Path::new();
                path.rect(bounds.x + 1.0, bounds.y + 1.0, bounds.w - 2.0, bounds.h - 2.0);
                let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
                paint.set_line_width(2.0);
                canvas.stroke_path(&mut path, paint);


                let child_top = cx.style.child_top.get(parent).cloned().unwrap_or_default();
                let child_bottom = cx.style.child_bottom.get(parent).cloned().unwrap_or_default();
                let child_left = cx.style.child_left.get(parent).cloned().unwrap_or_default();
                let child_right = cx.style.child_left.get(parent).cloned().unwrap_or_default();
                let parent_layout_type = cx.style.layout_type.get(parent).cloned().unwrap_or_default();
                for (index, child) in parent.child_iter(&cx.tree).enumerate() {

                    if child == cx.current {
                        continue;
                    }

                    let mut height = cx.style.height.get(child).cloned().unwrap_or_default();
                    let mut width = cx.style.width.get(child).cloned().unwrap_or_default();

                    let mut top = cx.style.top.get(child).cloned().unwrap_or_default();
                    if top == Units::Auto {
                        top = child_top;
                    };

                    let mut bottom = cx.style.bottom.get(child).cloned().unwrap_or_default();
                    if bottom == Units::Auto {
                        bottom = child_bottom;
                    };

                    let mut left = cx.style.left.get(child).cloned().unwrap_or_default();
                    if left == Units::Auto {
                        left = child_left;
                    };

                    let mut right = cx.style.right.get(child).cloned().unwrap_or_default();
                    if right == Units::Auto {
                        right = child_right;
                    };

                    println!("right: {:?}", right);

                    let space = cx.cache.space.get(child).unwrap();

                    println!("{:?}", space);

                    let bounds = cx.cache.get_bounds(child);

                    

                    if child == inspector_data.selected {
                        draw_vertical_marker(bounds.y, bounds.h, *font_id, true, canvas, &print_units(height));
                        
                        if space.top != 0.0 {
                            draw_vertical_marker(bounds.y - space.top, space.top, *font_id, true, canvas, &print_units(top));
                        }
    
                        if space.bottom != 0.0 {
                            draw_vertical_marker(bounds.y + bounds.h, space.bottom, *font_id, true, canvas, &print_units(bottom));
                        }

                        draw_horizontal_marker(bounds.x, bounds.w, *font_id, true, canvas, &print_units(width));
                        
                        if space.left != 0.0 {
                            draw_horizontal_marker(bounds.x - space.left, space.left, *font_id, true, canvas, &print_units(left));
                        }
    
                        if space.right != 0.0 {
                            draw_horizontal_marker(bounds.x + bounds.w, space.right, *font_id, true, canvas, &print_units(right));
                        }

                    } else {
                        if parent_layout_type == LayoutType::Column {
                            draw_vertical_marker(bounds.y, bounds.h, *font_id, false, canvas, &print_units(height));
                            if space.top != 0.0 {
                                draw_vertical_marker(bounds.y - space.top, space.top, *font_id, false, canvas, &print_units(top));
                            }
        
                            if space.bottom != 0.0 {
                                draw_vertical_marker(bounds.y + bounds.h, space.bottom, *font_id, false, canvas, &print_units(bottom));
                            }
                        } else if parent_layout_type == LayoutType::Row {
                            draw_horizontal_marker(bounds.x, bounds.w, *font_id, false, canvas, &print_units(width));
                            
                            if space.left != 0.0 {
                                draw_horizontal_marker(bounds.x - space.left, space.left, *font_id, false, canvas, &print_units(left));
                            }
        
                            if space.right != 0.0 {
                                draw_horizontal_marker(bounds.x + bounds.w, space.right, *font_id, false, canvas, &print_units(right));
                            }
                        }

                    }

                    

                    // // Draw line
                    // let mut path = Path::new();
                    // path.move_to(bounds.x - 30.0, bounds.y);
                    // path.line_to(bounds.x - 20.0, bounds.y);
                    // path.move_to(bounds.x - 25.0, bounds.y);
                    // path.line_to(bounds.x - 25.0, bounds.y + bounds.h);
                    // path.move_to(bounds.x - 30.0, bounds.y + bounds.h);
                    // path.line_to(bounds.x - 20.0, bounds.y + bounds.h);
                    // let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
                    // paint.set_line_width(2.0);
                    // canvas.stroke_path(&mut path, paint);
    
                    // // Draw box
                    // let mut path = Path::new();
                    // path.rounded_rect(bounds.x - 40.0, bounds.y + bounds.h/2.0 - 10.0, 30.0, 20.0, 1.0);
                    // let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
                    // canvas.fill_path(&mut path, paint);
    
                    // // Draw text
                    // let mut text_paint = Paint::color(femtovg::Color::rgb(50, 50, 50));
                    // text_paint.set_text_align(Align::Center);
                    // text_paint.set_text_baseline(Baseline::Middle);
                    // text_paint.set_font(&[*font_id]);
                    // text_paint.set_font_size(12.0);
                    // canvas.fill_text(bounds.x - 24.5, bounds.y + bounds.h/2.0 + 0.5, &bounds.h.to_string(), text_paint);
    
                    // let mut text_paint = Paint::color(femtovg::Color::rgb(255, 255, 255));
                    // text_paint.set_text_align(Align::Center);
                    // text_paint.set_text_baseline(Baseline::Middle);
                    // text_paint.set_font(&[*font_id]);
                    // text_paint.set_font_size(12.0);
                    // canvas.fill_text(bounds.x - 25.0, bounds.y + bounds.h/2.0, &bounds.h.to_string(), text_paint);
                }

            }
        }
    }
}

fn draw_vertical_marker(y: f32, h: f32, font_id: FontId, filled: bool, canvas: &mut Canvas, text: &str) {

    // Draw line
    let mut path = Path::new();
    path.move_to(20.0, y);
    path.line_to(30.0, y);
    path.move_to(25.0, y);
    path.line_to(25.0, y + h);
    path.move_to(20.0, y + h);
    path.line_to(30.0, y + h);
    let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
    paint.set_line_width(2.0);
    canvas.stroke_path(&mut path, paint);

    // Draw box
    let mut path = Path::new();
    path.rounded_rect(10.0, y + h/2.0 - 10.0, 30.0, 20.0, 1.0);
    if filled {
        canvas.fill_path(&mut path, Paint::color(femtovg::Color::rgb(255, 0, 255)));
    } else {
        canvas.fill_path(&mut path, Paint::color(femtovg::Color::rgb(255, 255, 255)));
    }
    let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
    paint.set_line_width(2.0);
    canvas.stroke_path(&mut path, paint);

    // Draw text
    let mut text_paint = Paint::color(femtovg::Color::rgb(0, 0, 0));
    text_paint.set_text_align(Align::Center);
    text_paint.set_text_baseline(Baseline::Middle);
    text_paint.set_font(&[font_id]);
    text_paint.set_font_size(12.0);
    canvas.fill_text(25.5, y + h/2.0 + 0.5, text, text_paint);
}

fn draw_horizontal_marker(x: f32, w: f32, font_id: FontId, filled: bool, canvas: &mut Canvas, text: &str) {

    // Draw line
    let mut path = Path::new();
    path.move_to(x, 20.0);
    path.line_to(x, 30.0);
    path.move_to(x, 25.0);
    path.line_to(x + w, 25.0);
    path.move_to(x + w, 20.0);
    path.line_to(x + w, 30.0);
    let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
    paint.set_line_width(2.0);
    canvas.stroke_path(&mut path, paint);

    // Draw box
    let mut path = Path::new();
    path.rounded_rect(x + w/2.0 - 15.0, 15.0, 30.0, 20.0, 1.0);
    if filled {
        canvas.fill_path(&mut path, Paint::color(femtovg::Color::rgb(255, 0, 255)));
    } else {
        canvas.fill_path(&mut path, Paint::color(femtovg::Color::rgb(255, 255, 255)));
    }
    let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 255));
    paint.set_line_width(2.0);
    canvas.stroke_path(&mut path, paint);

    // Draw text
    let mut text_paint = Paint::color(femtovg::Color::rgb(0, 0, 0));
    text_paint.set_text_align(Align::Center);
    text_paint.set_text_baseline(Baseline::Middle);
    text_paint.set_font(&[font_id]);
    text_paint.set_font_size(12.0);
    canvas.fill_text(x + w/2.0, 25.0, text, text_paint);
}

pub fn print_units(units: Units) -> String {
    match units {
        Pixels(val) => format!("{}", val),
        Percentage(val) => format!("{}%", val),
        Stretch(val) => format!("{}s", val),
        Auto => "auto".to_string(),
    }
}