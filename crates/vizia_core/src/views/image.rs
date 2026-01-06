use vizia_style::Url;

use crate::prelude::*;
use crate::style::Abilities;

/// A view which presents an image.
pub struct Image {}

impl Image {
    /// Creates a new [Image] view.
    ///
    /// Accepts either a plain value or a `Signal<T>` for reactive state.
    pub fn new<T: ToString + Clone + 'static>(
        cx: &mut Context,
        img: impl Res<T> + 'static,
    ) -> Handle<'_, Self> {
        let img = img.into_signal(cx);
        let initial = BackgroundImage::Url(Url { url: img.get(cx).to_string().into() });
        let background = cx.state(initial);
        Self {}.build(cx, |_| {}).background_image(background).bind(img, move |handle, img| {
            let img = BackgroundImage::Url(Url { url: img.get(&handle).to_string().into() });
            let mut event_cx = EventContext::new(handle.cx);
            background.set(&mut event_cx, img);
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
    ///
    /// Accepts either a plain value or a `Signal<T>` for reactive state.
    pub fn new<T>(cx: &mut Context, data: impl Res<T> + 'static) -> Handle<Self>
    where
        T: AsRef<[u8]> + Clone + 'static,
    {
        let data = data.into_signal(cx);
        let handle = Self {}.build(cx, |_| {});
        if let Some(abilities) = handle.cx.style.abilities.get_mut(handle.entity()) {
            abilities.set(Abilities::HOVERABLE, false);
        }
        handle.bind(data, move |mut handle, data| {
            let svg_bytes = data.get(&handle).as_ref().to_vec();
            let h = format!("{:x}", fxhash::hash64(&svg_bytes));

            let entity = handle.entity();
            let cx = handle.context();
            cx.load_svg(&h, svg_bytes.as_ref(), ImageRetentionPolicy::DropWhenNoObservers);
            cx.style.background_image.insert(entity, vec![ImageOrGradient::Image(h)]);
            cx.needs_redraw(entity);
        })
    }
}

impl View for Svg {
    fn element(&self) -> Option<&'static str> {
        Some("svg")
    }
}
