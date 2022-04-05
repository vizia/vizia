use crate::{Context, Handle, Res, View};

pub struct Image {}

impl Image {
    pub fn new<T: ToString>(cx: &mut Context, img: impl Res<T>) -> Handle<'_, Self> {
        Self {}.build(cx, |_| {}).image(img)
    }
}

impl View for Image {
    fn element(&self) -> Option<String> {
        Some("image".to_owned())
    }
}
