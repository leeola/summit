use fake::{faker::lorem, Dummy, Fake, Faker};
use rand::{seq::SliceRandom, Rng};
use std::{fmt::Debug, iter};

pub mod en;

// TODO: Rename `Locale`
pub trait LocaleText: Debug + Default + Copy {
    fn words(&self) -> &'static [&'static str];
    fn sentences(&self) -> &'static [&'static [&'static str]];
    fn punc(&self) -> &'static [&'static str];
    fn with_fallback<F: LocaleText>(self, fallback: F) -> FallbackLocale<Self, F> {
        FallbackLocale {
            primary: self,
            fallback,
        }
    }
}
#[derive(Debug, Default, Clone, Copy)]
pub struct FallbackLocale<Outer: LocaleText, Inner: LocaleText> {
    pub primary: Outer,
    pub fallback: Inner,
}
impl<O, I> LocaleText for FallbackLocale<O, I>
where
    O: LocaleText,
    I: LocaleText,
{
    fn words(&self) -> &'static [&'static str] {
        self.primary.words().is_empty_then(|| self.fallback.words())
    }
    fn sentences(&self) -> &'static [&'static [&'static str]] {
        self.primary
            .sentences()
            .is_empty_then(|| self.fallback.sentences())
    }
    fn punc(&self) -> &'static [&'static str] {
        self.primary.punc().is_empty_then(|| self.fallback.punc())
    }
}
trait IsEmptyThen<Rhs = Self> {
    fn is_empty_then<F: Fn() -> Self>(&self, f: F) -> Self;
}
impl<T> IsEmptyThen for &[T] {
    fn is_empty_then<F: Fn() -> Self>(&self, f: F) -> Self {
        if self.is_empty() {
            f()
        } else {
            self
        }
    }
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
    fn words(&self) -> &'static [&'static str] {
        match self {
            Locale::En => en::EnLorem.words(),
        }
    }
    fn sentences(&self) -> &'static [&'static [&'static str]] {
        match self {
            Locale::En => en::EnLorem.sentences(),
        }
    }
    fn punc(&self) -> &'static [&'static str] {
        match self {
            Locale::En => en::EnLorem.punc(),
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

#[derive(Debug, Clone, Copy)]
pub struct Sentence<L> {
    pub locale: L,
}
impl<L> Default for Sentence<L>
where
    L: Default,
{
    fn default() -> Self {
        Self {
            locale: L::default(),
        }
    }
}
// TODO: Fake a `Word` (ish?) type which can include punc, markup, etc. Avoiding the requirement
// that higher level primatives like Markdown have to style over a punc every time.
impl<L> Dummy<Sentence<L>> for Vec<String>
where
    L: LocaleText,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Sentence<L>, rng: &mut R) -> Self {
        // TODO: .. support a range lol.
        let range = 1..20;
        // TODO: Branch on Sentences, probably with ratios to randomly select between the two.
        let words = config.locale.words();
        iter::from_fn({
            let word_limit = range.fake_with_rng(rng);
            let mut i = 0;
            move || {
                if i >= word_limit {
                    return None;
                }
                i += 1;
                Some(words.choose(rng)?.to_string())
            }
        })
        .collect()
    }
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
