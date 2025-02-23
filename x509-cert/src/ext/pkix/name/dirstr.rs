use der::asn1::{PrintableStringRef, TeletexStringRef, Utf8StringRef};
use der::Choice;

/// DirectoryString as defined in [RFC 5280 Section 4.2.1.4].
///
/// ASN.1 structure for DirectoryString is below.
///
/// ```text
/// DirectoryString ::= CHOICE {
///     teletexString           TeletexString (SIZE (1..MAX)),
///     printableString         PrintableString (SIZE (1..MAX)),
///     universalString         UniversalString (SIZE (1..MAX)),
///     utf8String              UTF8String (SIZE (1..MAX)),
///     bmpString               BMPString (SIZE (1..MAX))
/// }
/// ```
///
/// Further, [RFC 5280 Section 4.2.1.4] states:
///
/// ```text
/// The DirectoryString type is defined as a choice of PrintableString,
/// TeletexString, BMPString, UTF8String, and UniversalString.  CAs
/// conforming to this profile MUST use either the PrintableString or
/// UTF8String encoding of DirectoryString, with two exceptions.  When
/// CAs have previously issued certificates with issuer fields with
/// attributes encoded using TeletexString, BMPString, or
/// UniversalString, then the CA MAY continue to use these encodings of
/// the DirectoryString to preserve backward compatibility.  Also, new
/// CAs that are added to a domain where existing CAs issue certificates
/// with issuer fields with attributes encoded using TeletexString,
/// BMPString, or UniversalString MAY encode attributes that they share
/// with the existing CAs using the same encodings as the existing CAs
/// use.
/// ```
///
/// The implication of the above paragraph is that `PrintableString` and
/// `UTF8String` are the new types and the other types are legacy. Until
/// the need arises, we only support `PrintableString` and `UTF8String`.
///
/// [RFC 5280 Section 4.2.1.4]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.4
#[derive(Clone, Debug, Eq, PartialEq, Choice)]
#[allow(missing_docs)]
pub enum DirectoryString<'a> {
    #[asn1(type = "PrintableString")]
    PrintableString(PrintableStringRef<'a>),

    #[asn1(type = "TeletexString")]
    TeletexString(TeletexStringRef<'a>),

    #[asn1(type = "UTF8String")]
    Utf8String(Utf8StringRef<'a>),
}
