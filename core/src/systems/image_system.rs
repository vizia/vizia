use crate::context::Context;
use crate::{prelude::*, resource::ImageOrId};

pub fn image_system(cx: &mut Context) {
    for entity in cx.tree.clone().into_iter() {
        // Check if the entity is using a background image
        if let Some(background_image) = cx.style.background_image.get(entity).cloned() {
            load_image(cx, entity, &background_image);
        }

        if let Some(image_name) = cx.style.image.get(entity).cloned() {
            load_image(cx, entity, &image_name);
        }
    }
}

fn load_image(cx: &mut Context, entity: Entity, image_name: &String) {
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

fn try_load_image(cx: &mut Context, entity: Entity, image_name: &str) -> bool {
    // Check if the background image is already loaded
    if let Some(image_store) = cx.resource_manager.images.get_mut(image_name) {
        match &image_store.image {
            // Image exists and is already loaded so just add this entity as an observer
            ImageOrId::Id(_, _) => {
                // TODO: check if the image is actually the same?
                image_store.observers.insert(entity);
            }

            // Image exists but isn't loaded yet
            ImageOrId::Image(_, _) => {
                if let Some(canvas) = cx.canvases.get_mut(&Entity::root()) {
                    // This loads the image and sets the image id
                    image_store.image.id(canvas);
                    cx.need_relayout();
                    cx.need_redraw();
                }
            }
        }

        return true;
    }

    false
}
