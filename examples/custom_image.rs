use std::cell::RefCell;

use rand::Rng;
use vizia::image::*;
use vizia::prelude::*;
use vizia::vg;
use vizia::vg::imgref::Img;
use vizia::vg::rgb::FromSlice;

pub struct CustomImageView {
    image: RefCell<Option<vg::ImageId>>,
}

impl CustomImageView {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self { image: RefCell::new(None) }.build(cx, |_| {})
    }
}

impl View for CustomImageView {
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let noise: RgbaImage = ImageBuffer::from_fn(256, 256, |_, _| {
            let pixel_value: u8 = rand::thread_rng().gen::<u8>();
            Rgba([pixel_value, pixel_value, pixel_value, 255])
        });

        let image_id = if self.image.borrow().is_none() {
            let img_src = Img::new(noise.as_rgba(), 256, 256);

            let image_id = canvas.create_image(img_src, vg::ImageFlags::empty()).unwrap();

            *self.image.borrow_mut() = Some(image_id);
            image_id
        } else {
            self.image.borrow().unwrap()
        };

        let mut path = vg::Path::new();
        path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
        let paint = vg::Paint::image(image_id, 0.0, 0.0, bounds.w, bounds.h, 0.0, 1.0);
        canvas.fill_path(&mut path, &paint);
    }
}

fn main() {
    Application::new(|cx| {
        CustomImageView::new(cx).size(Pixels(300.0));
    })
    .run();
}
