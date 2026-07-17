use crate::context::{Context, ResourceContext};
use crate::prelude::*;
use crate::style::ImageOrGradient;
// use crate::resource::{ImageId, ImageRetentionPolicy, StoredImage};
// use hashbrown::HashSet;

// Iterate the tree and load any images used by entities which aren't already loaded.
// Remove any images no longer being used.
pub(crate) fn image_system(cx: &mut Context) {
    let cx = &mut ResourceContext::new(cx);

    cx.resource_manager.mark_images_unused();

    // Iterate the tree and load any defined images that aren't already loaded
    for entity in cx.tree.into_iter() {
        // Load a background-image if the entity has one
        if let Some(background_images) = cx.style.background_image.get(entity).cloned() {
            for image in background_images.iter() {
                match image {
                    ImageOrGradient::Image(name) => {
                        associate_image(cx, entity, name);
                    }
                    _ => {}
                }
            }
        }
    }

    cx.resource_manager.evict_unused_images();
}

fn associate_image(cx: &mut ResourceContext, entity: Entity, image_name: &str) -> bool {
    if let Some(image_id) = cx.resource_manager.image_ids.get(image_name) {
        // Check if the image is already loaded
        if let Some(image_store) = cx.resource_manager.images.get_mut(image_id) {
            image_store.observers.insert(entity);
            image_store.used = true;

            return true;
        }
    }

    false
}
