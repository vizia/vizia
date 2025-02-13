use std::collections::HashMap;

use bytes::Bytes;
use civitai::{download, list, Status};
use vizia::prelude::*;

mod civitai;
use crate::civitai::{Id, ImageData, Size};

#[derive(Lens)]
pub struct AppData {
    images: Vec<[Id; 3]>,
    thumbnails: HashMap<Id, (ImageData, Status)>,
    original: Option<Id>,
    runtime: tokio::runtime::Runtime,
}

impl AppData {
    pub fn new(cx: &mut Context) {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let mut c = cx.get_proxy();
        runtime.block_on(async move {
            tokio::spawn(async move {
                let images = list().await;
                let _ = c.emit(AppEvent::ImagesListed(images));
            });
        });

        Self { images: Vec::default(), thumbnails: HashMap::default(), original: None, runtime }
            .build(cx);
    }
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|app_event, _| match app_event {
            AppEvent::ImagesListed(Ok(images)) => {
                for img in images.iter() {
                    self.thumbnails.insert(img.id, (img.clone(), Status::Loading));
                }
                self.images =
                    images.chunks(3).map(|items| [items[0].id, items[1].id, items[2].id]).collect();
            }

            AppEvent::ImagePoppedIn(id) => {
                if let Some(tn) = self.thumbnails.get_mut(&id) {
                    tn.1 = Status::Loading
                }
                if let Some(image) = self.thumbnails.get(&id).cloned() {
                    let mut c = cx.get_proxy();
                    self.runtime.block_on(async move {
                        tokio::spawn(async move {
                            let image = download(image.0.url, Size::Thumbnail).await;
                            let _ = c.emit(AppEvent::ImageDownloaded(id, image));
                        });
                    });
                }
            }

            AppEvent::ImageDownloaded(id, Ok(img)) => {
                cx.add_image_encoded(
                    &id.0.to_string(),
                    &img,
                    ImageRetentionPolicy::DropWhenNoObservers,
                );
                if let Some(tn) = self.thumbnails.get_mut(&id) {
                    tn.1 = Status::Loaded
                }
            }

            AppEvent::OriginalDownloaded(id, Ok(img)) => {
                cx.add_image_encoded(
                    &format!("original_{}", id.0.to_string()),
                    &img,
                    ImageRetentionPolicy::DropWhenNoObservers,
                );
                self.original = Some(id);
            }

            AppEvent::ShowOriginal(id) => {
                if let Some(image) = self.thumbnails.get(&id).cloned() {
                    let mut c = cx.get_proxy();
                    self.runtime.block_on(async move {
                        tokio::spawn(async move {
                            let image = download(image.0.url, Size::Original).await;
                            let _ = c.emit(AppEvent::OriginalDownloaded(id, image));
                        });
                    });
                }
            }

            _ => {}
        });
    }
}

enum AppEvent {
    ImagesListed(Result<Vec<ImageData>, reqwest::Error>),
    ImagePoppedIn(Id),
    ImageDownloaded(Id, Result<Bytes, reqwest::Error>),
    OriginalDownloaded(Id, Result<Bytes, reqwest::Error>),
    ShowOriginal(Id),
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("src/style.css")).expect("Failed to add stylesheet");

        AppData::new(cx);

        Binding::new(cx, AppData::images.map(|images| !images.is_empty()), |cx, has_images| {
            if has_images.get(cx) {
                VirtualList::new(cx, AppData::images, 420.0, |cx, _, item| {
                    HStack::new(cx, |cx| {
                        for id in item.get(cx) {
                            let status = AppData::thumbnails
                                .map(move |tn| tn.get(&id).map(|(_, s)| *s).unwrap_or_default());
                            HStack::new(cx, |cx| {
                                Image::new(cx, id.0.to_string())
                                    .on_build(move |cx| cx.emit(AppEvent::ImagePoppedIn(id)))
                                    .on_press(move |cx| cx.emit(AppEvent::ShowOriginal(id)))
                                    .toggle_class(
                                        "loaded",
                                        status.map(|status| *status == Status::Loaded),
                                    )
                                    .class("thumbnail")
                                    .width(Pixels(320.0))
                                    .height(Pixels(410.0));
                            })
                            .class("frame");
                        }
                    })
                    .alignment(Alignment::Center)
                    .gap(Pixels(10.0))
                });
            }
        });

        Image::new(
            cx,
            AppData::original
                .map(|o| o.map_or(String::default(), |o| format!("original_{}", o.0.to_string()))),
        )
        .background_color(Color::red())
        .width(Pixels(400.0))
        .height(Pixels(500.0))
        .space(Stretch(1.0))
        .position_type(PositionType::Absolute)
        .pointer_events(PointerEvents::None)
        .class("original")
        .toggle_class("show", AppData::original.map(|o| o.is_some()));
    })
    .title("Gallery")
    .inner_size((1200, 600))
    .run()
}
