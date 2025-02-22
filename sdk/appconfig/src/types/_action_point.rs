// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `ActionPoint`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let actionpoint = unimplemented!();
/// match actionpoint {
///     ActionPoint::OnDeploymentBaking => { /* ... */ },
///     ActionPoint::OnDeploymentComplete => { /* ... */ },
///     ActionPoint::OnDeploymentRolledBack => { /* ... */ },
///     ActionPoint::OnDeploymentStart => { /* ... */ },
///     ActionPoint::OnDeploymentStep => { /* ... */ },
///     ActionPoint::PreCreateHostedConfigurationVersion => { /* ... */ },
///     ActionPoint::PreStartDeployment => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `actionpoint` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ActionPoint::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ActionPoint::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ActionPoint::NewFeature` is defined.
/// Specifically, when `actionpoint` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ActionPoint::NewFeature` also yielding `"NewFeature"`.
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
pub enum ActionPoint {
    #[allow(missing_docs)] // documentation missing in model
    OnDeploymentBaking,
    #[allow(missing_docs)] // documentation missing in model
    OnDeploymentComplete,
    #[allow(missing_docs)] // documentation missing in model
    OnDeploymentRolledBack,
    #[allow(missing_docs)] // documentation missing in model
    OnDeploymentStart,
    #[allow(missing_docs)] // documentation missing in model
    OnDeploymentStep,
    #[allow(missing_docs)] // documentation missing in model
    PreCreateHostedConfigurationVersion,
    #[allow(missing_docs)] // documentation missing in model
    PreStartDeployment,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl ::std::convert::From<&str> for ActionPoint {
    fn from(s: &str) -> Self {
        match s {
            "ON_DEPLOYMENT_BAKING" => ActionPoint::OnDeploymentBaking,
            "ON_DEPLOYMENT_COMPLETE" => ActionPoint::OnDeploymentComplete,
            "ON_DEPLOYMENT_ROLLED_BACK" => ActionPoint::OnDeploymentRolledBack,
            "ON_DEPLOYMENT_START" => ActionPoint::OnDeploymentStart,
            "ON_DEPLOYMENT_STEP" => ActionPoint::OnDeploymentStep,
            "PRE_CREATE_HOSTED_CONFIGURATION_VERSION" => {
                ActionPoint::PreCreateHostedConfigurationVersion
            }
            "PRE_START_DEPLOYMENT" => ActionPoint::PreStartDeployment,
            other => ActionPoint::Unknown(crate::primitives::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl ::std::str::FromStr for ActionPoint {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(ActionPoint::from(s))
    }
}
impl ActionPoint {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ActionPoint::OnDeploymentBaking => "ON_DEPLOYMENT_BAKING",
            ActionPoint::OnDeploymentComplete => "ON_DEPLOYMENT_COMPLETE",
            ActionPoint::OnDeploymentRolledBack => "ON_DEPLOYMENT_ROLLED_BACK",
            ActionPoint::OnDeploymentStart => "ON_DEPLOYMENT_START",
            ActionPoint::OnDeploymentStep => "ON_DEPLOYMENT_STEP",
            ActionPoint::PreCreateHostedConfigurationVersion => {
                "PRE_CREATE_HOSTED_CONFIGURATION_VERSION"
            }
            ActionPoint::PreStartDeployment => "PRE_START_DEPLOYMENT",
            ActionPoint::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "ON_DEPLOYMENT_BAKING",
            "ON_DEPLOYMENT_COMPLETE",
            "ON_DEPLOYMENT_ROLLED_BACK",
            "ON_DEPLOYMENT_START",
            "ON_DEPLOYMENT_STEP",
            "PRE_CREATE_HOSTED_CONFIGURATION_VERSION",
            "PRE_START_DEPLOYMENT",
        ]
    }
}
impl ::std::convert::AsRef<str> for ActionPoint {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
