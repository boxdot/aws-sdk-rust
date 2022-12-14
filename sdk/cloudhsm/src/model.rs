// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A key-value pair that identifies or specifies metadata about an AWS CloudHSM resource.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[doc(hidden)]
    pub key: std::option::Option<std::string::String>,
    /// <p>The value of the tag.</p>
    #[doc(hidden)]
    pub value: std::option::Option<std::string::String>,
}
impl Tag {
    /// <p>The key of the tag.</p>
    pub fn key(&self) -> std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>The value of the tag.</p>
    pub fn value(&self) -> std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Tag");
        formatter.field("key", &self.key);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`Tag`](crate::model::Tag).
pub mod tag {

    /// A builder for [`Tag`](crate::model::Tag).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) key: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The key of the tag.</p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.key = Some(input.into());
            self
        }
        /// <p>The key of the tag.</p>
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.key = input;
            self
        }
        /// <p>The value of the tag.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        /// <p>The value of the tag.</p>
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`Tag`](crate::model::Tag).
        pub fn build(self) -> crate::model::Tag {
            crate::model::Tag {
                key: self.key,
                value: self.value,
            }
        }
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::model::Tag).
    pub fn builder() -> crate::model::tag::Builder {
        crate::model::tag::Builder::default()
    }
}

/// When writing a match expression against `ClientVersion`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let clientversion = unimplemented!();
/// match clientversion {
///     ClientVersion::FiveOne => { /* ... */ },
///     ClientVersion::FiveThree => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `clientversion` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ClientVersion::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ClientVersion::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ClientVersion::NewFeature` is defined.
/// Specifically, when `clientversion` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ClientVersion::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ClientVersion {
    #[allow(missing_docs)] // documentation missing in model
    FiveOne,
    #[allow(missing_docs)] // documentation missing in model
    FiveThree,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for ClientVersion {
    fn from(s: &str) -> Self {
        match s {
            "5.1" => ClientVersion::FiveOne,
            "5.3" => ClientVersion::FiveThree,
            other => ClientVersion::Unknown(crate::types::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for ClientVersion {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ClientVersion::from(s))
    }
}
impl ClientVersion {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ClientVersion::FiveOne => "5.1",
            ClientVersion::FiveThree => "5.3",
            ClientVersion::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["5.1", "5.3"]
    }
}
impl AsRef<str> for ClientVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// When writing a match expression against `SubscriptionType`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let subscriptiontype = unimplemented!();
/// match subscriptiontype {
///     SubscriptionType::Production => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `subscriptiontype` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `SubscriptionType::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `SubscriptionType::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `SubscriptionType::NewFeature` is defined.
/// Specifically, when `subscriptiontype` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `SubscriptionType::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
/// <p>Specifies the type of subscription for the HSM.</p>
/// <ul>
/// <li>
/// <p>
/// <b>PRODUCTION</b> - The HSM is being used in a production
/// environment.</p>
/// </li>
/// <li>
/// <p>
/// <b>TRIAL</b> - The HSM is being used in a product
/// trial.</p>
/// </li>
/// </ul>
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum SubscriptionType {
    #[allow(missing_docs)] // documentation missing in model
    Production,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for SubscriptionType {
    fn from(s: &str) -> Self {
        match s {
            "PRODUCTION" => SubscriptionType::Production,
            other => SubscriptionType::Unknown(crate::types::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for SubscriptionType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(SubscriptionType::from(s))
    }
}
impl SubscriptionType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionType::Production => "PRODUCTION",
            SubscriptionType::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["PRODUCTION"]
    }
}
impl AsRef<str> for SubscriptionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// When writing a match expression against `HsmStatus`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let hsmstatus = unimplemented!();
/// match hsmstatus {
///     HsmStatus::Degraded => { /* ... */ },
///     HsmStatus::Pending => { /* ... */ },
///     HsmStatus::Running => { /* ... */ },
///     HsmStatus::Suspended => { /* ... */ },
///     HsmStatus::Terminated => { /* ... */ },
///     HsmStatus::Terminating => { /* ... */ },
///     HsmStatus::Updating => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `hsmstatus` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `HsmStatus::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `HsmStatus::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `HsmStatus::NewFeature` is defined.
/// Specifically, when `hsmstatus` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `HsmStatus::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum HsmStatus {
    #[allow(missing_docs)] // documentation missing in model
    Degraded,
    #[allow(missing_docs)] // documentation missing in model
    Pending,
    #[allow(missing_docs)] // documentation missing in model
    Running,
    #[allow(missing_docs)] // documentation missing in model
    Suspended,
    #[allow(missing_docs)] // documentation missing in model
    Terminated,
    #[allow(missing_docs)] // documentation missing in model
    Terminating,
    #[allow(missing_docs)] // documentation missing in model
    Updating,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for HsmStatus {
    fn from(s: &str) -> Self {
        match s {
            "DEGRADED" => HsmStatus::Degraded,
            "PENDING" => HsmStatus::Pending,
            "RUNNING" => HsmStatus::Running,
            "SUSPENDED" => HsmStatus::Suspended,
            "TERMINATED" => HsmStatus::Terminated,
            "TERMINATING" => HsmStatus::Terminating,
            "UPDATING" => HsmStatus::Updating,
            other => HsmStatus::Unknown(crate::types::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for HsmStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(HsmStatus::from(s))
    }
}
impl HsmStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            HsmStatus::Degraded => "DEGRADED",
            HsmStatus::Pending => "PENDING",
            HsmStatus::Running => "RUNNING",
            HsmStatus::Suspended => "SUSPENDED",
            HsmStatus::Terminated => "TERMINATED",
            HsmStatus::Terminating => "TERMINATING",
            HsmStatus::Updating => "UPDATING",
            HsmStatus::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &[
            "DEGRADED",
            "PENDING",
            "RUNNING",
            "SUSPENDED",
            "TERMINATED",
            "TERMINATING",
            "UPDATING",
        ]
    }
}
impl AsRef<str> for HsmStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// When writing a match expression against `CloudHsmObjectState`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let cloudhsmobjectstate = unimplemented!();
/// match cloudhsmobjectstate {
///     CloudHsmObjectState::Degraded => { /* ... */ },
///     CloudHsmObjectState::Ready => { /* ... */ },
///     CloudHsmObjectState::Updating => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `cloudhsmobjectstate` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `CloudHsmObjectState::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `CloudHsmObjectState::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `CloudHsmObjectState::NewFeature` is defined.
/// Specifically, when `cloudhsmobjectstate` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `CloudHsmObjectState::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum CloudHsmObjectState {
    #[allow(missing_docs)] // documentation missing in model
    Degraded,
    #[allow(missing_docs)] // documentation missing in model
    Ready,
    #[allow(missing_docs)] // documentation missing in model
    Updating,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for CloudHsmObjectState {
    fn from(s: &str) -> Self {
        match s {
            "DEGRADED" => CloudHsmObjectState::Degraded,
            "READY" => CloudHsmObjectState::Ready,
            "UPDATING" => CloudHsmObjectState::Updating,
            other => {
                CloudHsmObjectState::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for CloudHsmObjectState {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(CloudHsmObjectState::from(s))
    }
}
impl CloudHsmObjectState {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            CloudHsmObjectState::Degraded => "DEGRADED",
            CloudHsmObjectState::Ready => "READY",
            CloudHsmObjectState::Updating => "UPDATING",
            CloudHsmObjectState::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["DEGRADED", "READY", "UPDATING"]
    }
}
impl AsRef<str> for CloudHsmObjectState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
