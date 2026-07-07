use crate::context::{Context, ResourceContext};
use crate::prelude::*;
use crate::resource::{ImageRequest, LoadingStatus, ResourceRequest};
use crate::style::ImageOrGradient;
// use crate::resource::{ImageId, ImageRetentionPolicy, StoredImage};
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
    // If the image is already loaded, just register the observer
    if try_load_image(cx, entity, image_name) {
        return;
    }

    // Avoid re-issuing loads every frame while a resource is already in-flight
    // (or has already failed/succeeded), which causes status churn and flicker.
    if cx.resource_manager.resource_status(image_name) != LoadingStatus::NotLoaded {
        return;
    }

    // Image doesn't exist yet, so try each loader in the chain
    // Collect loader references as pointers to avoid borrow issues during iteration
    let loaders: Vec<*const dyn crate::resource::ResourceLoader> =
        cx.resource_manager.resource_loaders.iter().map(|loader| &**loader as *const _).collect();

    for loader_ptr in loaders {
        let request = ResourceRequest::Image(ImageRequest {
            path: image_name.to_string(),
            policy: ImageRetentionPolicy::DropWhenNoObservers,
        });

        // SAFETY: We collected pointers from valid references and are not modifying the vector during iteration
        if unsafe { (*loader_ptr).load(request, cx) } {
            // Loader handled the request, try to register as observer
            try_load_image(cx, entity, image_name);
            break;
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
