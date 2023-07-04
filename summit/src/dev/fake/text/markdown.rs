use super::locale::LocaleText;
use crate::dev::fake::text::locale;
use fake::{Dummy, Fake, Faker};
use rand::Rng;
use std::{
    iter,
    ops::{Range, RangeBounds, RangeTo},
};

#[derive(Debug, Default, Clone, Copy)]
pub enum WordMarkup {
    #[default]
    None,
    ItalicStar,
    ItalicUnderscore,
    BoldStar,
    BoldUnderscore,
    BoldItalicStar,
    BoldItalicUnderscore,
    // InlineCode,
    // NIT: Hash and Internal will be difficult to generate, but it would be nice.
    // HashLink,
    // InternalLink,
    // ExternalLink,
}
impl WordMarkup {
    pub fn format_string(&self, s: &mut String) {
        match self {
            WordMarkup::None => return,
            WordMarkup::ItalicStar => *s = format!("*{}*", &s),
            WordMarkup::ItalicUnderscore => *s = format!("_{}_", &s),
            WordMarkup::BoldStar => *s = format!("**{}**", &s),
            WordMarkup::BoldUnderscore => *s = format!("__{}__", &s),
            WordMarkup::BoldItalicStar => *s = format!("***{}***", &s),
            WordMarkup::BoldItalicUnderscore => *s = format!("___{}___", &s),
        }
    }
}
impl Dummy<Faker> for WordMarkup {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        MarkupFreq(1.0).fake_with_rng(rng)
    }
}
/// Fake a markup percentage with markup usage frequency adjusted by the given multiplier.  
///
/// Ie 1.0 is the baseline "normal frequency", 0.9 is 90%, 1.10 110%, and so forth.
#[derive(Debug, Clone, Copy)]
pub struct MarkupFreq(pub f32);
impl MarkupFreq {
    /// Drop the frequencies by a fixed amount, keeping the percentages consistence for ease of dev,
    /// but allowing us to make them less spammy.
    const GLOBAL_REDUCTION: f32 = 0.25;
}
impl MarkupFreq {
    /// The frequency cap for [`WordMarkup`], ensuring we don't truncate RNG variants and mess
    /// up distribution mistakingly. The divisor should match the largest rng range used in the
    /// Dummy impl.
    const FREQ_CAP_WORD_MARKUP: f32 = 100. / 15.;
}
impl Default for MarkupFreq {
    fn default() -> Self {
        Self(1.)
    }
}
impl Dummy<MarkupFreq> for WordMarkup {
    fn dummy_with_rng<R: Rng + ?Sized>(&MarkupFreq(freq): &MarkupFreq, rng: &mut R) -> Self {
        let freq = freq.min(MarkupFreq::FREQ_CAP_WORD_MARKUP);
        match (0.0..1.0).fake_with_rng::<f32, _>(rng) {
            i if i < 0.025 * MarkupFreq::GLOBAL_REDUCTION * freq => Self::ItalicStar,
            i if i < 0.050 * MarkupFreq::GLOBAL_REDUCTION * freq => Self::ItalicUnderscore,
            i if i < 0.075 * MarkupFreq::GLOBAL_REDUCTION * freq => Self::BoldStar,
            i if i < 0.100 * MarkupFreq::GLOBAL_REDUCTION * freq => Self::BoldUnderscore,
            i if i < 0.125 * MarkupFreq::GLOBAL_REDUCTION * freq => Self::BoldItalicStar,
            i if i < 0.150 * MarkupFreq::GLOBAL_REDUCTION * freq => Self::BoldItalicUnderscore,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sentence<L, R>(pub L, pub R);
impl<L> Sentence<L, RangeTo<usize>> {
    pub fn locale(locale: L) -> Self {
        Self(locale, locale::Sentence::<L, _>::DEFAULT_RANGE)
    }
}
impl<L> Default for Sentence<L, RangeTo<usize>>
where
    L: Default,
{
    fn default() -> Self {
        Self(L::default(), locale::Sentence::<L, _>::DEFAULT_RANGE)
    }
}
// TODO: Fake a `Word` (ish?) type which can include punc, markup, etc. Avoiding the requirement
// that higher level primatives like Markdown have to style over a punc every time.
impl<L, R> Dummy<Sentence<L, R>> for Vec<String>
where
    L: LocaleText,
    R: RangeBounds<usize> + Clone,
    usize: Dummy<R>,
{
    fn dummy_with_rng<Rng: rand::Rng + ?Sized>(config: &Sentence<L, R>, rng: &mut Rng) -> Self {
        let Sentence(locale, range) = config.clone();
        // NIT: This should be `SentenceWord` to allow for iterative generation, rather than having
        // to generate and the mutate.
        let mut words: Vec<String> = locale::Sentence(locale, range).fake_with_rng(rng);
        words.iter_mut().for_each(|word| {
            let markup: WordMarkup = MarkupFreq(1.0).fake_with_rng(rng);
            markup.format_string(word)
        });
        words
    }
}
impl<L, R> Dummy<Sentence<L, R>> for String
where
    L: LocaleText,
    R: RangeBounds<usize> + Clone,
    usize: Dummy<R>,
{
    fn dummy_with_rng<Rng: rand::Rng + ?Sized>(config: &Sentence<L, R>, rng: &mut Rng) -> Self {
        let words: Vec<String> = config.fake_with_rng(rng);
        words.join(" ")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Paragraph<L, R>(pub L, pub R);
impl<L> Paragraph<L, Range<usize>> {}
impl<L> Default for Paragraph<L, Range<usize>>
where
    L: Default,
{
    fn default() -> Self {
        Self(L::default(), 1..3)
    }
}
impl<L, R> Dummy<Paragraph<L, R>> for Vec<String>
where
    L: LocaleText,
    R: RangeBounds<usize> + Clone,
    usize: Dummy<R>,
{
    fn dummy_with_rng<Rng: rand::Rng + ?Sized>(paragraph: &Paragraph<L, R>, rng: &mut Rng) -> Self {
        let Paragraph(locale, range) = paragraph.clone();
        let limit: usize = range.fake_with_rng(rng);
        iter::from_fn(|| Some(Sentence::locale(locale).fake_with_rng::<Vec<String>, Rng>(rng)))
            .take(limit)
            .flat_map(|words| words)
            .collect()
    }
}
impl<L, R> Dummy<Paragraph<L, R>> for String
where
    L: LocaleText,
    R: RangeBounds<usize> + Clone,
    usize: Dummy<R>,
{
    fn dummy_with_rng<Rng: rand::Rng + ?Sized>(paragraph: &Paragraph<L, R>, rng: &mut Rng) -> Self {
        let Paragraph(locale, range) = paragraph.clone();
        let limit: usize = range.fake_with_rng(rng);
        let words =
            iter::from_fn(|| Some(Sentence(locale, 2..=10).fake_with_rng::<String, Rng>(rng)))
                .take(limit)
                .collect::<Vec<_>>();
        words.join("\n\n")
    }
}
