use crate::context::{Context, ResourceContext};
use crate::prelude::*;
// use crate::resource::{ImageId, ImageRetentionPolicy, StoredImage};
use crate::style::ImageOrGradient;
// use hashbrown::HashSet;

// Iterate the tree and load any images used by entities which aren't already loaded. Remove any images no longer being used.
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
                        load_image(cx, entity, name);
                    }
                    _ => {}
                }
            }
        }
    }

    cx.resource_manager.evict_unused_images();
}

fn load_image(cx: &mut ResourceContext, entity: Entity, image_name: &str) {
    // if let Some(image_id) = cx.resource_manager.image_ids.get(image_name) {}

    if !try_load_image(cx, entity, image_name) {
        // Image doesn't exists yet so call the image loader
        if let Some(callback) = cx.resource_manager.image_loader.take() {
            (callback)(cx, image_name);

            cx.resource_manager.image_loader = Some(callback);

            // Then try to load the image again
            try_load_image(cx, entity, image_name);
        }
    }
}

fn try_load_image(cx: &mut ResourceContext, entity: Entity, image_name: &str) -> bool {
    if let Some(image_id) = cx.resource_manager.image_ids.get(&image_name.to_owned()) {
        // Check if the image is already loaded
        if let Some(image_store) = cx.resource_manager.images.get_mut(image_id) {
            // match &image_store.image {
            //     // Image exists and is already loaded so just add this entity as an observer and mark image as used
            //     ::Id(_, _) => {
            //         // TODO: check if the image is actually the same?
            //         image_store.observers.insert(entity);
            //         image_store.used = true;
            //     }

            //     // Image exists but isn't loaded yet
            //     ImageOrId::Image(_, _) => {
            //         if let Some(canvas) = cx.canvases.get_mut(&Entity::root()) {
            //             // This loads the image and sets the image id
            //             image_store.image.id(canvas);
            //             image_store.used = true;
            //             cx.style.needs_relayout();
            //             cx.style.needs_redraw();
            //         }
            //     }
            // }

            image_store.observers.insert(entity);
            image_store.used = true;

            return true;
        } else {
            // Safe to unwrap because we know it exsits.
            let broken_image = cx.resource_manager.images.get_mut(&ImageId::root()).unwrap();
            // Image doesn't exist yet so load and show placeholder image
            broken_image.observers.insert(entity);
        }
    }

    false
}
