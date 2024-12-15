//! `decode!` macro and the associated helper types.

use crate::{
    decoder::Decoder,
    wrappers::{Pem, SkipWhitespace, Skipper},
};

/// Computes the output length in compile time and decodes the input. This allows to skip specifying
/// output length manually.
///
/// The macro accepts two comma-separate expressions. The first arg must evaluate to [`Decoder`],
/// [`SkipWhitespace`], or [`Pem`]. The second argument must evaluate to `&[u8]`. Both expressions
/// must be assignable to constants. The output of a macro is an array `[u8; N]` with the decoded bytes.
///
/// # Examples
///
/// ## Usage with `Decoder`s
///
/// ```
/// use const_decoder::{decode, Decoder};
///
/// const HEX: &[u8] = &decode!(Decoder::Hex, b"c0ffee");
/// const BASE64: &[u8] = &decode!(Decoder::Base64, b"VGVzdCBzdHJpbmc=");
/// // Can be used with custom decoders as well
/// const BASE32: &[u8] = &decode!(
///     Decoder::custom("qpzry9x8gf2tvdw0s3jn54khce6mua7l"),
///     b"rp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q",
/// );
/// ```
///
/// ## Usage with `SkipWhitespace`
///
/// ```
/// # use const_decoder::{decode, Decoder};
/// const HEX: &[u8] = &decode!(
///     Decoder::Hex.skip_whitespace(),
///     b"c0ff ee00 beef",
/// );
/// ```
///
/// ## Usage with `Pem`
///
/// ```
/// # use const_decoder::{decode, Pem};
/// const PRIVATE_KEY: &[u8] = &decode!(
///     Pem,
///     b"-----BEGIN PRIVATE KEY-----
///       MC4CAQAwBQYDK2VuBCIEINAOV4yAyaoM2wmJPApQs3byDhw7oJRG47V0VHwGnctD
///       -----END PRIVATE KEY-----",
/// );
/// ```
#[macro_export]
macro_rules! decode {
    ($decoder:expr, $bytes:expr $(,)?) => {{
        const __OUTPUT_LEN: usize = $crate::DecoderWrapper($decoder).decode_len($bytes);
        $crate::DecoderWrapper($decoder).decode::<__OUTPUT_LEN>($bytes) as [u8; __OUTPUT_LEN]
    }};
}

/// decode base64 encoded data.
#[macro_export]
macro_rules! decode_base64 {
    ($bytes:expr $(,)?) => {{
        $crate::decode!($crate::Decoder::Base64, $bytes);
    }};
}

/// decode base64url encoded data.
#[macro_export]
macro_rules! decode_base64_url {
    ($bytes:expr $(,)?) => {{
        $crate::decode!($crate::Decoder::Base64Url, $bytes);
    }};
}

/// It conforms to [RFC4648](https://tools.ietf.org/html/rfc4648).
///
/// from: <https://docs.rs/data-encoding/2.6.0/data_encoding/constant.BASE32.html>
#[macro_export]
macro_rules! decode_base32 {
    ($bytes:expr $(,)?) => {{
        $crate::decode!($crate::Decoder::custom("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"), $bytes);
    }};
}

/// It conforms to [RFC4648](https://tools.ietf.org/html/rfc4648).
///
/// from: <https://docs.rs/data-encoding/2.6.0/data_encoding/constant.BASE32HEX.html>
#[macro_export]
macro_rules! decode_base32_hex {
    ($bytes:expr $(,)?) => {{
        $crate::decode!($crate::Decoder::custom("0123456789ABCDEFGHIJKLMNOPQRSTUV"), $bytes);
    }};
}

/// It conforms to [RFC5155](https://tools.ietf.org/html/rfc5155):
///     It uses a base32 extended hex alphabet.
///     It is case-insensitive when decoding and uses lowercase when encoding.
///     It does not use padding.
///
/// from: <https://docs.rs/data-encoding/2.6.0/data_encoding/constant.BASE32_DNSSEC.html>
#[macro_export]
macro_rules! decode_base32_dnssec {
    ($bytes:expr $(,)?) => {{
        $crate::decode!($crate::Decoder::custom("0123456789abcdefghijklmnopqrstuv"), $bytes);
    }};
}

/// It conforms to [DNSCurve](https://dnscurve.org/in-implement.html).
///
/// from: <https://docs.rs/data-encoding/2.6.0/data_encoding/constant.BASE32_DNSCURVE.html>
#[macro_export]
macro_rules! decode_base32_dnscurve {
    ($bytes:expr $(,)?) => {{
        $crate::decode!($crate::Decoder::custom("0123456789bcdfghjklmnpqrstuvwxyz"), $bytes);
    }};
}

#[derive(Debug)]
#[doc(hidden)] // implementation detail of the `decode!` macro
pub struct DecoderWrapper<T>(pub T);

impl DecoderWrapper<Decoder> {
    pub const fn decode_len(&self, input: &[u8]) -> usize {
        self.0.do_decode_len(input, None)
    }

    pub const fn decode<const N: usize>(self, input: &[u8]) -> [u8; N] {
        self.0.decode(input)
    }
}

impl DecoderWrapper<SkipWhitespace> {
    pub const fn decode_len(&self, input: &[u8]) -> usize {
        let Self(SkipWhitespace(decoder)) = self;
        decoder.do_decode_len(input, Some(Skipper::Whitespace))
    }

    pub const fn decode<const N: usize>(self, input: &[u8]) -> [u8; N] {
        self.0.decode(input)
    }
}

impl DecoderWrapper<Pem> {
    pub const fn decode_len(&self, input: &[u8]) -> usize {
        Decoder::Base64.do_decode_len(input, Some(Skipper::Pem))
    }

    pub const fn decode<const N: usize>(self, input: &[u8]) -> [u8; N] {
        Pem::decode(input)
    }
}
