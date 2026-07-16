//! Resource management for fonts, themes, images, and translations.

mod file_resource_loader;
mod image_id;
mod url_resource_loader;

pub use file_resource_loader::FileResourceLoader;
pub use image_id::ImageId;
#[cfg(feature = "url-loader")]
pub use url_resource_loader::UrlResourceLoader;
use vizia_id::IdManager;
use vizia_reactive::{Signal, SignalGet, SignalUpdate};

use crate::context::ResourceContext;
use crate::entity::Entity;
use crate::prelude::IntoCssStr;
// use crate::view::Canvas;
use chrono::{DateTime, Utc};
use fluent_bundle::types::{FluentNumber, FluentNumberOptions};
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use hashbrown::{HashMap, HashSet};
use std::fmt;
use unic_langid::LanguageIdentifier;

/// Error type for translation operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslationError {
    /// FTL file syntax is invalid.
    InvalidFtl(String),
    /// Failed to add resource to translation bundle.
    BundleError(String),
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslationError::InvalidFtl(msg) => write!(f, "Invalid FTL syntax: {}", msg),
            TranslationError::BundleError(msg) => {
                write!(f, "Failed to add to translation bundle: {}", msg)
            }
        }
    }
}

impl std::error::Error for TranslationError {}

fn fluent_number<'a>(positional: &[FluentValue<'a>], named: &FluentArgs) -> FluentValue<'a> {
    let Some(first) = positional.first() else {
        return FluentValue::Error;
    };

    let mut number = match first {
        FluentValue::Number(num) => num.clone(),
        FluentValue::String(value) => value
            .parse::<FluentNumber>()
            .unwrap_or_else(|_| FluentNumber::new(0.0, FluentNumberOptions::default())),
        _ => return FluentValue::Error,
    };

    number.options.merge(named);
    FluentValue::Number(number)
}

fn style_str(args: &FluentArgs, key: &str) -> Option<String> {
    match args.get(key) {
        Some(FluentValue::String(value)) => Some(value.to_string()),
        _ => None,
    }
}

fn datetime_format_pattern(args: &FluentArgs) -> String {
    let weekday = match style_str(args, "weekday").as_deref() {
        Some("long") => Some("%A"),
        Some("short") => Some("%a"),
        _ => None,
    };

    let month = match style_str(args, "month").as_deref() {
        Some("long") => Some("%B"),
        Some("short") => Some("%b"),
        Some("2-digit") => Some("%m"),
        Some("numeric") => Some("%-m"),
        _ => None,
    };

    let day = match style_str(args, "day").as_deref() {
        Some("2-digit") => Some("%d"),
        Some("numeric") => Some("%-d"),
        _ => None,
    };

    let year = match style_str(args, "year").as_deref() {
        Some("2-digit") => Some("%y"),
        Some("numeric") => Some("%Y"),
        _ => None,
    };

    let hour = match style_str(args, "hour").as_deref() {
        Some("2-digit") => Some("%H"),
        Some("numeric") => Some("%-H"),
        _ => None,
    };

    let minute = match style_str(args, "minute").as_deref() {
        Some("2-digit") => Some("%M"),
        Some("numeric") => Some("%-M"),
        _ => None,
    };

    let mut date_parts = Vec::new();
    if let Some(part) = weekday {
        date_parts.push(part);
    }
    if let Some(part) = month {
        date_parts.push(part);
    }
    if let Some(part) = day {
        date_parts.push(part);
    }
    if let Some(part) = year {
        date_parts.push(part);
    }

    let mut pattern = date_parts.join(" ");
    if hour.is_some() || minute.is_some() {
        if !pattern.is_empty() {
            pattern.push(' ');
        }
        let mut time_parts = Vec::new();
        if let Some(part) = hour {
            time_parts.push(part);
        }
        if let Some(part) = minute {
            time_parts.push(part);
        }
        pattern.push_str(&time_parts.join(":"));
    }

    if pattern.is_empty() { "%Y-%m-%d %H:%M:%S".to_string() } else { pattern }
}

fn fluent_datetime<'a>(positional: &[FluentValue<'a>], named: &FluentArgs) -> FluentValue<'a> {
    let Some(first) = positional.first() else {
        return FluentValue::Error;
    };

    let millis = match first {
        FluentValue::Number(num) => num.value as i64,
        FluentValue::String(value) => value.parse::<i64>().unwrap_or_default(),
        _ => return FluentValue::Error,
    };

    let Some(dt) = DateTime::<Utc>::from_timestamp_millis(millis) else {
        return FluentValue::Error;
    };

    let pattern = datetime_format_pattern(named);
    FluentValue::String(dt.format(&pattern).to_string().into())
}

fn make_bundle(lang: LanguageIdentifier) -> FluentBundle<FluentResource> {
    let mut bundle = FluentBundle::new(vec![lang]);

    bundle.add_function("NUMBER", fluent_number).expect("Failed to register NUMBER function");
    bundle.add_function("DATETIME", fluent_datetime).expect("Failed to register DATETIME function");

    bundle
}

/// Structured diagnostics emitted by localization while resolving messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LocalizationIssue {
    /// A message key was not found in any fallback bundle.
    MissingMessage { key: String, requested_locale: String },
    /// A message attribute was not found in any fallback bundle.
    MissingAttribute { key: String, attribute: String, requested_locale: String },
    /// Fluent formatting reported errors while resolving a message.
    FormatError { key: String, locale: String, details: String },
}

impl fmt::Display for LocalizationIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocalizationIssue::MissingMessage { key, requested_locale } => {
                write!(f, "Missing localized message '{}' for locale '{}'.", key, requested_locale)
            }
            LocalizationIssue::MissingAttribute { key, attribute, requested_locale } => write!(
                f,
                "Missing localized attribute '{}.{}' for locale '{}'.",
                key, attribute, requested_locale
            ),
            LocalizationIssue::FormatError { key, locale, details } => {
                write!(f, "Formatting error for key '{}' in locale '{}': {}", key, locale, details)
            }
        }
    }
}

/// Request to load an image resource.
#[derive(Debug, Clone)]
pub struct ImageRequest {
    /// Name used to reference the loaded image in styles and image views.
    pub name: String,
    /// Path or URL to the image resource.
    pub path: String,
    /// How long the image should be retained in memory.
    pub policy: ImageRetentionPolicy,
}

/// Request to load a font resource.
#[derive(Debug, Clone)]
pub struct FontRequest {
    /// Path or URL to the font resource.
    pub path: String,
}

/// Request to load a translation resource.
#[derive(Debug, Clone)]
pub struct TranslationRequest {
    /// Language identifier this translation file belongs to.
    pub lang: LanguageIdentifier,
    /// Path or URL to the translation resource.
    pub path: String,
}

/// Loading status of a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadingStatus {
    /// Resource has not been loaded yet.
    NotLoaded,
    /// Resource is currently loading.
    Loading,
    /// Resource has been successfully loaded.
    Loaded,
    /// Resource failed to load.
    Error,
}

/// Preferred execution strategy for handling a single resource request.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ResourceLoadExecution {
    /// Let loaders choose their default strategy.
    #[default]
    Auto,
    /// Prefer asynchronous loading.
    Async,
    /// Prefer synchronous loading.
    Sync,
}

/// Per-request options that influence how a resource is loaded.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ResourceLoadOptions {
    /// Preferred execution strategy for this request.
    pub execution: ResourceLoadExecution,
}

impl ResourceLoadOptions {
    /// Construct options with automatic execution strategy selection.
    pub const fn auto() -> Self {
        Self { execution: ResourceLoadExecution::Auto }
    }

    /// Construct options that prefer asynchronous loading.
    pub const fn asynchronous() -> Self {
        Self { execution: ResourceLoadExecution::Async }
    }

    /// Construct options that prefer synchronous loading.
    pub const fn synchronous() -> Self {
        Self { execution: ResourceLoadExecution::Sync }
    }
}

/// A resource request that loaders can handle.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ResourceRequest {
    /// Request to load an image.
    Image(ImageRequest),
    /// Request to load a font.
    Font(FontRequest),
    /// Request to load a translation file.
    Translation(TranslationRequest),
}

impl ResourceRequest {
    /// Returns the canonical resource path associated with this request.
    pub fn path(&self) -> &str {
        match self {
            ResourceRequest::Image(req) => &req.path,
            ResourceRequest::Font(req) => &req.path,
            ResourceRequest::Translation(req) => &req.path,
        }
    }
}

/// A resource request queued for dispatch, paired with loader policy.
#[derive(Debug, Clone)]
pub struct QueuedResourceRequest {
    /// Request payload describing what to load.
    pub request: ResourceRequest,
    /// Loader policy describing how to perform the load.
    pub options: ResourceLoadOptions,
}

/// Trait for loading resources asynchronously or from various sources.
///
/// Loaders are invoked in chain-of-responsibility order when a resource is requested.
/// Return `true` to indicate the request was handled (no further loaders are tried),
/// or `false` to continue to the next loader.
pub trait ResourceLoader: Send + Sync + 'static {
    /// Attempt to load a resource.
    ///
    /// `request` describes what to load, while `options` describes how to load it.
    /// Return `true` if this loader handled the request, `false` to try the next loader.
    fn load(
        &self,
        request: ResourceRequest,
        options: ResourceLoadOptions,
        cx: &mut ResourceContext,
    ) -> bool;

    /// Query the loading status of a resource path.
    ///
    /// Default implementation returns `NotLoaded` — override to track async loading progress.
    fn status(&self, _path: &str) -> LoadingStatus {
        LoadingStatus::NotLoaded
    }
}

pub(crate) enum ImageOrSvg {
    Svg(skia_safe::svg::Dom),
    Image(skia_safe::Image),
}

pub(crate) struct StoredImage {
    pub image: ImageOrSvg,
    pub retention_policy: ImageRetentionPolicy,
    pub used: bool,
    pub dirty: bool,
    pub observers: HashSet<Entity>,
}

/// An image should be stored in the resource manager.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImageRetentionPolicy {
    ///  The image should live for the entire duration of the application.
    Forever,
    /// The image should be dropped when not used for one frame.
    DropWhenUnusedForOneFrame,
    /// The image should be dropped when no views are using the image.
    DropWhenNoObservers,
}

#[doc(hidden)]
#[derive(Default)]
pub struct ResourceManager {
    pub styles: Vec<Box<dyn IntoCssStr>>,

    pub(crate) image_id_manager: IdManager<ImageId>,
    pub(crate) images: HashMap<ImageId, StoredImage>,
    pub(crate) image_ids: HashMap<String, ImageId>,
    pub(crate) image_sources: HashMap<String, String>,

    pub translations: HashMap<LanguageIdentifier, FluentBundle<FluentResource>>,

    pub language: LanguageIdentifier,

    pub(crate) resource_loaders: Vec<Box<dyn ResourceLoader>>,
    /// Resource requests waiting to be dispatched through the configured loader chain.
    pub(crate) pending_requests: Vec<QueuedResourceRequest>,
    /// Reactive status tracking per resource path.
    /// Signals allow automatic updates to Memos when status changes.
    pub(crate) loading_status: HashMap<String, Signal<LoadingStatus>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        // Get the system locale
        let locale = sys_locale::get_locale().and_then(|l| l.parse().ok()).unwrap_or_default();

        let mut image_id_manager = IdManager::new();

        // Create root id for broken image
        image_id_manager.create();

        let images = HashMap::new();

        #[cfg(feature = "url-loader")]
        // Keep file loading ahead of URL loading so local/file:// paths are resolved first.
        let resource_loaders: Vec<Box<dyn ResourceLoader>> =
            vec![Box::new(FileResourceLoader), Box::new(UrlResourceLoader::default())];

        #[cfg(not(feature = "url-loader"))]
        let resource_loaders: Vec<Box<dyn ResourceLoader>> = vec![Box::new(FileResourceLoader)];

        ResourceManager {
            image_id_manager,
            images,
            image_ids: HashMap::new(),
            image_sources: HashMap::new(),
            styles: Vec::new(),

            translations: HashMap::from([(
                LanguageIdentifier::default(),
                make_bundle(LanguageIdentifier::default()),
            )]),

            language: locale,
            resource_loaders,
            pending_requests: Vec::new(),
            loading_status: HashMap::new(),
        }
    }

    /// Registers a stable image key to source path/URL mapping.
    pub(crate) fn register_image_source(&mut self, name: String, path: String) {
        self.image_sources.insert(name, path);
    }

    /// Resolves an image key to its source path/URL when registered.
    pub(crate) fn resolve_image_source<'a>(&'a self, name_or_path: &'a str) -> &'a str {
        self.image_sources.get(name_or_path).map(String::as_str).unwrap_or(name_or_path)
    }

    pub(crate) fn report_localization_issue(&self, issue: LocalizationIssue) {
        // Localization issues are non-fatal and intended for diagnostics.
        log::warn!("{}", issue);
    }

    pub fn renegotiate_language(&mut self) {
        let available = self
            .translations
            .keys()
            .filter(|&x| x != &LanguageIdentifier::default())
            .collect::<Vec<_>>();
        let locale = sys_locale::get_locale()
            .and_then(|l| l.parse().ok())
            .unwrap_or_else(|| available.first().copied().cloned().unwrap_or_default());
        let default = LanguageIdentifier::default();
        let default_ref = &default; // ???
        let langs = fluent_langneg::negotiate::negotiate_languages(
            &[locale],
            &available,
            Some(&default_ref),
            fluent_langneg::NegotiationStrategy::Filtering,
        );
        self.language = (**langs.first().unwrap()).clone();
    }

    fn negotiate_translation_locale(&self, locale: &LanguageIdentifier) -> LanguageIdentifier {
        if self.translations.contains_key(locale) {
            return locale.clone();
        }

        let available = self
            .translations
            .keys()
            .filter(|&lang| lang != &LanguageIdentifier::default())
            .collect::<Vec<_>>();

        if available.is_empty() {
            return LanguageIdentifier::default();
        }

        // Pick a fallback from the registered translations: prefer `self.language` if it
        // is one of them, otherwise the first registered translation. `available` is
        // non-empty here (checked above), so `available.first()` is always `Some`.
        let first_available = *available.first().expect("non-empty checked above");
        let fallback =
            if available.contains(&&self.language) { &self.language } else { first_available };
        let langs = fluent_langneg::negotiate::negotiate_languages(
            &[locale],
            &available,
            Some(&fallback),
            fluent_langneg::NegotiationStrategy::Filtering,
        );

        langs.first().map(|lang| (**lang).clone()).unwrap_or_else(|| fallback.clone())
    }

    pub fn translation_locales(&self, locale: &LanguageIdentifier) -> Vec<LanguageIdentifier> {
        let mut locales = Vec::new();

        if self.translations.contains_key(locale) {
            locales.push(locale.clone());
        }

        let negotiated = self.negotiate_translation_locale(locale);
        if !locales.contains(&negotiated) {
            locales.push(negotiated);
        }

        let default = LanguageIdentifier::default();
        if !locales.contains(&default) {
            locales.push(default);
        }

        locales
    }

    pub fn add_translation(
        &mut self,
        lang: LanguageIdentifier,
        ftl: String,
    ) -> Result<(), TranslationError> {
        match fluent_bundle::FluentResource::try_new(ftl) {
            Ok(res) => {
                let bundle =
                    self.translations.entry(lang.clone()).or_insert_with(|| make_bundle(lang));
                bundle.add_resource(res).map_err(|errors| {
                    let msg = format!("{:?}", errors);
                    TranslationError::BundleError(msg)
                })?;
                self.renegotiate_language();
                Ok(())
            }
            Err((_, parse_errors)) => {
                let msg =
                    parse_errors.iter().map(|e| format!("{:?}", e)).collect::<Vec<_>>().join("; ");
                Err(TranslationError::InvalidFtl(msg))
            }
        }
    }

    pub fn current_translation(
        &self,
        locale: &LanguageIdentifier,
    ) -> &FluentBundle<FluentResource> {
        let locale = self.translation_locales(locale).into_iter().next().unwrap();
        self.translations.get(&locale).unwrap()
    }

    pub fn mark_images_unused(&mut self) {
        for (_, img) in self.images.iter_mut() {
            img.used = false;
        }
    }

    pub fn evict_unused_images(&mut self) {
        let rem = self
            .images
            .iter()
            .filter_map(|(id, img)| match img.retention_policy {
                ImageRetentionPolicy::DropWhenUnusedForOneFrame => (!img.used).then_some(*id),

                ImageRetentionPolicy::DropWhenNoObservers => {
                    img.observers.is_empty().then_some(*id)
                }

                ImageRetentionPolicy::Forever => None,
            })
            .collect::<Vec<_>>();

        for id in rem {
            self.images.remove(&id);

            // Clear path mappings and reactive loading-status entries for evicted images.
            // Otherwise a stale Loaded/Error status can prevent future reloads for the same path.
            let removed_paths = self
                .image_ids
                .iter()
                .filter_map(|(path, img)| (*img == id).then_some(path.clone()))
                .collect::<Vec<_>>();
            for path in removed_paths {
                self.image_ids.remove(&path);
                self.loading_status.remove(&path);
            }

            self.image_id_manager.destroy(id);
        }
    }

    /// Query the loading status of a resource path.
    pub fn resource_status(&self, path: &str) -> LoadingStatus {
        // First check cached status signal
        if let Some(signal) = self.loading_status.get(path) {
            return signal.get();
        }

        // If not in cache, ask each loader in the chain
        for loader in &self.resource_loaders {
            let status = loader.status(path);
            if status != LoadingStatus::NotLoaded {
                return status;
            }
        }

        LoadingStatus::NotLoaded
    }

    /// Enqueue a resource request to be handled by the resource system.
    pub(crate) fn queue_resource_request(
        &mut self,
        request: ResourceRequest,
        options: ResourceLoadOptions,
    ) {
        self.pending_requests.push(QueuedResourceRequest { request, options });
    }

    /// Drain pending resource requests for this frame.
    pub(crate) fn take_pending_resource_requests(&mut self) -> Vec<QueuedResourceRequest> {
        std::mem::take(&mut self.pending_requests)
    }

    /// Update the loading status of a resource path.
    pub(crate) fn set_resource_status(&mut self, path: impl Into<String>, status: LoadingStatus) {
        let path = path.into();
        // Get or create the signal for this path
        let signal = self.loading_status.entry(path).or_insert_with(|| Signal::new(status));
        // Update the signal - this will notify any observers (Memos, Bindings)
        signal.set_if_changed(status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;
    use hashbrown::HashSet;

    fn test_image() -> skia_safe::Image {
        skia_safe::Image::from_encoded(unsafe {
            skia_safe::Data::new_bytes(include_bytes!("../../resources/images/broken_image.png"))
        })
        .unwrap()
    }

    fn stored_image(
        policy: ImageRetentionPolicy,
        used: bool,
        observers: HashSet<Entity>,
    ) -> StoredImage {
        StoredImage {
            image: ImageOrSvg::Image(test_image()),
            retention_policy: policy,
            used,
            dirty: false,
            observers,
        }
    }

    #[test]
    fn add_translation_returns_error_for_invalid_ftl() {
        let mut manager = ResourceManager::new();

        // Invalid FTL: unclosed placeable
        let res = manager.add_translation("en-US".parse().unwrap(), "hello = { $name".to_string());

        assert!(matches!(res, Err(TranslationError::InvalidFtl(_))));
    }

    #[test]
    fn translation_locales_prefers_exact_then_default() {
        let mut manager = ResourceManager::new();

        manager.add_translation("fr".parse().unwrap(), "hello = Bonjour".to_string()).unwrap();

        let locales = manager.translation_locales(&"fr".parse().unwrap());

        assert_eq!(locales.first(), Some(&"fr".parse().unwrap()));
        assert!(locales.contains(&LanguageIdentifier::default()));
    }

    #[test]
    fn translation_locales_falls_back_to_default_when_no_locale_matches() {
        let manager = ResourceManager::new();

        let locales = manager.translation_locales(&"zz-ZZ".parse().unwrap());

        assert_eq!(locales, vec![LanguageIdentifier::default()]);
    }

    #[test]
    fn current_translation_falls_back_to_registered_bundle_when_requested_locale_missing() {
        let mut manager = ResourceManager::new();

        manager.add_translation("en-US".parse().unwrap(), "hello = Hello".to_string()).unwrap();

        let bundle = manager.current_translation(&"zz-ZZ".parse().unwrap());

        assert!(bundle.get_message("hello").is_some());
    }

    #[test]
    fn current_translation_returns_registered_bundle_for_exact_match() {
        let mut manager = ResourceManager::new();

        manager.add_translation("fr".parse().unwrap(), "hello = Bonjour".to_string()).unwrap();

        let bundle = manager.current_translation(&"fr".parse().unwrap());
        let message = bundle.get_message("hello");

        assert!(message.is_some());
    }

    #[test]
    fn current_translation_returns_empty_default_when_no_translations_registered() {
        let manager = ResourceManager::new();

        // No `add_translation` call. The only entry in `translations` is the seeded empty
        // default. A miss must not panic — it falls back to that default bundle.
        let bundle = manager.current_translation(&"zz-ZZ".parse().unwrap());

        assert!(bundle.get_message("hello").is_none());
    }

    #[test]
    fn report_localization_issue_does_not_panic() {
        let manager = ResourceManager::new();
        manager.report_localization_issue(LocalizationIssue::MissingMessage {
            key: "missing-key".to_string(),
            requested_locale: "en-US".to_string(),
        });
    }

    #[test]
    fn evict_unused_images_keeps_used_one_frame_images() {
        let mut manager = ResourceManager::new();

        let used_id = manager.image_id_manager.create();
        manager.images.insert(
            used_id,
            stored_image(ImageRetentionPolicy::DropWhenUnusedForOneFrame, true, HashSet::new()),
        );

        let unused_id = manager.image_id_manager.create();
        manager.images.insert(
            unused_id,
            stored_image(ImageRetentionPolicy::DropWhenUnusedForOneFrame, false, HashSet::new()),
        );

        manager.evict_unused_images();

        assert!(manager.images.contains_key(&used_id));
        assert!(!manager.images.contains_key(&unused_id));
    }

    #[test]
    fn evict_unused_images_clears_status_for_evicted_paths() {
        let mut manager = ResourceManager::new();

        let id = manager.image_id_manager.create();
        let path = "test://evicted-image".to_string();

        manager.images.insert(
            id,
            stored_image(ImageRetentionPolicy::DropWhenNoObservers, false, HashSet::new()),
        );
        manager.image_ids.insert(path.clone(), id);
        manager.set_resource_status(path.clone(), LoadingStatus::Loaded);

        manager.evict_unused_images();

        assert!(!manager.image_ids.contains_key(&path));
        assert_eq!(manager.resource_status(&path), LoadingStatus::NotLoaded);
    }
}
