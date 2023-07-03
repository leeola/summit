use crate::dev::fake::text::locale::LocaleText;
use fake::locales::{Data, EN};

#[derive(Debug, Default, Clone, Copy)]
pub struct EnLorem;
impl LocaleText for EnLorem {
    fn words(&self) -> &'static [&'static str] {
        EN::LOREM_WORD
    }
    fn adjectives(&self) -> &'static [&'static str] {
        EN::LOREM_WORD
    }
    fn nouns(&self) -> &'static [&'static str] {
        EN::LOREM_WORD
    }
    fn verbs(&self) -> &'static [&'static str] {
        EN::LOREM_WORD
    }
    fn sentences(&self) -> &'static [&'static [&'static str]] {
        &[]
    }
    fn punc(&self) -> &'static [&'static str] {
        &[".", ",", ";", ":", "!"]
    }
}
