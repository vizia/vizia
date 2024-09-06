use sha2::{Digest, Sha256};
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
    pub fn new<T>(cx: &mut Context, data: impl Res<T>) -> Handle<Self>
    where
        T: AsRef<[u8]> + 'static,
    {
        Self {}.build(cx, |_| {}).bind(data, |mut handle, data| {
            let svg_data = data.get(&handle);
            let mut hasher = Sha256::default();
            hasher.update(svg_data.as_ref());
            let h = format!("{:x}", hasher.finalize());

            handle.context().load_svg(
                &h,
                svg_data.as_ref(),
                ImageRetentionPolicy::DropWhenNoObservers,
            );
            handle.background_image(format!("'{h}'").as_str()).hoverable(false);
        })
    }
}

impl View for Svg {
    fn element(&self) -> Option<&'static str> {
        Some("svg")
    }
}
