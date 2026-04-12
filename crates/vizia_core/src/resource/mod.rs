//! Resource management for fonts, themes, images, and translations.

mod image_id;

pub use image_id::ImageId;
use vizia_id::{GenerationalId, IdManager};

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

    if pattern.is_empty() {
        "%Y-%m-%d %H:%M:%S".to_string()
    } else {
        pattern
    }
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
    bundle
        .add_function("DATETIME", fluent_datetime)
        .expect("Failed to register DATETIME function");

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
#[derive(Copy, Clone, PartialEq)]
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

    pub translations: HashMap<LanguageIdentifier, FluentBundle<FluentResource>>,

    pub language: LanguageIdentifier,

    pub image_loader: Option<Box<dyn Fn(&mut ResourceContext, &str)>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        // Get the system locale
        let locale = sys_locale::get_locale().and_then(|l| l.parse().ok()).unwrap_or_default();

        let default_image_loader: Option<Box<dyn Fn(&mut ResourceContext, &str)>> = None;

        // Disable this for now because reqwest pulls in too many dependencies.
        // let default_image_loader: Option<Box<dyn Fn(&mut ResourceContext, &str)>> =
        //     Some(Box::new(|cx: &mut ResourceContext, path: &str| {
        //         if path.starts_with("https://") {
        //             let path = path.to_string();
        //             cx.spawn(move |cx| {
        //                 let data = reqwest::blocking::get(&path).unwrap().bytes().unwrap();
        //                 cx.load_image(
        //                     path,
        //                     image::load_from_memory_with_format(
        //                         &data,
        //                         image::guess_format(&data).unwrap(),
        //                     )
        //                     .unwrap(),
        //                     ImageRetentionPolicy::DropWhenUnusedForOneFrame,
        //                 )
        //                 .unwrap();
        //             });
        //         } else {
        //             // TODO: Try to load path from file
        //         }
        //     }));

        let mut image_id_manager = IdManager::new();

        // Create root id for broken image
        image_id_manager.create();

        let mut images = HashMap::new();

        images.insert(
            ImageId::root(),
            StoredImage {
                image: ImageOrSvg::Image(
                    skia_safe::Image::from_encoded(unsafe {
                        skia_safe::Data::new_bytes(include_bytes!(
                            "../../resources/images/broken_image.png"
                        ))
                    })
                    .unwrap(),
                ),

                retention_policy: ImageRetentionPolicy::Forever,
                used: true,
                dirty: false,
                observers: HashSet::new(),
            },
        );

        ResourceManager {
            image_id_manager,
            images,
            image_ids: HashMap::new(),
            styles: Vec::new(),

            translations: HashMap::from([(
                LanguageIdentifier::default(),
                make_bundle(LanguageIdentifier::default()),
            )]),

            language: locale,
            image_loader: default_image_loader,
        }
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

        let default = LanguageIdentifier::default();
        let default_ref = &default;
        let langs = fluent_langneg::negotiate::negotiate_languages(
            &[locale],
            &available,
            Some(&default_ref),
            fluent_langneg::NegotiationStrategy::Filtering,
        );

        langs.first().map(|lang| (**lang).clone()).unwrap_or(default)
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
                let bundle = self
                    .translations
                    .entry(lang.clone())
                    .or_insert_with(|| make_bundle(lang));
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
                ImageRetentionPolicy::DropWhenUnusedForOneFrame => (img.used).then_some(*id),

                ImageRetentionPolicy::DropWhenNoObservers => {
                    img.observers.is_empty().then_some(*id)
                }

                ImageRetentionPolicy::Forever => None,
            })
            .collect::<Vec<_>>();

        for id in rem {
            self.images.remove(&id);
            self.image_ids.retain(|_, img| *img != id);
            self.image_id_manager.destroy(id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn current_translation_uses_default_bundle_when_requested_locale_missing() {
        let mut manager = ResourceManager::new();

        manager.add_translation("en-US".parse().unwrap(), "hello = Hello".to_string()).unwrap();

        let bundle = manager.current_translation(&"zz-ZZ".parse().unwrap());
        let message = bundle.get_message("hello");

        assert!(message.is_some());
    }

    #[test]
    fn report_localization_issue_does_not_panic() {
        let manager = ResourceManager::new();
        manager.report_localization_issue(LocalizationIssue::MissingMessage {
            key: "missing-key".to_string(),
            requested_locale: "en-US".to_string(),
        });
    }
}
