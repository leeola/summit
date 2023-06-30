use super::LocaleText;

#[derive(Debug, Default, Clone, Copy)]
pub struct EnLocale;
impl LocaleText for EnLocale {
    fn punc(&self) -> &'static [&'static str] {
        &[".", ",", ";", ":", "!"]
    }
}
