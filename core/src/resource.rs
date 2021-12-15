#![allow(dead_code)]

use std::collections::HashMap;

// pub struct Image {
//     name: String,
//     pub width: u32,
//     pub height: u32,
//     pub data: Vec<u8>,
// }

// pub enum ImageOrId {
//     Image(image::DynamicImage),
//     Id(femtovg::ImageId),
// }

pub enum FontOrId {
    Font(Vec<u8>),
    Id(femtovg::FontId),
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct Resource(u32);

pub struct ResourceManager {
    //pub images: HashMap<String, Image>,
    pub stylesheets: Vec<String>, // Stylesheets refer to a fiel path
    pub themes: Vec<String>,      // Themes are the string content stylesheets
    //pub images: Vec<Image>,
    pub fonts: HashMap<String, FontOrId>,

    //pub image_ids: HashMap<Rc<()>, ImageOrId>,
    count: u32,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            //images: HashMap::new(),
            stylesheets: Vec::new(),
            themes: Vec::new(),
            //images: Vec::new(),
            //image_ids: HashMap::new(),
            count: 0,
            fonts: HashMap::new(),
        }
    }

    // TODO
    // pub(crate) fn add_image(&mut self, image: image::DynamicImage) -> Rc<()> {
    //     // self.images.push(Image {
    //     //     name: name.to_string(),
    //     //     width,
    //     //     height,
    //     //     data,
    //     // });

    //     let resource = Rc::new(());

    //     self.image_ids
    //         .insert(resource.clone(), ImageOrId::Image(image));

    //     resource.clone()
    // }

    pub(crate) fn add_font(&mut self, _name: &str, _path: &str) {}
    // pub fn add_stylesheet(&mut self, path: String) -> Result<(), std::io::Error> {

    //     let style_string = std::fs::read_to_string(path.clone())?;
    //     self.stylesheets.push(path);

    //     Ok(())
    // }

    // pub fn load_image(&mut self, name: &str, path: &str) {
    //     let img = image::open(path).expect(&format!("failed to load image: {}", path));
    //     let mut raw_data: Vec<u32> = vec![0; img.to_rgba().into_raw().len() / 4];
    //     LittleEndian::read_u32_into(img.to_rgba().into_raw().as_ref(), &mut raw_data);
    //     let image = Image {
    //         width: img.width() as usize,
    //         height: img.height() as usize,
    //         data: raw_data,
    //     };
    //     self.images.insert(name.to_string(), image);
    // }
}
