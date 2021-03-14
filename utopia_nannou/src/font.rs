#[derive(Debug, Clone)]
pub enum Font {
    Default,
    Font(nannou::text::Font),
}

impl Font {
    pub fn resolve(&self) -> nannou::text::Font {
        match self {
            Font::Default => nannou::text::font::default_notosans(),
            Font::Font(font) => font.clone(),
        }
    }
}
