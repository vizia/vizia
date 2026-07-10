use std::collections::HashMap;

use bytes::Bytes;
use images::{Status, download, list};
use vizia::prelude::*;

mod images;
use crate::images::{Id, ImageData, Size};

pub struct AppData {
    images: Signal<Vec<Vec<Id>>>,
    thumbnails: Signal<HashMap<Id, (ImageData, Status)>>,
    original: Signal<Option<Id>>,
}

type GallerySignals =
    (Signal<Vec<Vec<Id>>>, Signal<HashMap<Id, (ImageData, Status)>>, Signal<Option<Id>>);

impl AppData {
    pub fn create(cx: &mut Context) -> GallerySignals {
        cx.add_task(Task::new(|_| async move { list().await }).on_result(|result, proxy| {
            match result {
                TaskResult::Completed(images) => {
                    let _ = proxy.emit(AppEvent::ImagesListed(Ok(images)));
                }
                TaskResult::Error(error) => {
                    let _ = proxy.emit(AppEvent::ImagesListed(Err(error)));
                }
                TaskResult::Timeout => {
                    eprintln!("Image list request timed out");
                }
                TaskResult::Cancelled => {
                    eprintln!("Image list request was cancelled");
                }
                TaskResult::Disconnected { .. } => {
                    eprintln!("Image list worker disconnected");
                }
            }
        }));

        let images = Signal::new(Vec::default());
        let thumbnails = Signal::new(HashMap::default());
        let original = Signal::new(None);

        Self { images, thumbnails, original }.build(cx);

        (images, thumbnails, original)
    }
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|app_event, _| match app_event {
            AppEvent::ImagesListed(Err(e)) => {
                eprintln!("Failed to list images: {e}");
            }

            AppEvent::ImagesListed(Ok(images)) => {
                self.thumbnails.update(|thumbnails| {
                    for img in images.iter() {
                        thumbnails.insert(img.id, (img.clone(), Status::Loading));
                    }
                });
                self.images.set(
                    images
                        .chunks(3)
                        .map(|items| items.iter().map(|item| item.id).collect())
                        .collect(),
                );
            }

            AppEvent::ImagePoppedIn(id) => {
                self.thumbnails.update(|thumbnails| {
                    if let Some(tn) = thumbnails.get_mut(&id) {
                        tn.1 = Status::Loading;
                    }
                });
                if let Some(url) = self.thumbnails.get().get(&id).map(|image| image.0.url.clone()) {
                    cx.add_task(
                        Task::new(move |_| {
                            let url = url.clone();
                            async move { download(url, Size::Thumbnail).await }
                        })
                        .on_result(move |result, proxy| match result {
                            TaskResult::Completed(image) => {
                                let _ = proxy.emit(AppEvent::ImageDownloaded(id, Ok(image)));
                            }
                            TaskResult::Error(error) => {
                                let _ = proxy.emit(AppEvent::ImageDownloaded(id, Err(error)));
                            }
                            TaskResult::Timeout => {
                                eprintln!("Thumbnail download timed out for image {}", id.0);
                            }
                            TaskResult::Cancelled => {
                                eprintln!("Thumbnail download cancelled for image {}", id.0);
                            }
                            TaskResult::Disconnected { .. } => {
                                eprintln!("Thumbnail worker disconnected for image {}", id.0);
                            }
                        }),
                    );
                }
            }

            AppEvent::ImageDownloaded(id, Err(e)) => {
                eprintln!("Failed to download image {}: {e}", id.0);
            }

            AppEvent::ImageDownloaded(id, Ok(img)) => {
                cx.add_image_encoded(
                    &id.0.to_string(),
                    &img,
                    ImageRetentionPolicy::DropWhenNoObservers,
                    None,
                );
                self.thumbnails.update(|thumbnails| {
                    if let Some(tn) = thumbnails.get_mut(&id) {
                        tn.1 = Status::Loaded;
                    }
                });
            }

            AppEvent::OriginalDownloaded(id, Ok(img)) => {
                cx.add_image_encoded(
                    &format!("original_{}", id.0),
                    &img,
                    ImageRetentionPolicy::DropWhenNoObservers,
                    None,
                );
                self.original.set(Some(id));
            }

            AppEvent::ShowOriginal(id) => {
                if let Some(url) = self.thumbnails.get().get(&id).map(|image| image.0.url.clone()) {
                    cx.add_task(
                        Task::new(move |_| {
                            let url = url.clone();
                            async move { download(url, Size::Original).await }
                        })
                        .on_result(move |result, proxy| match result {
                            TaskResult::Completed(image) => {
                                let _ = proxy.emit(AppEvent::OriginalDownloaded(id, Ok(image)));
                            }
                            TaskResult::Error(error) => {
                                let _ = proxy.emit(AppEvent::OriginalDownloaded(id, Err(error)));
                            }
                            TaskResult::Timeout => {
                                eprintln!("Original download timed out for image {}", id.0);
                            }
                            TaskResult::Cancelled => {
                                eprintln!("Original download cancelled for image {}", id.0);
                            }
                            TaskResult::Disconnected { .. } => {
                                eprintln!("Original worker disconnected for image {}", id.0);
                            }
                        }),
                    );
                }
            }

            AppEvent::HideOriginal => {
                self.original.set(None);
            }

            _ => (),
        });
    }
}

enum AppEvent {
    ImagesListed(Result<Vec<ImageData>, reqwest::Error>),
    ImagePoppedIn(Id),
    ImageDownloaded(Id, Result<Bytes, reqwest::Error>),
    OriginalDownloaded(Id, Result<Bytes, reqwest::Error>),
    ShowOriginal(Id),
    HideOriginal,
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("src/style.css")).expect("Failed to add stylesheet");

        let (images, thumbnails, original) = AppData::create(cx);

        let has_images = Memo::new(move |_| !images.get().is_empty());
        let is_original_visible = Memo::new(move |_| original.get().is_some());
        let original_image_name = Memo::new(move |_| {
            original.get().map_or(String::default(), |id| format!("original_{}", id.0))
        });

        Binding::new(cx, has_images, move |cx| {
            let has_images = has_images.get();
            if has_images {
                VirtualList::new(cx, images, 420.0, move |cx, _, item| {
                    HStack::new(cx, |cx| {
                        for id in item.get() {
                            let is_loaded = Memo::new(move |_| {
                                thumbnails
                                    .get()
                                    .get(&id)
                                    .map(|(_, status)| *status == Status::Loaded)
                                    .unwrap_or(false)
                            });
                            HStack::new(cx, |cx| {
                                Image::new(cx, id.0.to_string())
                                    .on_build(move |cx| cx.emit(AppEvent::ImagePoppedIn(id)))
                                    .on_press(move |cx| cx.emit(AppEvent::ShowOriginal(id)))
                                    .toggle_class("loaded", is_loaded)
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

        Element::new(cx)
            .on_press(|cx| cx.emit(AppEvent::HideOriginal))
            .display(is_original_visible)
            .class("background");

        Image::new(cx, original_image_name)
            .background_color(Color::red())
            .width(Pixels(400.0))
            .height(Pixels(500.0))
            .space(Stretch(1.0))
            .position_type(PositionType::Absolute)
            .pointer_events(PointerEvents::None)
            .class("original")
            .toggle_class("show", is_original_visible);
    })
    .title("Gallery")
    .inner_size((1200, 600))
    .run()
}
