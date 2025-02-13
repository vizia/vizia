use vizia_style::Url;

use crate::prelude::*;

/// A view which presents an image.
pub struct Image {}

impl Image {
    /// Creates a new [Image] view.
    pub fn new<T: ToString>(cx: &mut Context, img: impl Res<T>) -> Handle<'_, Self> {
        // TODO: Make this reactive

        Self {}.build(cx, |_| {}).bind(img, |handle, img| {
            let img = BackgroundImage::Url(Url { url: img.get(&handle).to_string().into() });
            handle.background_image(img);
        })
    }
}

impl View for Image {
    fn element(&self) -> Option<&'static str> {
        Some("image")
    }
}

/// A view which presents an SVG image.
pub struct Svg {}

impl Svg {
    /// Creates a new [Svg] view.
    pub fn new<T>(cx: &mut Context, data: impl Res<T>) -> Handle<Self>
    where
        T: AsRef<[u8]> + 'static,
    {
        Self {}.build(cx, |_| {}).bind(data, |mut handle, data| {
            let svg_data = data.get(&handle);
            let h = format!("{:x}", fxhash::hash64(svg_data.as_ref()));

            handle.context().load_svg(
                &h,
                svg_data.as_ref(),
                ImageRetentionPolicy::DropWhenNoObservers,
            );
            handle.background_image(format!("'{}'", h).as_str()).hoverable(false);
        })
    }
}

impl View for Svg {
    fn element(&self) -> Option<&'static str> {
        Some("svg")
    }
}
