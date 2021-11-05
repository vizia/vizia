use crate::{Context, Entity, Event, Handle, ViewHandler};

use femtovg::{renderer::OpenGl, Paint, Path, Baseline, Align};

pub type Canvas = femtovg::Canvas<OpenGl>;

pub trait View: 'static + Sized {
    fn body<'a>(&mut self, cx: &'a mut Context) {}
    fn build<'a>(mut self, cx: &'a mut Context) -> Handle<Self> {
        let id = if let Some(id) = cx.tree.get_child(cx.current, cx.count) {
            let prev = cx.current;
            cx.current = id;
            let prev_count = cx.count;
            cx.count = 0;
            self.body(cx);
            cx.current = prev;
            cx.count = prev_count;
            id
        } else {
            let id = cx.entity_manager.create();
            cx.tree.add(id, cx.current);
            cx.cache.add(id);
            let prev = cx.current;
            cx.current = id;
            let prev_count = cx.count;
            cx.count = 0;
            self.body(cx);
            cx.current = prev;
            cx.count = prev_count;
            cx.views.insert(id, Box::new(self));
            id  
        };

        cx.count += 1;

        Handle {
            entity: id,
            style: cx.style.clone(),
            p: Default::default(),
        }
    }
    fn debug(&self, entity: Entity) -> String {
        "".to_string()
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {

    }

    fn draw(&self, cx: &Context, canvas: &mut Canvas) {
        //println!("{}", debug(&mut context, entity));
        let entity = cx.current;
        let bounds = cx.cache.get_bounds(entity);
        let mut path = Path::new();
        path.rect(bounds.x, bounds.y, bounds.w, bounds.h);

        let background_color: femtovg::Color = cx.style.borrow_mut().background_color.get(entity).cloned().unwrap_or_default().into();
        canvas.fill_path(&mut path, Paint::color(background_color));
        
        if let Some(text) = cx.style.borrow().text.get(entity) {
            let mut paint = Paint::color(femtovg::Color::black());
            paint.set_font(&cx.fonts);
            paint.set_text_align(Align::Center);
            paint.set_text_baseline(Baseline::Middle);
            canvas.fill_text(bounds.x + bounds.w / 2.0, bounds.y + bounds.h / 2.0, text, paint);
        }
    }
    
}

impl<T: View> ViewHandler for T 
where
    T: std::marker::Sized + View + 'static
{
    fn debug(&self, entity: Entity) -> String {
        <T as View>::debug(self, entity)
    }

    fn body(&mut self, cx: &mut Context) {
        <T as View>::body(self, cx);
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        <T as View>::event(self, cx, event);
    }

    fn draw(&self, cx: &Context, canvas: &mut Canvas) {
        <T as View>::draw(self, cx, canvas);
    }
}