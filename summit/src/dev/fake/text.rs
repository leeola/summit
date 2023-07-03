use self::locale::{Locale, LocaleText};
use compact_str::{format_compact, CompactString};
use fake::{faker, Dummy, Fake, Faker};
use rand::{seq::SliceRandom, Rng};

pub mod locale;
pub mod markdown;

pub struct Name(pub Locale);
impl Dummy<Name> for CompactString {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Name, rng: &mut R) -> Self {
        match config.0 {
            Locale::En => faker::name::en::Name()
                .fake_with_rng::<String, _>(rng)
                .into(),
            Locale::EnBob => faker::name::en::Name()
                .fake_with_rng::<String, _>(rng)
                .into(),
        }
    }
}

/// A user-centric style combinator,
#[derive(Debug, Dummy, Clone, Copy)]
pub enum NameJoinStyle {
    AllLowercase,
    CamelCase,
    AllLowerHyphen,
    AllLowerUnderscore,
    CamelHyphen,
    CamelUnderscore,
    /// Use any of the above casing, once per joining format.
    Random,
}
impl NameJoinStyle {
    fn fmt<S: AsRef<str>, R: rand::Rng + ?Sized>(&self, s: S, rng: &mut R) -> CompactString {
        // using a single random check first to ensure we only randomize once with no risk for
        // unexpected recursion depth if we instead called `self.fmt(..)`. A single repeated
        // `Self::random` result will unevenly distribute to one of the variants below, but that's
        // not a concern.
        let style = if let Self::Random = self {
            Faker.fake_with_rng::<Self, _>(rng)
        } else {
            *self
        };
        match style {
            Self::CamelHyphen | Self::CamelUnderscore | Self::CamelCase | Self::Random => {
                let mut c = s.as_ref().chars();
                match c.next() {
                    None => CompactString::default(),
                    Some(f) => f.to_uppercase().collect::<CompactString>() + c.as_str(),
                }
            },
            Self::AllLowerHyphen | Self::AllLowerUnderscore | Self::AllLowercase => {
                s.as_ref().to_lowercase().into()
            },
        }
    }
    pub fn join<T: AsRef<str>, R: rand::Rng + ?Sized>(
        &self,
        strs: &[T],
        rng: &mut R,
    ) -> CompactString {
        match self {
            Self::AllLowercase => strs
                .iter()
                .map(|s| s.as_ref().to_lowercase())
                .collect::<CompactString>(),
            Self::CamelCase => strs
                .iter()
                .map(|s| self.fmt(s, rng))
                .collect::<CompactString>(),
            Self::AllLowerHyphen | Self::AllLowerUnderscore => {
                let sep = matches!(self, Self::AllLowerHyphen)
                    .then(|| "-")
                    .unwrap_or("_");
                strs.iter()
                    .enumerate()
                    .map(|(i, s)| {
                        if i == 0 {
                            self.fmt(s, rng)
                        } else {
                            CompactString::new(sep) + &self.fmt(s, rng)
                        }
                    })
                    .collect::<CompactString>()
            },
            Self::CamelHyphen | Self::CamelUnderscore => {
                let sep = matches!(self, Self::AllLowerHyphen)
                    .then(|| "-")
                    .unwrap_or("_");
                strs.iter()
                    .enumerate()
                    .map(|(i, s)| {
                        if i == 0 {
                            self.fmt(s, rng)
                        } else {
                            CompactString::new(sep) + &self.fmt(s, rng)
                        }
                    })
                    .collect::<CompactString>()
            },
            Self::Random => strs
                .iter()
                .map(|s| self.fmt(s, rng))
                .collect::<CompactString>(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FediUserName(pub Locale);
impl Dummy<FediUserName> for CompactString {
    fn dummy_with_rng<R: Rng + ?Sized>(&FediUserName(locale): &FediUserName, rng: &mut R) -> Self {
        let adj = locale.adjectives().choose(rng).copied().unwrap_or("");
        let noun = locale.nouns().choose(rng).copied().unwrap_or("");
        let verb = locale.verbs().choose(rng).copied().unwrap_or("");
        // TODO: generate random number suffix, randomly existing.
        Faker
            .fake_with_rng::<NameJoinStyle, _>(rng)
            .join(&[adj, noun, verb], rng)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FediHostName(pub Locale);
impl Dummy<FediHostName> for CompactString {
    fn dummy_with_rng<R: Rng + ?Sized>(&FediHostName(locale): &FediHostName, rng: &mut R) -> Self {
        let adj = locale.adjectives().choose(rng).copied().unwrap_or("");
        let noun = locale.nouns().choose(rng).copied().unwrap_or("");
        let suffix = locale
            .verbs()
            .choose(rng)
            .map(|s| s.to_lowercase())
            .unwrap_or(String::from("example"));
        let host_name = Faker
            .fake_with_rng::<bool, _>(rng)
            .then(|| NameJoinStyle::AllLowerHyphen)
            .unwrap_or(NameJoinStyle::AllLowerUnderscore)
            .join(&[adj, noun], rng);
        format_compact!("{host_name}.{suffix}")
    }
}
