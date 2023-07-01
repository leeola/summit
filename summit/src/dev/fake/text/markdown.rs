use super::locale::{Locale, LocaleText};
use crate::dev::fake::text::locale;
use fake::{Dummy, Fake, Faker};
use rand::Rng;

#[derive(Debug, Default, Clone, Copy)]
pub enum WordMarkup {
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
impl Dummy<Faker> for WordMarkup {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        match (0..100).fake_with_rng(rng) {
            i if (0..15).contains(&i) => Self::Italic,
            i if (15..30).contains(&i) => Self::Bold,
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
        // TODO: This should be `SentenceWord` to allow for iterative generation, rather than having
        // to generate and the mutate.
        let words: Vec<String> = locale::Sentence { locale }.fake_with_rng(rng);
        todo!()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::dev::fake::text::locale::en::EnLocale;
    use fake::Fake;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn markdown_sentence() {
        let mut rng = StdRng::seed_from_u64(0xDEADBEEF);
        assert_eq!(
            Sentence::<EnLocale>::default()
                .fake_with_rng::<Vec<String>, _>(&mut rng)
                .join(" "),
            "foo"
        );
    }
}
