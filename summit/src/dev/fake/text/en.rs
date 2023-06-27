use super::LocaleText;

pub struct EnLocale;
impl LocaleText for EnLocale {
    fn punc(&self) -> &'static [&'static str] {
        &[".", ",", ";", ":", "!"]
    }
}
