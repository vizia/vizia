use vizia_style::Url;

use crate::prelude::*;

/// A view which presents an image.
///
/// The image URL can be static or reactive (via a Signal). Loading and error
/// UI can be composed externally with `Binding` and `LoadingStatus`.
pub struct Image {}

impl Image {
    /// Creates a new [Image] view.
    ///
    /// The `img` parameter can be a static string or a reactive signal of image paths/URLs.
    pub fn new<T>(cx: &mut Context, img: impl Res<T> + 'static) -> Handle<'_, Self>
    where
        T: ToString + Clone + 'static,
    {
        let img = img.to_signal(cx).map(|img| {
            let path = img.to_string();
            BackgroundImage::Url(Url { url: path.into() })
        });

        let handle = Self {}.build(cx, |_| {});

        handle.background_image(img)
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
        let svg_data = data.get_value(cx);
        let h = format!("{:x}", fxhash::hash64(svg_data.as_ref()));
        let mut handle = Self {}.build(cx, |_| {});
        handle.context().load_svg(&h, svg_data.as_ref(), ImageRetentionPolicy::DropWhenNoObservers);
        handle.background_image(format!("'{}'", h).as_str()).hoverable(false)
    }
}

impl View for Svg {
    fn element(&self) -> Option<&'static str> {
        Some("svg")
    }
}
