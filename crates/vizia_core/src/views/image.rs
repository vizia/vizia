use vizia_style::Url;

use crate::prelude::*;

/// A view which presents an image.
pub struct Image {}

impl Image {
    /// Creates a new [Image] view.
    pub fn new<T: ToString>(cx: &mut Context, img: impl Res<T>) -> Handle<'_, Self> {
        // TODO: Make this reactive
        let img = img.get_value(cx);
        let img = BackgroundImage::Url(Url { url: img.to_string().into() });
        Self {}.build(cx, |_| {}).background_image(img)
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
        let mut handle = Self {}.build(cx, |_| {});
        let entity = handle.entity();

        data.set_or_bind(handle.context(), move |cx, data| {
            let svg_data = data.get_value(cx);
            let hash = format!("{:x}", fxhash::hash64(svg_data.as_ref()));

            cx.load_svg(&hash, svg_data.as_ref(), ImageRetentionPolicy::DropWhenNoObservers);
            cx.style.background_image.insert(entity, vec![ImageOrGradient::Image(hash)]);
            cx.needs_redraw(entity);
        });

        handle.hoverable(false)
    }
}

impl View for Svg {
    fn element(&self) -> Option<&'static str> {
        Some("svg")
    }
}
