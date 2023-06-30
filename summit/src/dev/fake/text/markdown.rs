use super::locale::Locale;

#[derive(Debug, Default)]
pub struct Sent {
    pub locale: Locale,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum SentWordDirective {
    #[default]
    None,
    Italic,
    Bold,
    // InlineCode,
    // NIT: Hash and Internal will be difficult to generate, but it would be nice.
    // HashLink,
    // InternalLink,
    // ExternalLink,
}
