use fake::{faker::lorem, Dummy, Fake, Faker};
use rand::{seq::SliceRandom, Rng};
use std::fmt::Debug;

pub mod en;

pub trait LocaleText: Debug + Default + Copy {
    fn punc(&self) -> &'static [&'static str];
    fn with_fallback<F: LocaleText>(self, fallback: F) -> FallbackLocale<Self, F> {
        FallbackLocale {
            outer: self,
            inner: fallback,
        }
    }
}
#[derive(Debug, Default, Clone, Copy)]
pub struct FallbackLocale<Outer: LocaleText, Inner: LocaleText> {
    pub outer: Outer,
    pub inner: Inner,
}

/// An easier to configure value for locale generating [`Dummy`]s.
//
// NIT: This makes dummy generation super verbose, need to find a way to make this more generic,
// otherwise every dummy has a match that will be awful anytime a new lang variant is added.
#[derive(Debug, Default, Clone, Copy, Dummy)]
pub enum Locale {
    #[default]
    En,
}
impl LocaleText for Locale {
    fn punc(&self) -> &'static [&'static str] {
        match self {
            Locale::En => en::EnLocale.punc(),
        }
    }
}

pub struct Punc(pub Locale);
impl Dummy<Punc> for &'static str {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Punc, rng: &mut R) -> Self {
        config.0.punc().choose(rng).unwrap()
    }
}

#[derive(Debug, Dummy, Clone, Copy)]
pub enum SentFrag {
    Word,
    Punc,
}

pub struct Sent(pub Locale);
impl Dummy<Sent> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Sent, rng: &mut R) -> Self {
        let punc = Punc(config.0);
        let end = 15 - 1;
        let mut buf = String::new();
        let mut i = 0;
        let mut prev_frag = SentFrag::Punc;
        while i < end {
            let frag = Faker.fake_with_rng::<SentFrag, _>(rng);
            match (prev_frag, frag) {
                (SentFrag::Word, SentFrag::Punc) => {
                    buf.push_str(punc.fake_with_rng(rng));
                },
                (_, SentFrag::Word) | (_, _) => {
                    // This is pretty crude, but works for now.
                    if i > 0 {
                        buf.push_str(" ");
                    }
                    // TODO: Hook up to Locale.
                    //
                    // :grimace: The allocs..
                    let words: Vec<String> = lorem::en::Words(0..end).fake_with_rng(rng);
                    i += words.len();
                    buf += &words.join(" ");
                },
            }
            prev_frag = frag;
        }
        // TODO: Add this to punc type to the locale trait? Maybe generalized?
        buf.push_str(".");
        buf
    }
}
