use vizia_style::Url;

use crate::prelude::*;

pub struct Image {}

impl Image {
    pub fn new<T: ToString>(cx: &mut Context, img: impl Res<T>) -> Handle<'_, Self> {
        // TODO: Make this reactive
        let img = vec![BackgroundImage::Url(Url { url: img.get_val(cx).to_string().into() })];
        Self {}.build(cx, |_| {}).background_image(img)
    }
}

impl View for Image {
    fn element(&self) -> Option<&'static str> {
        Some("image")
    }
}
