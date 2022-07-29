use crate::context::Context;
use crate::{prelude::*, resource::ImageOrId};

pub fn image_system(cx: &mut Context) {
    cx.resource_manager.mark_images_unused();

    // Iterate the tree and load any defined images that aren't already loaded
    for entity in cx.tree.clone().into_iter() {
        // Load a background-image if the entity has one
        if let Some(background_image) = cx.style.background_image.get(entity).cloned() {
            load_image(cx, entity, &background_image);
        }

        // Load an image if the entity has one
        if let Some(image_name) = cx.style.image.get(entity).cloned() {
            load_image(cx, entity, &image_name);
        }
    }

    cx.resource_manager.evict_unused_images();
}

fn load_image(cx: &mut Context, entity: Entity, image_name: &String) {
    if !try_load_image(cx, entity, image_name) {
        // Image doesn't exists yet so call the image loader
        if let Some(callback) = cx.resource_manager.image_loader.take() {
            // TODO: Figure out a way to not recall the image loader when an image is in the process of being loaded
            // asynchronously. The issue is that the same image loader could be responsible for loading multiple images,
            // some of which aren't asynchronous. So we can't just selectively call the image loader callback.
            (callback)(cx, image_name);

            cx.resource_manager.image_loader = Some(callback);

            // Then try to load the image again
            try_load_image(cx, entity, image_name);
        }
    }
}

fn try_load_image(cx: &mut Context, entity: Entity, image_name: &str) -> bool {
    // Check if the image is already loaded
    if let Some(image_store) = cx.resource_manager.images.get_mut(image_name) {
        match &image_store.image {
            // Image exists and is already loaded so just add this entity as an observer and mark image as used
            ImageOrId::Id(_, _) => {
                // TODO: check if the image is actually the same?
                image_store.observers.insert(entity);
                image_store.used = true;
            }

            // Image exists but isn't loaded yet
            ImageOrId::Image(_, _) => {
                if let Some(canvas) = cx.canvases.get_mut(&Entity::root()) {
                    // This loads the image and sets the image id
                    image_store.image.id(canvas);
                    image_store.used = true;
                    cx.need_relayout();
                    cx.need_redraw();
                }
            }
        }

        return true;
    }

    false
}
