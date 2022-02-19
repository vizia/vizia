use crate::{Binding, Context, Handle, PropSet, Res, StaticLens, View};
use morphorm::Units;

pub struct Image {}

static DUMMY: i32 = 0;

impl Image {
    pub fn new<R: ToString>(cx: &mut Context, img: impl 'static + Res<R>) -> Handle<'_, Self> {
        Self {}.build2(cx, move |cx| {
            // this binding is necessary so that if we're NOT passed a lens, then cx.current in the
            // set_or_bind call still needs to refer to a binding so add_image_observer works right
            let current = cx.current;
            Binding::new(cx, StaticLens::new(&DUMMY), move |cx, _| {
                img.set_or_bind(cx, cx.current, move |cx, _, img| {
                    let img = img.to_string();
                    let dim = cx.get_image(&img).dimensions();
                    cx.add_image_observer(&img, cx.current);
                    current.set_width(cx, Units::Pixels(dim.0 as f32));
                    current.set_height(cx, Units::Pixels(dim.1 as f32));
                    current.set_background_image(cx, img);
                });
            });
        })
    }
}

impl View for Image {
    fn element(&self) -> Option<String> {
        Some("image".to_owned())
    }
}
