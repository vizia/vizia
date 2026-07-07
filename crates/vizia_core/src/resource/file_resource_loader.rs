use crate::context::ResourceContext;

use super::{LoadingStatus, ResourceLoader, ResourceRequest};

/// Built-in resource loader for local file paths and `file://` URLs.
pub struct FileResourceLoader;

#[cfg(not(feature = "tokio"))]
impl ResourceLoader for FileResourceLoader {
    fn load(&self, request: ResourceRequest, cx: &mut ResourceContext) -> bool {
        match request {
            ResourceRequest::Image(req) => {
                let path = if req.path.starts_with("file://") {
                    req.path.strip_prefix("file://").unwrap_or(&req.path).to_string()
                } else {
                    req.path.clone()
                };

                // Mark as loading
                cx.resource_manager.set_resource_status(req.path.clone(), LoadingStatus::Loading);

                // Try to read the file synchronously
                if let Ok(data) = std::fs::read(&path) {
                    if let Some(image) =
                        skia_safe::Image::from_encoded(skia_safe::Data::new_copy(&data))
                    {
                        cx.load_image(req.name.clone(), image, req.policy);
                        cx.resource_manager
                            .set_resource_status(req.path.clone(), LoadingStatus::Loaded);
                        return true;
                    }
                }

                // Mark as error if loading failed
                cx.resource_manager.set_resource_status(req.path.clone(), LoadingStatus::Error);
                false
            }
            ResourceRequest::Font(req) => {
                let path = if req.path.starts_with("file://") {
                    req.path.strip_prefix("file://").unwrap_or(&req.path).to_string()
                } else {
                    req.path.clone()
                };

                // Mark as loading
                cx.resource_manager.set_resource_status(req.path.clone(), LoadingStatus::Loading);

                if let Ok(data) = std::fs::read(&path) {
                    return cx.load_font(req.path.clone(), &data);
                }

                cx.resource_manager.set_resource_status(req.path.clone(), LoadingStatus::Error);
                false
            }
            ResourceRequest::Translation(req) => {
                let path = if req.path.starts_with("file://") {
                    req.path.strip_prefix("file://").unwrap_or(&req.path).to_string()
                } else {
                    req.path.clone()
                };

                cx.resource_manager.set_resource_status(req.path.clone(), LoadingStatus::Loading);

                if let Ok(data) = std::fs::read(&path) {
                    if let Ok(ftl) = String::from_utf8(data) {
                        return cx.load_translation(req.lang, req.path.clone(), &ftl);
                    }
                }

                cx.resource_manager.set_resource_status(req.path.clone(), LoadingStatus::Error);
                false
            }
            ResourceRequest::CursorIcon(req) => {
                let path = if req.path.starts_with("file://") {
                    req.path.strip_prefix("file://").unwrap_or(&req.path).to_string()
                } else {
                    req.path.clone()
                };

                cx.resource_manager.set_resource_status(req.path.clone(), LoadingStatus::Loading);

                if let Ok(data) = std::fs::read(&path) {
                    return cx.load_cursor_icon(req.path.clone(), req.name, &data, req.hotspot);
                }

                cx.resource_manager.set_resource_status(req.path.clone(), LoadingStatus::Error);
                false
            }
        }
    }
}

#[cfg(feature = "tokio")]
impl ResourceLoader for FileResourceLoader {
    fn load(&self, request: ResourceRequest, cx: &mut ResourceContext) -> bool {
        match request {
            ResourceRequest::Image(req) => {
                let path = if req.path.starts_with("file://") {
                    req.path.strip_prefix("file://").unwrap_or(&req.path).to_string()
                } else {
                    req.path.clone()
                };

                let req_name = req.name.clone();
                let req_path = req.path.clone();
                let policy = req.policy;

                // Mark as loading before spawning task
                cx.resource_manager.set_resource_status(req_path.clone(), LoadingStatus::Loading);

                // Spawn an async task to read the file
                cx.spawn_task(
                    crate::context::Task::new(move |_| {
                        let path = path.clone();
                        async move { tokio::fs::read(&path).await }
                    })
                    .on_result(move |result, proxy| {
                        use crate::context::TaskResult;
                        if let TaskResult::Completed(data) = result {
                            let status = match proxy.load_image(req_name.clone(), &data, policy) {
                                Ok(true) => LoadingStatus::Loaded,
                                _ => LoadingStatus::Error,
                            };
                            let _ = proxy.update_resource_status(req_path.clone(), status);
                        } else {
                            let _ = proxy
                                .update_resource_status(req_path.clone(), LoadingStatus::Error);
                        }
                    }),
                );

                true
            }
            ResourceRequest::Font(req) => {
                let path = if req.path.starts_with("file://") {
                    req.path.strip_prefix("file://").unwrap_or(&req.path).to_string()
                } else {
                    req.path.clone()
                };

                let req_path = req.path.clone();

                cx.resource_manager.set_resource_status(req_path.clone(), LoadingStatus::Loading);

                cx.spawn_task(
                    crate::context::Task::new(move |_| {
                        let path = path.clone();
                        async move { tokio::fs::read(&path).await }
                    })
                    .on_result(move |result, proxy| {
                        use crate::context::TaskResult;
                        if let TaskResult::Completed(data) = result {
                            if proxy.load_font(req_path.clone(), &data).is_err() {
                                let _ = proxy
                                    .update_resource_status(req_path.clone(), LoadingStatus::Error);
                            }
                        } else {
                            let _ = proxy
                                .update_resource_status(req_path.clone(), LoadingStatus::Error);
                        }
                    }),
                );

                true
            }
            ResourceRequest::Translation(req) => {
                let path = if req.path.starts_with("file://") {
                    req.path.strip_prefix("file://").unwrap_or(&req.path).to_string()
                } else {
                    req.path.clone()
                };

                let req_path = req.path.clone();
                let lang = req.lang;

                cx.resource_manager.set_resource_status(req_path.clone(), LoadingStatus::Loading);

                cx.spawn_task(
                    crate::context::Task::new(move |_| {
                        let path = path.clone();
                        async move { tokio::fs::read(&path).await }
                    })
                    .on_result(move |result, proxy| {
                        use crate::context::TaskResult;
                        if let TaskResult::Completed(data) = result {
                            match String::from_utf8(data) {
                                Ok(ftl) => {
                                    if proxy
                                        .load_translation(lang.clone(), req_path.clone(), ftl)
                                        .is_err()
                                    {
                                        let _ = proxy.update_resource_status(
                                            req_path.clone(),
                                            LoadingStatus::Error,
                                        );
                                    }
                                }
                                Err(_) => {
                                    let _ = proxy.update_resource_status(
                                        req_path.clone(),
                                        LoadingStatus::Error,
                                    );
                                }
                            }
                        } else {
                            let _ = proxy
                                .update_resource_status(req_path.clone(), LoadingStatus::Error);
                        }
                    }),
                );

                true
            }
            ResourceRequest::CursorIcon(req) => {
                let path = if req.path.starts_with("file://") {
                    req.path.strip_prefix("file://").unwrap_or(&req.path).to_string()
                } else {
                    req.path.clone()
                };

                let req_path = req.path.clone();
                let name = req.name;
                let hotspot = req.hotspot;

                cx.resource_manager.set_resource_status(req_path.clone(), LoadingStatus::Loading);

                cx.spawn_task(
                    crate::context::Task::new(move |_| {
                        let path = path.clone();
                        async move { tokio::fs::read(&path).await }
                    })
                    .on_result(move |result, proxy| {
                        use crate::context::TaskResult;
                        if let TaskResult::Completed(data) = result {
                            if proxy
                                .load_cursor_icon(req_path.clone(), name.clone(), &data, hotspot)
                                .is_err()
                            {
                                let _ = proxy
                                    .update_resource_status(req_path.clone(), LoadingStatus::Error);
                            }
                        } else {
                            let _ = proxy
                                .update_resource_status(req_path.clone(), LoadingStatus::Error);
                        }
                    }),
                );

                true
            }
        }
    }
}
