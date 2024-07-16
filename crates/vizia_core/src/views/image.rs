use std::ops::Deref;

use vizia_style::Url;

use crate::prelude::*;

pub struct Image {}

impl Image {
    pub fn new<T: ToString>(cx: &mut Context, img: impl Res<T>) -> Handle<'_, Self> {
        // TODO: Make this reactive
        let img = BackgroundImage::Url(Url { url: img.get(cx).to_string().into() });
        Self {}.build(cx, |_| {}).background_image(img)
    }
}

impl View for Image {
    fn element(&self) -> Option<&'static str> {
        Some("image")
    }
}

pub struct Svg {}

impl Svg {
    pub fn new<'a, T>(cx: &'a mut Context, data: impl Res<T>) -> Handle<'a, Self>
    where
        T: AsRef<[u8]> + 'static,
    {
        Self {}.build(cx, |_| {}).bind(data, |mut handle, data| {
            let svg_data = data.get(&handle);
            let num_images = handle.context().resource_manager.images.len();
            handle.context().load_svg(
                &format!("svg{}", num_images),
                svg_data.as_ref(),
                ImageRetentionPolicy::Forever,
            );
            handle.background_image(format!("'svg{}'", num_images).as_str());
        })
    }
}

impl View for Svg {
    fn element(&self) -> Option<&'static str> {
        Some("svg")
    }
}
