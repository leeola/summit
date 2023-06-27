use compact_str::CompactString;
use fake::{
    faker::{self, lorem},
    Dummy, Fake, Faker,
};
use rand::{seq::SliceRandom, Rng};

pub mod en;

trait LocaleText {
    fn punc(&self) -> &'static [&'static str];
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

pub struct Name(pub Locale);
impl Dummy<Name> for CompactString {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Name, rng: &mut R) -> Self {
        match config.0 {
            Locale::En => faker::name::en::Name()
                .fake_with_rng::<String, _>(rng)
                .into(),
        }
    }
}

pub struct FediUser(pub Locale);
impl Dummy<FediUser> for CompactString {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &FediUser, rng: &mut R) -> Self {
        // TODO: add in additional chars like underscores or whatever users normally use.
        Name(config.0).fake_with_rng(rng)
    }
}

pub struct FediHost(pub Locale);
impl Dummy<FediHost> for CompactString {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &FediHost, rng: &mut R) -> Self {
        // TODO: convert this to a host. Prob add a URL type and generate for that?
        Name(config.0).fake_with_rng(rng)
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
// impl Dummy<SentFrag> for String {
//     fn dummy_with_rng<R: Rng + ?Sized>(config: &SentFrag, rng: &mut R) -> Self {}
// }

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

#[cfg(test)]
pub mod test {
    use super::{Locale, Sent};
    use fake::Fake;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn sentence() {
        let mut rng = StdRng::seed_from_u64(0xDEADBEEF);
        assert_eq!(Sent(Locale::En).fake_with_rng::<String, _>(&mut rng), "foo");
    }
}
