use std::collections::HashMap;

use bytes::Bytes;
use civitai::{download, list, Status};
use vizia::prelude::*;

mod civitai;
use crate::civitai::{Id, ImageData, Size};

enum AppEvent {
    ImagesListed(Result<Vec<ImageData>, reqwest::Error>),
    ImagePoppedIn(Id),
    ImageDownloaded(Id, Result<Bytes, reqwest::Error>),
    OriginalDownloaded(Id, Result<Bytes, reqwest::Error>),
    ShowOriginal(Id),
    HideOriginal,
}

struct GalleryApp {
    images: Signal<Vec<[Id; 3]>>,
    thumbnails: Signal<HashMap<Id, (ImageData, Status)>>,
    original: Signal<Option<Id>>,
    runtime: tokio::runtime::Runtime,
    thumb_width: Signal<Units>,
    thumb_height: Signal<Units>,
    gap_10: Signal<Units>,
    align_center: Signal<Alignment>,
    original_width: Signal<Units>,
    original_height: Signal<Units>,
    stretch_one: Signal<Units>,
}

impl App for GalleryApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            images: cx.state(Vec::<[Id; 3]>::default()),
            thumbnails: cx.state(HashMap::<Id, (ImageData, Status)>::default()),
            original: cx.state(None::<Id>),
            runtime: tokio::runtime::Runtime::new().unwrap(),
            thumb_width: cx.state(Pixels(320.0)),
            thumb_height: cx.state(Pixels(410.0)),
            gap_10: cx.state(Pixels(10.0)),
            align_center: cx.state(Alignment::Center),
            original_width: cx.state(Pixels(400.0)),
            original_height: cx.state(Pixels(500.0)),
            stretch_one: cx.state(Stretch(1.0)),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        cx.add_stylesheet(include_style!("src/style.css")).expect("Failed to add stylesheet");

        let images = self.images;
        let thumbnails = self.thumbnails;
        let original = self.original;
        let thumb_width = self.thumb_width;
        let thumb_height = self.thumb_height;
        let gap_10 = self.gap_10;
        let align_center = self.align_center;
        let original_width = self.original_width;
        let original_height = self.original_height;
        let stretch_one = self.stretch_one;

        let mut c = cx.get_proxy();
        self.runtime.spawn(async move {
            let images = list().await;
            let _ = c.emit(AppEvent::ImagesListed(images));
        });

        let has_images = images.drv(cx, |v, _| !v.is_empty());

        Binding::new(cx, has_images, move |cx| {
            if *has_images.get(cx) {
                VirtualList::new(cx, images, 420.0, move |cx, _, item| {
                    HStack::new(cx, move |cx| {
                        let ids = *item.get(cx);
                        for id in ids {
                            let is_loaded = thumbnails.drv(cx, move |v, _| {
                                v.get(&id)
                                    .map(|(_, status)| *status == Status::Loaded)
                                    .unwrap_or(false)
                            });
                            HStack::new(cx, move |cx| {
                                let src = cx.state(id.0.to_string());
                                Image::new(cx, src)
                                    .on_build(move |cx| cx.emit(AppEvent::ImagePoppedIn(id)))
                                    .on_press(move |cx| cx.emit(AppEvent::ShowOriginal(id)))
                                    .toggle_class("loaded", is_loaded)
                                    .class("thumbnail")
                                    .width(thumb_width)
                                    .height(thumb_height);
                            })
                            .class("frame");
                        }
                    })
                    .alignment(align_center)
                    .gap(gap_10)
                });
            }
        });

        let show_original = original.drv(cx, |v, _| v.is_some());

        Element::new(cx)
            .on_press(|cx| cx.emit(AppEvent::HideOriginal))
            .display(show_original)
            .class("background");

        let original_src = cx.state(String::default());
        Binding::new(cx, original, {
            let original_src = original_src.clone();
            move |cx| {
                let src = original
                    .get(cx)
                    .as_ref()
                    .map_or(String::default(), |o| format!("original_{}", o.0.to_string()));
                let mut event_cx = EventContext::new(cx);
                original_src.set(&mut event_cx, src);
            }
        });

        Image::new(cx, original_src)
            .background_color(Color::red())
            .width(original_width)
            .height(original_height)
            .space(stretch_one)
            .position_type(PositionType::Absolute)
            .pointer_events(PointerEvents::None)
            .class("original")
            .toggle_class("show", show_original);

        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Gallery").inner_size((1200, 600)))
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|app_event, _| match app_event {
            AppEvent::ImagesListed(Ok(images)) => {
                let mut thumbnails = HashMap::default();
                for img in images.iter() {
                    thumbnails.insert(img.id, (img.clone(), Status::Loading));
                }
                self.thumbnails.set(cx, thumbnails);
                self.images.set(
                    cx,
                    images
                        .chunks(3)
                        .map(|items| [items[0].id, items[1].id, items[2].id])
                        .collect(),
                );
            }

            AppEvent::ImagePoppedIn(id) => {
                self.thumbnails.update(cx, |thumbnails| {
                    if let Some(tn) = thumbnails.get_mut(&id) {
                        tn.1 = Status::Loading;
                    }
                });
                if let Some(image) = self.thumbnails.get(cx).get(&id).cloned() {
                    let mut c = cx.get_proxy();
                    self.runtime.spawn(async move {
                        let image = download(image.0.url, Size::Thumbnail).await;
                        let _ = c.emit(AppEvent::ImageDownloaded(id, image));
                    });
                }
            }

            AppEvent::ImageDownloaded(id, Ok(img)) => {
                cx.add_image_encoded(
                    &id.0.to_string(),
                    &img,
                    ImageRetentionPolicy::DropWhenNoObservers,
                );
                self.thumbnails.update(cx, |thumbnails| {
                    if let Some(tn) = thumbnails.get_mut(&id) {
                        tn.1 = Status::Loaded;
                    }
                });
            }

            AppEvent::OriginalDownloaded(id, Ok(img)) => {
                cx.add_image_encoded(
                    &format!("original_{}", id.0.to_string()),
                    &img,
                    ImageRetentionPolicy::DropWhenNoObservers,
                );
                self.original.set(cx, Some(id));
            }

            AppEvent::ShowOriginal(id) => {
                if let Some(image) = self.thumbnails.get(cx).get(&id).cloned() {
                    let mut c = cx.get_proxy();
                    self.runtime.block_on(async move {
                        tokio::spawn(async move {
                            let image = download(image.0.url, Size::Original).await;
                            let _ = c.emit(AppEvent::OriginalDownloaded(id, image));
                        });
                    });
                }
            }

            AppEvent::HideOriginal => {
                self.original.set(cx, None);
            }

            _ => (),
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    GalleryApp::run()
}
