

pub trait LocalizedStringKey<'a> {
    fn key(&self) -> &'a str;
}

impl<'a> LocalizedStringKey<'a> for &'a str {
    fn key(&self) -> &'a str {
        self
    }
}

impl<'a> LocalizedStringKey<'a> for &'a String {
    fn key(&self) -> &'a str {
        self.as_str()
    }
}