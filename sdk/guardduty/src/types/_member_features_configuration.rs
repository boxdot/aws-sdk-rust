// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the features for the member account.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MemberFeaturesConfiguration {
    /// <p>The name of the feature.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<crate::types::OrgFeature>,
    /// <p>The status of the feature.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::FeatureStatus>,
    /// <p>Additional configuration of the feature for the member account.</p>
    #[doc(hidden)]
    pub additional_configuration:
        ::std::option::Option<::std::vec::Vec<crate::types::MemberAdditionalConfiguration>>,
}
impl MemberFeaturesConfiguration {
    /// <p>The name of the feature.</p>
    pub fn name(&self) -> ::std::option::Option<&crate::types::OrgFeature> {
        self.name.as_ref()
    }
    /// <p>The status of the feature.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::FeatureStatus> {
        self.status.as_ref()
    }
    /// <p>Additional configuration of the feature for the member account.</p>
    pub fn additional_configuration(
        &self,
    ) -> ::std::option::Option<&[crate::types::MemberAdditionalConfiguration]> {
        self.additional_configuration.as_deref()
    }
}
impl MemberFeaturesConfiguration {
    /// Creates a new builder-style object to manufacture [`MemberFeaturesConfiguration`](crate::types::MemberFeaturesConfiguration).
    pub fn builder() -> crate::types::builders::MemberFeaturesConfigurationBuilder {
        crate::types::builders::MemberFeaturesConfigurationBuilder::default()
    }
}

/// A builder for [`MemberFeaturesConfiguration`](crate::types::MemberFeaturesConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MemberFeaturesConfigurationBuilder {
    pub(crate) name: ::std::option::Option<crate::types::OrgFeature>,
    pub(crate) status: ::std::option::Option<crate::types::FeatureStatus>,
    pub(crate) additional_configuration:
        ::std::option::Option<::std::vec::Vec<crate::types::MemberAdditionalConfiguration>>,
}
impl MemberFeaturesConfigurationBuilder {
    /// <p>The name of the feature.</p>
    pub fn name(mut self, input: crate::types::OrgFeature) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the feature.</p>
    pub fn set_name(mut self, input: ::std::option::Option<crate::types::OrgFeature>) -> Self {
        self.name = input;
        self
    }
    /// <p>The status of the feature.</p>
    pub fn status(mut self, input: crate::types::FeatureStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the feature.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::FeatureStatus>) -> Self {
        self.status = input;
        self
    }
    /// Appends an item to `additional_configuration`.
    ///
    /// To override the contents of this collection use [`set_additional_configuration`](Self::set_additional_configuration).
    ///
    /// <p>Additional configuration of the feature for the member account.</p>
    pub fn additional_configuration(
        mut self,
        input: crate::types::MemberAdditionalConfiguration,
    ) -> Self {
        let mut v = self.additional_configuration.unwrap_or_default();
        v.push(input);
        self.additional_configuration = ::std::option::Option::Some(v);
        self
    }
    /// <p>Additional configuration of the feature for the member account.</p>
    pub fn set_additional_configuration(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MemberAdditionalConfiguration>>,
    ) -> Self {
        self.additional_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`MemberFeaturesConfiguration`](crate::types::MemberFeaturesConfiguration).
    pub fn build(self) -> crate::types::MemberFeaturesConfiguration {
        crate::types::MemberFeaturesConfiguration {
            name: self.name,
            status: self.status,
            additional_configuration: self.additional_configuration,
        }
    }
}
