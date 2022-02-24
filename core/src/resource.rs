#![allow(dead_code)]

use crate::{Canvas, Context, Entity};
use image::GenericImageView;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

pub struct StoredImage {
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
                let res = canvas
                    .create_image(femtovg::ImageSource::try_from(image.borrow()).unwrap(), *flags)
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
    pub stylesheets: Vec<String>, // Stylesheets refer to a fiel path
    pub themes: Vec<String>,      // Themes are the string content stylesheets
    pub images: HashMap<String, StoredImage>,
    pub fonts: HashMap<String, FontOrId>,

    pub image_loader: Option<Box<dyn Fn(&mut Context, &str)>>,

    count: u32,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            stylesheets: Vec::new(),
            themes: Vec::new(),
            fonts: HashMap::new(),
            images: HashMap::new(),
            image_loader: None,
            count: 0,
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
        self.images.retain(|_, img| {
            !((!img.used
                && img.retention_policy == ImageRetentionPolicy::DropWhenUnusedForOneFrame)
                || (img.observers.len() == 0
                    && img.retention_policy == ImageRetentionPolicy::DropWhenNoObservers))
        });
    }
}
