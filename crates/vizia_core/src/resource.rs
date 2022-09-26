#![allow(dead_code)]

use crate::context::Context;
use crate::entity::Entity;
use crate::view::Canvas;
use fluent_bundle::{FluentBundle, FluentResource};
use image::GenericImageView;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use unic_langid::LanguageIdentifier;

pub(crate) struct StoredImage {
    pub image: ImageOrId,
    pub retention_policy: ImageRetentionPolicy,
    pub used: bool,
    pub dirty: bool,
    pub observers: HashSet<Entity>,
}

pub enum ImageOrId {
    Image(image::DynamicImage, femtovg::ImageFlags),
    Id(femtovg::ImageId, (u32, u32)), // need to be able to get dimensions without a canvas
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

    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ImageOrId::Image(image, _) => image.dimensions(),
            ImageOrId::Id(_, dim) => *dim,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ImageRetentionPolicy {
    Forever,
    DropWhenUnusedForOneFrame,
    DropWhenNoObservers,
}

pub enum FontOrId {
    Font(Vec<u8>),
    Id(femtovg::FontId),
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct Resource(u32);

#[derive(Default)]
pub struct ResourceManager {
    pub stylesheets: Vec<PathBuf>, // Stylesheets refer to a file path
    pub themes: Vec<String>,       // Themes are the string content stylesheets
    pub(crate) images: HashMap<String, StoredImage>,
    pub fonts: HashMap<String, FontOrId>,
    pub translations: HashMap<LanguageIdentifier, FluentBundle<FluentResource>>,
    pub language: LanguageIdentifier,

    pub image_loader: Option<Box<dyn Fn(&mut Context, &str)>>,

    count: u32,
}

impl ResourceManager {
    pub fn new() -> Self {
        let locale = sys_locale::get_locale().map(|l| l.parse().ok()).flatten().unwrap_or_default();

        ResourceManager {
            stylesheets: Vec::new(),
            themes: Vec::new(),
            fonts: HashMap::new(),
            images: HashMap::new(),
            translations: HashMap::from([(
                LanguageIdentifier::default(),
                FluentBundle::new(vec![LanguageIdentifier::default()]),
            )]),
            language: locale,
            image_loader: None,
            count: 0,
        }
    }

    fn renegotiate_language(&mut self) {
        let available = self
            .translations
            .keys()
            .filter(|&x| x != &LanguageIdentifier::default())
            .collect::<Vec<_>>();
        let locale = sys_locale::get_locale()
            .map(|l| l.parse().ok())
            .flatten()
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

    pub(crate) fn add_font(&mut self, _name: &str, _path: &str) {}
    // pub fn add_stylesheet(&mut self, path: String) -> Result<(), std::io::Error> {

    //     let style_string = std::fs::read_to_string(path.clone())?;
    //     self.stylesheets.push(path);

    //     Ok(())
    // }

    pub fn mark_images_unused(&mut self) {
        for (_, img) in self.images.iter_mut() {
            img.used = false;
        }
    }

    pub fn evict_unused_images(&mut self) {
        self.images.retain(|name, img| match img.retention_policy {
            ImageRetentionPolicy::DropWhenUnusedForOneFrame => {
                if !img.used {
                    println!("Evict image: {}", name);
                }
                img.used
            }

            ImageRetentionPolicy::DropWhenNoObservers => !img.observers.is_empty(),

            ImageRetentionPolicy::Forever => true,
        });
    }
}
