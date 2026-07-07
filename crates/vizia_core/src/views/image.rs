use vizia_style::Url;

use crate::prelude::*;

/// A view which presents an image.
///
/// The image URL can be static or reactive (via a Signal). The framework
/// automatically handles loading through the ResourceLoader chain.
///
/// To track loading status, query [ResourceManager::resource_status] from
/// [ResourceContext] or via the [LoadingStatus] API.
pub struct Image {}

impl Image {
    /// Creates a new [Image] view.
    ///
    /// The `img` parameter can be a static string or a reactive signal of image paths/URLs.
    ///
    /// # Examples
    ///
    /// Static path:
    /// ```ignore
    /// Image::new(cx, "path/to/image.png")
    /// ```
    ///
    /// Reactive signal:
    /// ```ignore
    /// let url = Signal::new("https://example.com/image1.png");
    /// Image::new(cx, url)
    /// ```
    pub fn new<T: ToString>(cx: &mut Context, img: impl Res<T>) -> Handle<'_, Self> {
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
