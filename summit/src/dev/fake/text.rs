use compact_str::CompactString;
use fake::{faker::name::en, Dummy, Fake};

/// An easier to configure value for locale generating [`Dummy`]s.
//
// NIT: This makes dummy generation super verbose, need to find a way to make this more generic,
// otherwise every dummy has a match that will be awful anytime a new lang variant is added.
#[derive(Debug, Default, Clone, Copy)]
pub enum Locale {
    #[default]
    En,
}
pub struct Name(pub Locale);
impl Dummy<Name> for CompactString {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Name, rng: &mut R) -> Self {
        match config.0 {
            Locale::En => en::Name().fake_with_rng::<String, _>(rng).into(),
        }
    }
}

pub struct FediUser(pub Locale);
impl Dummy<FediUser> for CompactString {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &FediUser, rng: &mut R) -> Self {
        // TODO: add in additional chars like underscores or whatever users normally use.
        match config.0 {
            Locale::En => en::Name().fake_with_rng::<String, _>(rng).into(),
        }
    }
}

pub struct FediHost(pub Locale);
impl Dummy<FediHost> for CompactString {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &FediHost, rng: &mut R) -> Self {
        // TODO: convert this to a host. Prob add a URL type and generate for that?
        match config.0 {
            Locale::En => en::Name().fake_with_rng::<String, _>(rng).into(),
        }
    }
}
