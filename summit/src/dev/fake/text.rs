use self::locale::Locale;
use compact_str::CompactString;
use fake::{faker, Dummy, Fake};
use rand::Rng;

pub mod locale;
pub mod markdown;

pub struct Name(pub Locale);
impl Dummy<Name> for CompactString {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Name, rng: &mut R) -> Self {
        match config.0 {
            Locale::En => faker::name::en::Name()
                .fake_with_rng::<String, _>(rng)
                .into(),
        }
    }
}

pub struct FediUser(pub Locale);
impl Dummy<FediUser> for CompactString {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &FediUser, rng: &mut R) -> Self {
        // TODO: add in additional chars like underscores or whatever users normally use.
        Name(config.0).fake_with_rng(rng)
    }
}

pub struct FediHost(pub Locale);
impl Dummy<FediHost> for CompactString {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &FediHost, rng: &mut R) -> Self {
        // TODO: convert this to a host. Prob add a URL type and generate for that?
        Name(config.0).fake_with_rng(rng)
    }
}
