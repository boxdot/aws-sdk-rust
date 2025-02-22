// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `ConflictExceptionReason`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let conflictexceptionreason = unimplemented!();
/// match conflictexceptionreason {
///     ConflictExceptionReason::DeletionProtectionEnabled => { /* ... */ },
///     ConflictExceptionReason::DestinationPhoneNumberNotVerified => { /* ... */ },
///     ConflictExceptionReason::DestinationPhoneNumberOptedOut => { /* ... */ },
///     ConflictExceptionReason::EventDestinationMismatch => { /* ... */ },
///     ConflictExceptionReason::KeywordMismatch => { /* ... */ },
///     ConflictExceptionReason::LastPhoneNumber => { /* ... */ },
///     ConflictExceptionReason::MessageTypeMismatch => { /* ... */ },
///     ConflictExceptionReason::NoOriginationIdentitiesFound => { /* ... */ },
///     ConflictExceptionReason::OptOutListMismatch => { /* ... */ },
///     ConflictExceptionReason::PhoneNumberAssociatedToPool => { /* ... */ },
///     ConflictExceptionReason::PhoneNumberNotAssociatedToPool => { /* ... */ },
///     ConflictExceptionReason::PhoneNumberNotInRegistrationRegion => { /* ... */ },
///     ConflictExceptionReason::ResourceAlreadyExists => { /* ... */ },
///     ConflictExceptionReason::ResourceDeletionNotAllowed => { /* ... */ },
///     ConflictExceptionReason::ResourceModificationNotAllowed => { /* ... */ },
///     ConflictExceptionReason::ResourceNotActive => { /* ... */ },
///     ConflictExceptionReason::ResourceNotEmpty => { /* ... */ },
///     ConflictExceptionReason::SelfManagedOptOutsMismatch => { /* ... */ },
///     ConflictExceptionReason::TwoWayConfigMismatch => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `conflictexceptionreason` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ConflictExceptionReason::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ConflictExceptionReason::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ConflictExceptionReason::NewFeature` is defined.
/// Specifically, when `conflictexceptionreason` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ConflictExceptionReason::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    ::std::clone::Clone,
    ::std::cmp::Eq,
    ::std::cmp::Ord,
    ::std::cmp::PartialEq,
    ::std::cmp::PartialOrd,
    ::std::fmt::Debug,
    ::std::hash::Hash,
)]
pub enum ConflictExceptionReason {
    #[allow(missing_docs)] // documentation missing in model
    DeletionProtectionEnabled,
    #[allow(missing_docs)] // documentation missing in model
    DestinationPhoneNumberNotVerified,
    #[allow(missing_docs)] // documentation missing in model
    DestinationPhoneNumberOptedOut,
    #[allow(missing_docs)] // documentation missing in model
    EventDestinationMismatch,
    #[allow(missing_docs)] // documentation missing in model
    KeywordMismatch,
    #[allow(missing_docs)] // documentation missing in model
    LastPhoneNumber,
    #[allow(missing_docs)] // documentation missing in model
    MessageTypeMismatch,
    #[allow(missing_docs)] // documentation missing in model
    NoOriginationIdentitiesFound,
    #[allow(missing_docs)] // documentation missing in model
    OptOutListMismatch,
    #[allow(missing_docs)] // documentation missing in model
    PhoneNumberAssociatedToPool,
    #[allow(missing_docs)] // documentation missing in model
    PhoneNumberNotAssociatedToPool,
    #[allow(missing_docs)] // documentation missing in model
    PhoneNumberNotInRegistrationRegion,
    #[allow(missing_docs)] // documentation missing in model
    ResourceAlreadyExists,
    #[allow(missing_docs)] // documentation missing in model
    ResourceDeletionNotAllowed,
    #[allow(missing_docs)] // documentation missing in model
    ResourceModificationNotAllowed,
    #[allow(missing_docs)] // documentation missing in model
    ResourceNotActive,
    #[allow(missing_docs)] // documentation missing in model
    ResourceNotEmpty,
    #[allow(missing_docs)] // documentation missing in model
    SelfManagedOptOutsMismatch,
    #[allow(missing_docs)] // documentation missing in model
    TwoWayConfigMismatch,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl ::std::convert::From<&str> for ConflictExceptionReason {
    fn from(s: &str) -> Self {
        match s {
            "DELETION_PROTECTION_ENABLED" => ConflictExceptionReason::DeletionProtectionEnabled,
            "DESTINATION_PHONE_NUMBER_NOT_VERIFIED" => {
                ConflictExceptionReason::DestinationPhoneNumberNotVerified
            }
            "DESTINATION_PHONE_NUMBER_OPTED_OUT" => {
                ConflictExceptionReason::DestinationPhoneNumberOptedOut
            }
            "EVENT_DESTINATION_MISMATCH" => ConflictExceptionReason::EventDestinationMismatch,
            "KEYWORD_MISMATCH" => ConflictExceptionReason::KeywordMismatch,
            "LAST_PHONE_NUMBER" => ConflictExceptionReason::LastPhoneNumber,
            "MESSAGE_TYPE_MISMATCH" => ConflictExceptionReason::MessageTypeMismatch,
            "NO_ORIGINATION_IDENTITIES_FOUND" => {
                ConflictExceptionReason::NoOriginationIdentitiesFound
            }
            "OPT_OUT_LIST_MISMATCH" => ConflictExceptionReason::OptOutListMismatch,
            "PHONE_NUMBER_ASSOCIATED_TO_POOL" => {
                ConflictExceptionReason::PhoneNumberAssociatedToPool
            }
            "PHONE_NUMBER_NOT_ASSOCIATED_TO_POOL" => {
                ConflictExceptionReason::PhoneNumberNotAssociatedToPool
            }
            "PHONE_NUMBER_NOT_IN_REGISTRATION_REGION" => {
                ConflictExceptionReason::PhoneNumberNotInRegistrationRegion
            }
            "RESOURCE_ALREADY_EXISTS" => ConflictExceptionReason::ResourceAlreadyExists,
            "RESOURCE_DELETION_NOT_ALLOWED" => ConflictExceptionReason::ResourceDeletionNotAllowed,
            "RESOURCE_MODIFICATION_NOT_ALLOWED" => {
                ConflictExceptionReason::ResourceModificationNotAllowed
            }
            "RESOURCE_NOT_ACTIVE" => ConflictExceptionReason::ResourceNotActive,
            "RESOURCE_NOT_EMPTY" => ConflictExceptionReason::ResourceNotEmpty,
            "SELF_MANAGED_OPT_OUTS_MISMATCH" => ConflictExceptionReason::SelfManagedOptOutsMismatch,
            "TWO_WAY_CONFIG_MISMATCH" => ConflictExceptionReason::TwoWayConfigMismatch,
            other => ConflictExceptionReason::Unknown(crate::primitives::UnknownVariantValue(
                other.to_owned(),
            )),
        }
    }
}
impl ::std::str::FromStr for ConflictExceptionReason {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(ConflictExceptionReason::from(s))
    }
}
impl ConflictExceptionReason {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ConflictExceptionReason::DeletionProtectionEnabled => "DELETION_PROTECTION_ENABLED",
            ConflictExceptionReason::DestinationPhoneNumberNotVerified => {
                "DESTINATION_PHONE_NUMBER_NOT_VERIFIED"
            }
            ConflictExceptionReason::DestinationPhoneNumberOptedOut => {
                "DESTINATION_PHONE_NUMBER_OPTED_OUT"
            }
            ConflictExceptionReason::EventDestinationMismatch => "EVENT_DESTINATION_MISMATCH",
            ConflictExceptionReason::KeywordMismatch => "KEYWORD_MISMATCH",
            ConflictExceptionReason::LastPhoneNumber => "LAST_PHONE_NUMBER",
            ConflictExceptionReason::MessageTypeMismatch => "MESSAGE_TYPE_MISMATCH",
            ConflictExceptionReason::NoOriginationIdentitiesFound => {
                "NO_ORIGINATION_IDENTITIES_FOUND"
            }
            ConflictExceptionReason::OptOutListMismatch => "OPT_OUT_LIST_MISMATCH",
            ConflictExceptionReason::PhoneNumberAssociatedToPool => {
                "PHONE_NUMBER_ASSOCIATED_TO_POOL"
            }
            ConflictExceptionReason::PhoneNumberNotAssociatedToPool => {
                "PHONE_NUMBER_NOT_ASSOCIATED_TO_POOL"
            }
            ConflictExceptionReason::PhoneNumberNotInRegistrationRegion => {
                "PHONE_NUMBER_NOT_IN_REGISTRATION_REGION"
            }
            ConflictExceptionReason::ResourceAlreadyExists => "RESOURCE_ALREADY_EXISTS",
            ConflictExceptionReason::ResourceDeletionNotAllowed => "RESOURCE_DELETION_NOT_ALLOWED",
            ConflictExceptionReason::ResourceModificationNotAllowed => {
                "RESOURCE_MODIFICATION_NOT_ALLOWED"
            }
            ConflictExceptionReason::ResourceNotActive => "RESOURCE_NOT_ACTIVE",
            ConflictExceptionReason::ResourceNotEmpty => "RESOURCE_NOT_EMPTY",
            ConflictExceptionReason::SelfManagedOptOutsMismatch => "SELF_MANAGED_OPT_OUTS_MISMATCH",
            ConflictExceptionReason::TwoWayConfigMismatch => "TWO_WAY_CONFIG_MISMATCH",
            ConflictExceptionReason::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "DELETION_PROTECTION_ENABLED",
            "DESTINATION_PHONE_NUMBER_NOT_VERIFIED",
            "DESTINATION_PHONE_NUMBER_OPTED_OUT",
            "EVENT_DESTINATION_MISMATCH",
            "KEYWORD_MISMATCH",
            "LAST_PHONE_NUMBER",
            "MESSAGE_TYPE_MISMATCH",
            "NO_ORIGINATION_IDENTITIES_FOUND",
            "OPT_OUT_LIST_MISMATCH",
            "PHONE_NUMBER_ASSOCIATED_TO_POOL",
            "PHONE_NUMBER_NOT_ASSOCIATED_TO_POOL",
            "PHONE_NUMBER_NOT_IN_REGISTRATION_REGION",
            "RESOURCE_ALREADY_EXISTS",
            "RESOURCE_DELETION_NOT_ALLOWED",
            "RESOURCE_MODIFICATION_NOT_ALLOWED",
            "RESOURCE_NOT_ACTIVE",
            "RESOURCE_NOT_EMPTY",
            "SELF_MANAGED_OPT_OUTS_MISMATCH",
            "TWO_WAY_CONFIG_MISMATCH",
        ]
    }
}
impl ::std::convert::AsRef<str> for ConflictExceptionReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
