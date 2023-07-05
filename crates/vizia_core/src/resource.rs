//! Resource management for fonts, themes, images, and translations.

use crate::context::ResourceContext;
use crate::entity::Entity;
use crate::prelude::IntoCssStr;
use crate::view::Canvas;
use fluent_bundle::{FluentBundle, FluentResource};
use image::GenericImageView;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use unic_langid::LanguageIdentifier;

pub(crate) struct StoredImage {
    pub image: ImageOrId,
    pub retention_policy: ImageRetentionPolicy,
    pub used: bool,
    pub dirty: bool,
    pub observers: HashSet<Entity>,
}

pub(crate) enum ImageOrId {
    Image(image::DynamicImage, femtovg::ImageFlags),
    Id(femtovg::ImageId, (u32, u32)),
}

impl ImageOrId {
    pub fn id(&mut self, canvas: &mut Canvas) -> femtovg::ImageId {
        match self {
            ImageOrId::Image(image, flags) => {
                let image_ref: &image::DynamicImage = image.borrow();
                let res = canvas
                    .create_image(femtovg::ImageSource::try_from(image_ref).unwrap(), *flags)
                    .unwrap();
                *self = ImageOrId::Id(res, image.dimensions());
                res
            }
            ImageOrId::Id(i, _) => *i,
        }
    }
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
    pub(crate) images: HashMap<String, StoredImage>,
    pub translations: HashMap<LanguageIdentifier, FluentBundle<FluentResource>>,

    pub language: LanguageIdentifier,

    pub image_loader: Option<Box<dyn Fn(&mut ResourceContext, &str)>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        let locale = sys_locale::get_locale().and_then(|l| l.parse().ok()).unwrap_or_default();

        #[cfg(not(target_arch = "wasm32"))]
        let default_image_loader: Option<Box<dyn Fn(&mut ResourceContext, &str)>> = None;

        // Disable this for now because reqwest pulls in too many dependencies.
        // #[cfg(not(target_arch = "wasm32"))]
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

        #[cfg(target_arch = "wasm32")]
        let default_image_loader: Option<Box<dyn Fn(&mut ResourceContext, &str)>> = None;

        ResourceManager {
            themes: Vec::new(),
            images: HashMap::new(),
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
            available.as_slice(),
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
        self.images.retain(|_, img| match img.retention_policy {
            ImageRetentionPolicy::DropWhenUnusedForOneFrame => img.used,

            ImageRetentionPolicy::DropWhenNoObservers => !img.observers.is_empty(),

            ImageRetentionPolicy::Forever => true,
        });
    }
}
