//! Resource management for fonts, themes, images, and translations.

mod image_id;

pub use image_id::ImageId;
use vizia_id::{GenerationalId, IdManager};

use crate::context::ResourceContext;
use crate::entity::Entity;
use crate::prelude::IntoCssStr;
// use crate::view::Canvas;
use fluent_bundle::{FluentBundle, FluentResource};
use hashbrown::{HashMap, HashSet};
use unic_langid::LanguageIdentifier;

pub(crate) enum ImageOrSvg {
    Svg(skia_safe::svg::Dom),
    Image(skia_safe::Image),
}

pub(crate) struct StoredImage {
    pub image: ImageOrSvg,
    pub retention_policy: ImageRetentionPolicy,
    pub used: bool,
    pub dirty: bool,
    pub observers: HashSet<Entity>,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ImageRetentionPolicy {
    Forever,
    DropWhenUnusedForOneFrame,
    DropWhenNoObservers,
}

#[doc(hidden)]
#[derive(Default)]
pub struct ResourceManager {
    pub themes: Vec<String>, // Themes are the string content stylesheets
    pub styles: Vec<Box<dyn IntoCssStr>>,

    pub(crate) image_id_manager: IdManager<ImageId>,
    pub(crate) images: HashMap<ImageId, StoredImage>,
    pub(crate) image_ids: HashMap<String, ImageId>,

    pub translations: HashMap<LanguageIdentifier, FluentBundle<FluentResource>>,

    pub language: LanguageIdentifier,

    pub image_loader: Option<Box<dyn Fn(&mut ResourceContext, &str)>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        // Get the system locale
        let locale = sys_locale::get_locale().and_then(|l| l.parse().ok()).unwrap_or_default();

        let default_image_loader: Option<Box<dyn Fn(&mut ResourceContext, &str)>> = None;

        // Disable this for now because reqwest pulls in too many dependencies.
        // let default_image_loader: Option<Box<dyn Fn(&mut ResourceContext, &str)>> =
        //     Some(Box::new(|cx: &mut ResourceContext, path: &str| {
        //         if path.starts_with("https://") {
        //             let path = path.to_string();
        //             cx.spawn(move |cx| {
        //                 let data = reqwest::blocking::get(&path).unwrap().bytes().unwrap();
        //                 cx.load_image(
        //                     path,
        //                     image::load_from_memory_with_format(
        //                         &data,
        //                         image::guess_format(&data).unwrap(),
        //                     )
        //                     .unwrap(),
        //                     ImageRetentionPolicy::DropWhenUnusedForOneFrame,
        //                 )
        //                 .unwrap();
        //             });
        //         } else {
        //             // TODO: Try to load path from file
        //         }
        //     }));

        let mut image_id_manager = IdManager::new();

        // Create root id for broken image
        image_id_manager.create();

        let mut images = HashMap::new();

        images.insert(
            ImageId::root(),
            StoredImage {
                image: ImageOrSvg::Image(
                    skia_safe::Image::from_encoded(unsafe {
                        skia_safe::Data::new_bytes(include_bytes!(
                            "../../resources/images/broken_image.png"
                        ))
                    })
                    .unwrap(),
                ),

                retention_policy: ImageRetentionPolicy::Forever,
                used: true,
                dirty: false,
                observers: HashSet::new(),
            },
        );

        ResourceManager {
            themes: Vec::new(),

            image_id_manager,
            images,
            image_ids: HashMap::new(),
            styles: Vec::new(),

            translations: HashMap::from([(
                LanguageIdentifier::default(),
                FluentBundle::new(vec![LanguageIdentifier::default()]),
            )]),

            language: locale,
            image_loader: default_image_loader,
        }
    }

    pub fn renegotiate_language(&mut self) {
        let available = self
            .translations
            .keys()
            .filter(|&x| x != &LanguageIdentifier::default())
            .collect::<Vec<_>>();
        let locale = sys_locale::get_locale()
            .and_then(|l| l.parse().ok())
            .unwrap_or_else(|| available.first().copied().cloned().unwrap_or_default());
        let default = LanguageIdentifier::default();
        let default_ref = &default; // ???
        let langs = fluent_langneg::negotiate::negotiate_languages(
            &[locale],
            &available,
            Some(&default_ref),
            fluent_langneg::NegotiationStrategy::Filtering,
        );
        self.language = (**langs.first().unwrap()).clone();
    }

    pub fn add_translation(&mut self, lang: LanguageIdentifier, ftl: String) {
        let res = fluent_bundle::FluentResource::try_new(ftl)
            .expect("Failed to parse translation as FTL");
        let bundle =
            self.translations.entry(lang.clone()).or_insert_with(|| FluentBundle::new(vec![lang]));
        bundle.add_resource(res).expect("Failed to add resource to bundle");
        self.renegotiate_language();
    }

    pub fn current_translation(
        &self,
        locale: &LanguageIdentifier,
    ) -> &FluentBundle<FluentResource> {
        if let Some(bundle) = self.translations.get(locale) {
            bundle
        } else {
            self.translations.get(&self.language).unwrap()
        }
    }

    pub fn mark_images_unused(&mut self) {
        for (_, img) in self.images.iter_mut() {
            img.used = false;
        }
    }

    pub fn evict_unused_images(&mut self) {
        let rem = self
            .images
            .iter()
            .filter_map(|(id, img)| match img.retention_policy {
                ImageRetentionPolicy::DropWhenUnusedForOneFrame => (img.used).then(|| Some(*id)),

                ImageRetentionPolicy::DropWhenNoObservers => {
                    img.observers.is_empty().then(|| Some(*id))
                }

                ImageRetentionPolicy::Forever => None,
            })
            .flatten()
            .collect::<Vec<_>>();

        for id in rem {
            self.images.remove(&id);
            self.image_ids.retain(|_, img| *img != id);
        }
    }
}
