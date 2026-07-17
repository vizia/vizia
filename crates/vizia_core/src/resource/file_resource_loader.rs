use crate::context::ResourceContext;

#[cfg(feature = "tokio")]
use super::ResourceLoadExecution;
use super::{LoadingStatus, ResourceLoader, ResourceRequest};

/// Built-in resource loader for local file paths and `file://` URLs.
pub struct FileResourceLoader;

fn is_http_url(path: &str) -> bool {
    path.starts_with("http://") || path.starts_with("https://")
}

#[cfg(feature = "tokio")]
fn is_likely_svg(path: &str, data: &[u8]) -> bool {
    if path.to_ascii_lowercase().ends_with(".svg") {
        return true;
    }

    let trimmed = data
        .strip_prefix(&[0xEF, 0xBB, 0xBF])
        .unwrap_or(data)
        .iter()
        .copied()
        .skip_while(|b| b.is_ascii_whitespace())
        .collect::<Vec<u8>>();

    trimmed.starts_with(b"<svg") || trimmed.starts_with(b"<?xml")
}

#[cfg(not(feature = "tokio"))]
impl ResourceLoader for FileResourceLoader {
    fn load(
        &self,
        request: ResourceRequest,
        _options: super::ResourceLoadOptions,
        cx: &mut ResourceContext,
    ) -> bool {
        match request {
            ResourceRequest::Image(req) => {
                if is_http_url(&req.path) {
                    return false;
                }

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
                true
            }
            ResourceRequest::Font(req) => {
                if is_http_url(&req.path) {
                    return false;
                }

                let path = if req.path.starts_with("file://") {
                    req.path.strip_prefix("file://").unwrap_or(&req.path).to_string()
                } else {
                    req.path.clone()
                };

                // Mark as loading
                cx.resource_manager.set_resource_status(req.path.clone(), LoadingStatus::Loading);

                if let Ok(data) = std::fs::read(&path) {
                    cx.load_font(req.path.clone(), &data);
                    return true;
                }

                cx.resource_manager.set_resource_status(req.path.clone(), LoadingStatus::Error);
                true
            }
            ResourceRequest::Translation(req) => {
                if is_http_url(&req.path) {
                    return false;
                }

                let path = if req.path.starts_with("file://") {
                    req.path.strip_prefix("file://").unwrap_or(&req.path).to_string()
                } else {
                    req.path.clone()
                };

                cx.resource_manager.set_resource_status(req.path.clone(), LoadingStatus::Loading);

                if let Ok(data) = std::fs::read(&path) {
                    if let Ok(ftl) = String::from_utf8(data) {
                        cx.load_translation(req.lang, req.path.clone(), &ftl);
                        return true;
                    }
                }

                cx.resource_manager.set_resource_status(req.path.clone(), LoadingStatus::Error);
                true
            }
        }
    }
}

#[cfg(feature = "tokio")]
impl ResourceLoader for FileResourceLoader {
    fn load(
        &self,
        request: ResourceRequest,
        options: super::ResourceLoadOptions,
        cx: &mut ResourceContext,
    ) -> bool {
        match request {
            ResourceRequest::Image(req) => {
                if is_http_url(&req.path) {
                    return false;
                }

                let path = if req.path.starts_with("file://") {
                    req.path.strip_prefix("file://").unwrap_or(&req.path).to_string()
                } else {
                    req.path.clone()
                };

                let req_name = req.name.clone();
                let req_path = req.path.clone();
                let policy = req.policy;
                let execution = options.execution;

                if matches!(execution, ResourceLoadExecution::Sync) {
                    cx.resource_manager
                        .set_resource_status(req_path.clone(), LoadingStatus::Loading);

                    if let Ok(data) = std::fs::read(&path) {
                        if let Some(image) =
                            skia_safe::Image::from_encoded(skia_safe::Data::new_copy(&data))
                        {
                            cx.load_image(req_name, image, policy);
                            cx.resource_manager
                                .set_resource_status(req_path, LoadingStatus::Loaded);
                            return true;
                        }
                    }

                    cx.resource_manager.set_resource_status(req_path, LoadingStatus::Error);
                    return true;
                }

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
                            let loaded =
                                match proxy.load_image_encoded(req_name.clone(), &data, policy) {
                                    Ok(true) => true,
                                    Ok(false) => {
                                        if is_likely_svg(&req_path, &data) {
                                            proxy.load_svg(req_name.clone(), &data, policy).is_ok()
                                        } else {
                                            false
                                        }
                                    }
                                    Err(_) => false,
                                };

                            let _ = proxy.update_resource_status(
                                req_path.clone(),
                                if loaded { LoadingStatus::Loaded } else { LoadingStatus::Error },
                            );
                        } else {
                            let _ = proxy
                                .update_resource_status(req_path.clone(), LoadingStatus::Error);
                        }
                    }),
                );

                true
            }
            ResourceRequest::Font(req) => {
                if is_http_url(&req.path) {
                    return false;
                }

                let path = if req.path.starts_with("file://") {
                    req.path.strip_prefix("file://").unwrap_or(&req.path).to_string()
                } else {
                    req.path.clone()
                };

                let req_path = req.path.clone();
                let execution = options.execution;

                if matches!(execution, ResourceLoadExecution::Sync) {
                    cx.resource_manager
                        .set_resource_status(req_path.clone(), LoadingStatus::Loading);

                    if let Ok(data) = std::fs::read(&path) {
                        cx.load_font(req_path, &data);
                        return true;
                    }

                    cx.resource_manager.set_resource_status(req_path, LoadingStatus::Error);
                    return true;
                }

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
                if is_http_url(&req.path) {
                    return false;
                }

                let path = if req.path.starts_with("file://") {
                    req.path.strip_prefix("file://").unwrap_or(&req.path).to_string()
                } else {
                    req.path.clone()
                };

                let req_path = req.path.clone();
                let lang = req.lang;
                let execution = options.execution;

                if matches!(execution, ResourceLoadExecution::Sync) {
                    cx.resource_manager
                        .set_resource_status(req_path.clone(), LoadingStatus::Loading);

                    if let Ok(data) = std::fs::read(&path) {
                        if let Ok(ftl) = String::from_utf8(data) {
                            cx.load_translation(lang, req_path, &ftl);
                            return true;
                        }
                    }

                    cx.resource_manager.set_resource_status(req_path, LoadingStatus::Error);
                    return true;
                }

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
        }
    }
}
