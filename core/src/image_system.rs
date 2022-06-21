use crate::{prelude::*, resource::ImageOrId};

pub fn image_system(cx: &mut Context) {
    for entity in cx.tree.into_iter() {
        // Check if the entity is using a background image
        if let Some(background_image) = cx.style.background_image.get(entity) {
            // Check if the background image is already loaded
            if let Some(image_store) = cx.resource_manager.images.get_mut(background_image) {
                match &image_store.image {
                    // Image is already loaded so just add this entity as an observer
                    ImageOrId::Id(image_id, (width, height)) => {}

                    // Image is not loaded so call the image loader
                    ImageOrId::Image(img, flags) => {}
                }
            }
        }
    }
}
