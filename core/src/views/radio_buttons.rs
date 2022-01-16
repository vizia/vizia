
use femtovg::{Paint, Path};
use crate::{Canvas, Context, Event, Handle, MouseButton, Units, View, WindowEvent};
use crate::style::PropGet;

pub struct RadioButton {
    on_select: Option<Box<dyn Fn(&mut Context)>>,
}

impl RadioButton {
    pub fn new(cx: &mut Context, checked: bool) -> Handle<Self> {
        Self { on_select: None }
            .build2(cx, |_| {})
            .checked(checked)
    }
}

impl View for RadioButton {
    fn element(&self) -> Option<String> {
        Some("radiobutton".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(WindowEvent::MouseDown(MouseButton::Left)) = event.message.downcast() {
            if let Some(f) = self.on_select.take() {
                (f)(cx);
                self.on_select = Some(f);
            }
        }
    }

    fn draw(&self, cx: &mut Context, canvas: &mut Canvas) {
        let entity = cx.current;
        let bounds = cx.cache.get_bounds(entity);
        let border_width =
            match cx.style.border_width.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
                _ => 0.0,
            };
        let dot_radius_x = (bounds.w / 2.0 - border_width) / 2.0;
        let dot_radius_y = (bounds.h / 2.0 - border_width) / 2.0;

        let background_color =
            cx.style.background_color.get(entity).cloned().unwrap_or_default();
        let border_color = cx.style.border_color.get(entity).cloned().unwrap_or_default();
        let font_color = cx.style.font_color.get(entity).cloned().unwrap_or_default();

        let mut path = Path::new();
        path.ellipse(
            bounds.x + bounds.w / 2.0,
            bounds.y + bounds.h / 2.0,
            bounds.w / 2.0 - border_width / 2.0,
            bounds.h / 2.0 - border_width / 2.0,
        );
        canvas.fill_path(&mut path, Paint::color(background_color.into()));
        canvas.stroke_path(&mut path, Paint::color(border_color.into()).with_line_width(border_width));

        if entity.is_checked(cx) {
            let mut path = Path::new();
            path.ellipse(
                bounds.x + bounds.w / 2.0,
                bounds.y + bounds.h / 2.0,
                dot_radius_x,
                dot_radius_y,
            );
            canvas.fill_path(&mut path, Paint::color(font_color.into()));
        }
    }
}

impl Handle<'_, RadioButton> {
    pub fn on_select<F>(self, callback: F) -> Self
        where
            F: 'static + Fn(&mut Context),
    {
        if let Some(view) = self.cx.views.get_mut(&self.entity) {
            if let Some(checkbox) = view.downcast_mut::<RadioButton>() {
                checkbox.on_select = Some(Box::new(callback));
            }
        }

        self
    }
}
