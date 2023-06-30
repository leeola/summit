//! A Base32 encoding inspired by [Crockford 32](https://www.crockford.com/base32.html) with
//! some slight alterations resulting in a character set that offers worse spoken ambiguity but
//! less chance that "offensive" words are created from an encoded UUID.
//!
//! - Copy-paste focused. Spoken ambiguity is not a concern. Readability is a concern, but only
//!   slightly, as no one is expected to type these out by hand.
//! - Removal of all vowels, making most words difficult to create.
//! - Removal of vowel lookalikes, `v0134`.
//! - Removal of letters by letter frequency from various sources, `tnslrc`, and any lookalikes for
//!   these, `75`.
//!
//! Leaving us with the chosen 32 characters: `"2689bdfghjkmpqwxyzBDFGHJKMPQWXYZ"`
//!
//! # Motivations
//!
//! Base64 struggles with being both double-click select and URL friendly, so Base32 was chosen.
//! After that, one needs to choose which character set they're going with.
//!
//! One non-controversial measurement to help form a decision could be spoken or read ambiguity.
//! Crockford gets rid of several of these, but kept some ambiguity by still supporting `0` and
//! `1`[1], `5`, `S`, and so forth; All of which are easy to misread with the right font.
//! Crockford32's solution for some of this is to allow aliasing. I disagree with this point, as i
//! find aliasing itself to allow for potentially worse ambiguous situations, such as searching
//! records which happen to contain an aliased value.
//!
//! Removing `0`, `1`, `5`, `S` and etc however leaves us below the necessary 32 chars. Expanding
//! the set to include upper case gives more than enough, but that brings one final question, now we
//! have too many chars.. which subset do we pick?
//!
//! While i don't personally value removing offensive words, some do. Since i've already gotten the
//! few criteria i want in a base32 charset, i could just truncate the rest .. _or_, try to build
//! additional value. I chose the latter, and attempted to extend Crockford's "Accidental Obscenity"
//! goal. This mostly just translates to letters that common words _("offensive" or otherwise)_
//! frequently use.
//!
//! [1]: Yes, for both `0` and `1` Crockford does not include the necessary letters to conflate
//! them in the encoding. However if you misread and mistype them, not knowing that the `1` is a one
//! and not an `l`, you still fell into the trap.

/// A simple Base32 character set.
///
/// See also: [`crate::uuid::base32_nowords`].
pub const BASE32_NOWORDS: data_encoding::Encoding = data_encoding_macro::new_encoding! {
    symbols: "2689bdfghjkmpqwxyzBDFGHJKMPQWXYZ",
};
