use std::path::{Path, PathBuf};

pub trait IntoCssStr: 'static {
    fn get_style(&self) -> Result<String, std::io::Error>;
}

impl IntoCssStr for CSS {
    fn get_style(&self) -> Result<String, std::io::Error> {
        match self {
            CSS::Path(path) => std::fs::read_to_string(path),

            CSS::String(style_string) => Ok(style_string.to_owned()),
        }
    }
}

impl IntoCssStr for &'static str {
    fn get_style(&self) -> Result<String, std::io::Error> {
        Ok(self.to_string())
    }
}

impl IntoCssStr for PathBuf {
    fn get_style(&self) -> Result<String, std::io::Error> {
        std::fs::read_to_string(self)
    }
}

impl IntoCssStr for Path {
    fn get_style(&self) -> Result<String, std::io::Error> {
        std::fs::read_to_string(self)
    }
}

pub enum CSS {
    Path(PathBuf),
    String(String),
}

impl CSS {
    pub fn from_string(style: &str) -> Self {
        Self::String(style.to_owned())
    }

    pub fn from_file(path: impl AsRef<Path>) -> Self {
        Self::Path(path.as_ref().to_owned())
    }
}

impl From<&str> for CSS {
    fn from(value: &str) -> Self {
        CSS::from_string(value)
    }
}

impl From<PathBuf> for CSS {
    fn from(value: PathBuf) -> Self {
        CSS::from_file(value)
    }
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! include_style {
    ($filename:tt) => {
        $crate::prelude::CSS::from_file(concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename))
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! include_style {
    ($filename:tt) => {
        $crate::prelude::CSS::from_string(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/",
            $filename
        )))
    };
}
