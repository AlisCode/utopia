#[derive(Debug, Clone)]
pub enum Font {
    Default,
    Font(nannou::text::Font),
}

impl Default for Font {
    fn default() -> Self {
        Font::Default
    }
}

impl Font {
    pub fn resolve(&self) -> nannou::text::Font {
        match self {
            Font::Default => nannou::text::font::default_notosans(),
            Font::Font(font) => font.clone(),
        }
    }
}
