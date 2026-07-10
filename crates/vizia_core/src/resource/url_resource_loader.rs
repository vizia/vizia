#[cfg(feature = "url-loader")]
use crate::context::ResourceContext;

#[cfg(feature = "url-loader")]
use super::{LoadingStatus, ResourceLoader, ResourceRequest};
#[cfg(all(feature = "url-loader", feature = "tokio"))]
use super::ResourceLoadExecution;

/// Built-in resource loader for HTTP(S) URLs (requires `url-loader` feature).
#[cfg(feature = "url-loader")]
pub struct UrlResourceLoader {
    #[cfg(feature = "tokio")]
    async_client: reqwest::Client,
    blocking_client: reqwest::blocking::Client,
}

#[cfg(feature = "url-loader")]
impl Default for UrlResourceLoader {
    fn default() -> Self {
        Self {
            #[cfg(feature = "tokio")]
            async_client: reqwest::Client::new(),
            blocking_client: reqwest::blocking::Client::new(),
        }
    }
}

#[cfg(feature = "url-loader")]
fn is_likely_svg(path: &str, data: &[u8]) -> bool {
    if path.to_ascii_lowercase().ends_with(".svg") {
        return true;
    }

    // Allow leading whitespace/BOM before checking for an SVG tag.
    let trimmed = data
        .strip_prefix(&[0xEF, 0xBB, 0xBF])
        .unwrap_or(data)
        .iter()
        .copied()
        .skip_while(|b| b.is_ascii_whitespace())
        .collect::<Vec<u8>>();

    trimmed.starts_with(b"<svg") || trimmed.starts_with(b"<?xml")
}

#[cfg(all(feature = "url-loader", not(feature = "tokio")))]
impl ResourceLoader for UrlResourceLoader {
    fn load(
        &self,
        request: ResourceRequest,
        _options: super::ResourceLoadOptions,
        cx: &mut ResourceContext,
    ) -> bool {
        match request {
            ResourceRequest::Image(req) => {
                if req.path.starts_with("http://") || req.path.starts_with("https://") {
                    let path = req.path.clone();
                    let name = req.name.clone();
                    let policy = req.policy;
                    let client = self.blocking_client.clone();

                    // Mark as loading before spawning
                    cx.resource_manager.set_resource_status(path.clone(), LoadingStatus::Loading);

                    cx.spawn(move |proxy| match client.get(&path).send() {
                        Ok(response) => match response.bytes() {
                            Ok(data) => {
                                let loaded =
                                    match proxy.load_image_encoded(name.clone(), &data, policy) {
                                        Ok(true) => true,
                                        Ok(false) => {
                                            if is_likely_svg(&path, &data) {
                                                proxy.load_svg(name.clone(), &data, policy).is_ok()
                                            } else {
                                                false
                                            }
                                        }
                                        Err(_) => false,
                                    };

                                let _ = proxy.update_resource_status(
                                    path,
                                    if loaded {
                                        LoadingStatus::Loaded
                                    } else {
                                        LoadingStatus::Error
                                    },
                                );
                            }
                            Err(_) => {
                                let _ = proxy.update_resource_status(path, LoadingStatus::Error);
                            }
                        },
                        Err(_) => {
                            let _ = proxy.update_resource_status(path, LoadingStatus::Error);
                        }
                    });
                    return true;
                }
                false
            }
            ResourceRequest::Font(req) => {
                if req.path.starts_with("http://") || req.path.starts_with("https://") {
                    let path = req.path.clone();
                    let client = self.blocking_client.clone();

                    cx.resource_manager.set_resource_status(path.clone(), LoadingStatus::Loading);

                    cx.spawn(move |proxy| match client.get(&path).send() {
                        Ok(response) => match response.bytes() {
                            Ok(data) => {
                                if proxy.load_font(path.clone(), &data).is_err() {
                                    let _ =
                                        proxy.update_resource_status(path, LoadingStatus::Error);
                                }
                            }
                            Err(_) => {
                                let _ = proxy.update_resource_status(path, LoadingStatus::Error);
                            }
                        },
                        Err(_) => {
                            let _ = proxy.update_resource_status(path, LoadingStatus::Error);
                        }
                    });

                    return true;
                }

                false
            }
            ResourceRequest::Translation(req) => {
                if req.path.starts_with("http://") || req.path.starts_with("https://") {
                    let path = req.path.clone();
                    let lang = req.lang;
                    let client = self.blocking_client.clone();

                    cx.resource_manager.set_resource_status(path.clone(), LoadingStatus::Loading);

                    cx.spawn(move |proxy| match client.get(&path).send() {
                        Ok(response) => match response.bytes() {
                            Ok(data) => match String::from_utf8(data.to_vec()) {
                                Ok(ftl) => {
                                    if proxy.load_translation(lang, path.clone(), ftl).is_err() {
                                        let _ = proxy
                                            .update_resource_status(path, LoadingStatus::Error);
                                    }
                                }
                                Err(_) => {
                                    let _ =
                                        proxy.update_resource_status(path, LoadingStatus::Error);
                                }
                            },
                            Err(_) => {
                                let _ = proxy.update_resource_status(path, LoadingStatus::Error);
                            }
                        },
                        Err(_) => {
                            let _ = proxy.update_resource_status(path, LoadingStatus::Error);
                        }
                    });

                    return true;
                }

                false
            }
        }
    }
}

#[cfg(all(feature = "url-loader", feature = "tokio"))]
impl ResourceLoader for UrlResourceLoader {
    fn load(
        &self,
        request: ResourceRequest,
        options: super::ResourceLoadOptions,
        cx: &mut ResourceContext,
    ) -> bool {
        match request {
            ResourceRequest::Image(req) => {
                if req.path.starts_with("http://") || req.path.starts_with("https://") {
                    let path = req.path.clone();
                    let name = req.name.clone();
                    let path_for_result = path.clone();
                    let policy = req.policy;
                    let execution = options.execution;

                    if matches!(execution, ResourceLoadExecution::Sync) {
                        cx.resource_manager
                            .set_resource_status(path_for_result.clone(), LoadingStatus::Loading);

                        let loaded = match self.blocking_client.get(&path).send() {
                            Ok(response) => match response.bytes() {
                                Ok(data) => {
                                    if let Some(image) = skia_safe::Image::from_encoded(
                                        skia_safe::Data::new_copy(&data),
                                    ) {
                                        cx.load_image(name, image, policy);
                                        true
                                    } else {
                                        false
                                    }
                                }
                                Err(_) => false,
                            },
                            Err(_) => false,
                        };

                        cx.resource_manager.set_resource_status(
                            path_for_result,
                            if loaded { LoadingStatus::Loaded } else { LoadingStatus::Error },
                        );
                        return true;
                    }

                    // Mark as loading before spawning
                    cx.resource_manager
                        .set_resource_status(path_for_result.clone(), LoadingStatus::Loading);

                    // Spawn an async task to fetch the URL
                    let client = self.async_client.clone();
                    cx.spawn_task(
                        crate::context::Task::new(move |_| {
                            let path = path.clone();
                            let client = client.clone();
                            async move {
                                match client.get(&path).send().await {
                                    Ok(response) => match response.bytes().await {
                                        Ok(data) => Ok::<_, String>(Some(data)),
                                        Err(_) => Ok(None),
                                    },
                                    Err(_) => Ok(None),
                                }
                            }
                        })
                        .on_result(move |result, proxy| {
                            use crate::context::TaskResult;
                            match result {
                                TaskResult::Completed(Some(data)) => {
                                    let loaded =
                                        match proxy.load_image_encoded(name.clone(), &data, policy)
                                        {
                                            Ok(true) => true,
                                            Ok(false) => {
                                                if is_likely_svg(&path_for_result, &data) {
                                                    proxy
                                                        .load_svg(name.clone(), &data, policy)
                                                        .is_ok()
                                                } else {
                                                    false
                                                }
                                            }
                                            Err(_) => false,
                                        };

                                    let _ = proxy.update_resource_status(
                                        path_for_result,
                                        if loaded {
                                            LoadingStatus::Loaded
                                        } else {
                                            LoadingStatus::Error
                                        },
                                    );
                                }
                                TaskResult::Completed(None) => {
                                    let _ = proxy.update_resource_status(
                                        path_for_result,
                                        LoadingStatus::Error,
                                    );
                                }
                                _ => {}
                            }
                        }),
                    );

                    return true;
                }
                false
            }
            ResourceRequest::Font(req) => {
                if req.path.starts_with("http://") || req.path.starts_with("https://") {
                    let path = req.path.clone();
                    let path_for_result = path.clone();
                    let execution = options.execution;

                    if matches!(execution, ResourceLoadExecution::Sync) {
                        cx.resource_manager
                            .set_resource_status(path_for_result.clone(), LoadingStatus::Loading);

                        let loaded = match self.blocking_client.get(&path).send() {
                            Ok(response) => match response.bytes() {
                                Ok(data) => cx.load_font(path_for_result.clone(), &data),
                                Err(_) => false,
                            },
                            Err(_) => false,
                        };

                        if !loaded {
                            cx.resource_manager
                                .set_resource_status(path_for_result, LoadingStatus::Error);
                        }

                        return true;
                    }

                    cx.resource_manager
                        .set_resource_status(path_for_result.clone(), LoadingStatus::Loading);

                    let client = self.async_client.clone();
                    cx.spawn_task(
                        crate::context::Task::new(move |_| {
                            let path = path.clone();
                            let client = client.clone();
                            async move {
                                match client.get(&path).send().await {
                                    Ok(response) => match response.bytes().await {
                                        Ok(data) => Ok::<_, String>(Some(data)),
                                        Err(_) => Ok(None),
                                    },
                                    Err(_) => Ok(None),
                                }
                            }
                        })
                        .on_result(move |result, proxy| {
                            use crate::context::TaskResult;
                            match result {
                                TaskResult::Completed(Some(data)) => {
                                    if proxy.load_font(path_for_result.clone(), &data).is_err() {
                                        let _ = proxy.update_resource_status(
                                            path_for_result,
                                            LoadingStatus::Error,
                                        );
                                    }
                                }
                                TaskResult::Completed(None) => {
                                    let _ = proxy.update_resource_status(
                                        path_for_result,
                                        LoadingStatus::Error,
                                    );
                                }
                                _ => {}
                            }
                        }),
                    );

                    return true;
                }

                false
            }
            ResourceRequest::Translation(req) => {
                if req.path.starts_with("http://") || req.path.starts_with("https://") {
                    let path = req.path.clone();
                    let path_for_result = path.clone();
                    let lang = req.lang;
                    let execution = options.execution;

                    if matches!(execution, ResourceLoadExecution::Sync) {
                        cx.resource_manager
                            .set_resource_status(path_for_result.clone(), LoadingStatus::Loading);

                        let loaded = match self.blocking_client.get(&path).send() {
                            Ok(response) => match response.bytes() {
                                Ok(data) => match String::from_utf8(data.to_vec()) {
                                    Ok(ftl) => {
                                        cx.load_translation(lang, path_for_result.clone(), &ftl)
                                    }
                                    Err(_) => false,
                                },
                                Err(_) => false,
                            },
                            Err(_) => false,
                        };

                        if !loaded {
                            cx.resource_manager
                                .set_resource_status(path_for_result, LoadingStatus::Error);
                        }

                        return true;
                    }

                    cx.resource_manager
                        .set_resource_status(path_for_result.clone(), LoadingStatus::Loading);

                    let client = self.async_client.clone();
                    cx.spawn_task(
                        crate::context::Task::new(move |_| {
                            let path = path.clone();
                            let client = client.clone();
                            async move {
                                match client.get(&path).send().await {
                                    Ok(response) => match response.bytes().await {
                                        Ok(data) => Ok::<_, String>(Some(data)),
                                        Err(_) => Ok(None),
                                    },
                                    Err(_) => Ok(None),
                                }
                            }
                        })
                        .on_result(move |result, proxy| {
                            use crate::context::TaskResult;
                            match result {
                                TaskResult::Completed(Some(data)) => {
                                    match String::from_utf8(data.to_vec()) {
                                        Ok(ftl) => {
                                            if proxy
                                                .load_translation(
                                                    lang.clone(),
                                                    path_for_result.clone(),
                                                    ftl,
                                                )
                                                .is_err()
                                            {
                                                let _ = proxy.update_resource_status(
                                                    path_for_result.clone(),
                                                    LoadingStatus::Error,
                                                );
                                            }
                                        }
                                        Err(_) => {
                                            let _ = proxy.update_resource_status(
                                                path_for_result.clone(),
                                                LoadingStatus::Error,
                                            );
                                        }
                                    }
                                }
                                TaskResult::Completed(None) => {
                                    let _ = proxy.update_resource_status(
                                        path_for_result.clone(),
                                        LoadingStatus::Error,
                                    );
                                }
                                _ => {}
                            }
                        }),
                    );

                    return true;
                }

                false
            }
        }
    }
}
