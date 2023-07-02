use super::locale::LocaleText;
use crate::dev::fake::text::locale;
use fake::{Dummy, Fake, Faker};
use rand::Rng;

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
        match (0.0..100.).fake_with_rng::<f32, _>(rng) {
            i if i < 2.5 * freq => Self::ItalicStar,
            i if i < 5.0 * freq => Self::ItalicUnderscore,
            i if i < 7.5 * freq => Self::BoldStar,
            i if i < 10. * freq => Self::BoldUnderscore,
            i if i < 12.5 * freq => Self::BoldItalicStar,
            i if i < 15.0 * freq => Self::BoldItalicUnderscore,
            _ => Self::None,
        }
    }
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
        let Sentence { locale } = *config;
        // NIT: This should be `SentenceWord` to allow for iterative generation, rather than having
        // to generate and the mutate.
        let mut words: Vec<String> = locale::Sentence { locale }.fake_with_rng(rng);
        words.iter_mut().for_each(|word| {
            let markup: WordMarkup = MarkupFreq::default().fake_with_rng(rng);
            markup.format_string(word)
        });
        words
    }
}
impl<L> Dummy<Sentence<L>> for String
where
    L: LocaleText,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Sentence<L>, rng: &mut R) -> Self {
        let words: Vec<String> = config.fake_with_rng(rng);
        words.join(" ")
    }
}
