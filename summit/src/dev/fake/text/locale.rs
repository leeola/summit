use fake::{faker::lorem, Dummy, Fake, Faker};
use rand::{seq::SliceRandom, Rng};
use std::{
    fmt::Debug,
    iter,
    ops::{RangeBounds, RangeTo},
};

pub mod en;

/// An easier to configure value for locale generating [`Dummy`]s.
//
// NIT: This makes dummy generation super verbose, need to find a way to make this more generic,
// otherwise every dummy has a match that will be awful anytime a new lang variant is added.
#[derive(Debug, Default, Clone, Copy, Dummy)]
pub enum Locale {
    #[default]
    En,
    EnBob,
}
impl LocaleText for Locale {
    fn words(&self) -> &'static [&'static str] {
        match self {
            Locale::En => en::EnLorem.words(),
            Locale::EnBob => en::EnBob.words(),
        }
    }
    fn adjectives(&self) -> &'static [&'static str] {
        match self {
            Locale::En => en::EnLorem.adjectives(),
            Locale::EnBob => en::EnBob.adjectives(),
        }
    }
    fn nouns(&self) -> &'static [&'static str] {
        match self {
            Locale::En => en::EnLorem.nouns(),
            Locale::EnBob => en::EnBob.nouns(),
        }
    }
    fn verbs(&self) -> &'static [&'static str] {
        match self {
            Locale::En => en::EnLorem.verbs(),
            Locale::EnBob => en::EnBob.verbs(),
        }
    }
    fn sentences(&self) -> &'static [&'static [&'static str]] {
        match self {
            Locale::En => en::EnLorem.sentences(),
            Locale::EnBob => en::EnBob.sentences(),
        }
    }
    fn punc(&self) -> &'static [&'static str] {
        match self {
            Locale::En => en::EnLorem.punc(),
            Locale::EnBob => en::EnBob.punc(),
        }
    }
}

pub trait LocaleText: Debug + Default + Copy {
    fn words(&self) -> &'static [&'static str];
    fn adjectives(&self) -> &'static [&'static str];
    fn nouns(&self) -> &'static [&'static str];
    fn verbs(&self) -> &'static [&'static str];
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
    fn adjectives(&self) -> &'static [&'static str] {
        self.primary
            .adjectives()
            .is_empty_then(|| self.fallback.adjectives())
    }
    fn nouns(&self) -> &'static [&'static str] {
        self.primary.nouns().is_empty_then(|| self.fallback.nouns())
    }
    fn verbs(&self) -> &'static [&'static str] {
        self.primary.verbs().is_empty_then(|| self.fallback.verbs())
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

pub struct Punc(pub Locale);
impl Dummy<Punc> for &'static str {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Punc, rng: &mut R) -> Self {
        config.0.punc().choose(rng).copied().unwrap_or(".")
    }
}

#[derive(Debug, Dummy, Clone, Copy)]
pub enum SentFrag {
    Word,
    Punc,
}

#[derive(Debug, Clone, Copy)]
pub struct Sentence<L, R>(pub L, pub R);
impl<L> Sentence<L, RangeTo<usize>> {
    /// The range used as default for this type, exposed for other similar types to keep consistency
    /// with this as a base.
    pub const DEFAULT_RANGE: RangeTo<usize> = ..6;
}
impl<L> Default for Sentence<L, RangeTo<usize>>
where
    L: Default,
{
    fn default() -> Self {
        Self(L::default(), Self::DEFAULT_RANGE)
    }
}
// TODO: Fake a `Word` (ish?) type which can include punc, markup, etc. Avoiding the requirement
// that higher level primatives like Markdown have to style over a punc every time.
//
// NIT: This should probably be a `Vec<Vec<_>>` by default, but Vec<String> is still desired in the
// common case i think. When we eventually start using lossless `Word` instead of `String` this
// could be converted to being sentences, too.
impl<L, R> Dummy<Sentence<L, R>> for Vec<String>
where
    L: LocaleText,
    R: RangeBounds<usize>,
    usize: Dummy<R>,
{
    fn dummy_with_rng<Rng: rand::Rng + ?Sized>(
        Sentence(locale, range): &Sentence<L, R>,
        rng: &mut Rng,
    ) -> Self {
        let sent_limit: usize = range.fake_with_rng(rng);
        // TODO: Branch on Sentences, probably with ratios to randomly select between the two.
        let sentences = locale.sentences();
        if !sentences.is_empty() {
            iter::from_fn(|| sentences.choose(rng))
                .take(sent_limit)
                .flat_map(|sent_words| sent_words.iter())
                .map(|word| word.to_string())
                .collect()
        } else {
            let words = locale.words();
            iter::from_fn(|| Some(3..15))
                .take(sent_limit)
                .flat_map(|iter| iter)
                .map(|_| words.choose(rng).clone())
                .map_while(|word_opt| word_opt)
                .map(|word| word.to_string())
                .collect()
        }
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
