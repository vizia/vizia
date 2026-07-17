use crate::context::{Context, ResourceContext};
use crate::resource::{
    ImageRequest, ImageRetentionPolicy, LoadingStatus, ResourceLoadOptions, ResourceRequest,
};
use crate::style::ImageOrGradient;

use super::image_system;

// Process queued resource requests through configured loaders and then maintain image resources.
pub(crate) fn resource_system(cx: &mut Context) {
    {
        let resource_cx = &mut ResourceContext::new(cx);
        request_style_images(resource_cx);
        process_pending_requests(resource_cx);
    }

    image_system(cx);
}

fn request_style_images(cx: &mut ResourceContext) {
    for entity in cx.tree.into_iter() {
        if let Some(background_images) = cx.style.background_image.get(entity).cloned() {
            for image in background_images.iter() {
                if let ImageOrGradient::Image(name_or_path) = image {
                    // Borrow resource_manager immutably to check status without allocating.
                    let already_queued = {
                        let resolved = cx.resource_manager.resolve_image_source(name_or_path);
                        cx.resource_manager.resource_status(resolved) != LoadingStatus::NotLoaded
                    };
                    if already_queued {
                        continue;
                    }
                    // Only allocate when we actually need to enqueue a request.
                    let source_path =
                        cx.resource_manager.resolve_image_source(name_or_path).to_string();
                    request_image_if_not_loaded(cx, name_or_path, &source_path);
                }
            }
        }
    }
}

fn request_image_if_not_loaded(cx: &mut ResourceContext, name: &str, source_path: &str) {
    if cx.resource_manager.resource_status(source_path) != LoadingStatus::NotLoaded {
        return;
    }

    let request = ResourceRequest::Image(ImageRequest {
        name: name.to_string(),
        path: source_path.to_string(),
        policy: ImageRetentionPolicy::DropWhenNoObservers,
    });

    if !cx.request_resource(request, ResourceLoadOptions::default()) {
        cx.resource_manager.set_resource_status(source_path.to_owned(), LoadingStatus::Error);
    }
}

fn process_pending_requests(cx: &mut ResourceContext) {
    let requests = cx.resource_manager.take_pending_resource_requests();

    for pending in requests {
        let path = pending.request.path().to_owned();

        // Requests can become stale if another code path loaded the same resource first.
        if cx.resource_manager.resource_status(&path) != LoadingStatus::NotLoaded {
            continue;
        }

        if !cx.request_resource(pending.request, pending.options) {
            cx.resource_manager.set_resource_status(path, LoadingStatus::Error);
        }
    }
}
