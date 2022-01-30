pub struct Enviroment {
    // Signifies whether the app should be rebuilt.
    pub needs_rebuild: bool,
    pub include_default_theme: bool,
}

impl Default for Enviroment {
    fn default() -> Self {
        Enviroment::new()
    }
}

impl Enviroment {
    pub fn new() -> Self {
        // let lang =  "en-US".parse::<LanguageIdentifier>().expect("Failed to parse locale");
        // let resolved_locales = vec![&lang];
        // let mut bundle = FluentBundle::new(resolved_locales.into_iter().cloned().collect());
        // let mut file = std::fs::File::open("examples/resources/en-US/hello.ftl").expect("No File Found");
        // let mut source: String = String::new();
        // file.read_to_string(&mut source).expect("Failed to read ftl file");
        // let resource = FluentResource::try_new(source).expect("Could not parse an FTL string.");
        // bundle
        //     .add_resource(resource)
        //     .expect("Failed to add FTL resources to the bundle.");
        Self {
            needs_rebuild: true,
            //bundle,
            include_default_theme: true,
        }
    }

    // TODO
    pub fn set_locale(&mut self, _locale: &str) {
        // TODO
        // let lang =  locale.parse::<LanguageIdentifier>().expect("Failed to parse locale");
        // let resolved_locales = vec![&lang];
        // let mut bundle = FluentBundle::new(resolved_locales.into_iter().cloned().collect());
        // let mut file = std::fs::File::open(&format!("examples/resources/{}/hello.ftl", locale)).expect("No File Found");
        // let mut source: String = String::new();
        // file.read_to_string(&mut source).expect("Failed to read ftl file");
        // let resource = FluentResource::try_new(source).expect("Could not parse an FTL string.");
        // bundle
        //     .add_resource(resource)
        //     .expect("Failed to add FTL resources to the bundle.");
        // self.bundle = bundle;
        // self.needs_rebuild = true;
    }
}

pub trait Env {
    fn ignore_default_styles(self) -> Self;
}
