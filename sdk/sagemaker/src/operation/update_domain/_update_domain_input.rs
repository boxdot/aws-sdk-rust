// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateDomainInput {
    /// <p>The ID of the domain to be updated.</p>
    #[doc(hidden)]
    pub domain_id: ::std::option::Option<::std::string::String>,
    /// <p>A collection of settings.</p>
    #[doc(hidden)]
    pub default_user_settings: ::std::option::Option<crate::types::UserSettings>,
    /// <p>A collection of <code>DomainSettings</code> configuration values to update.</p>
    #[doc(hidden)]
    pub domain_settings_for_update: ::std::option::Option<crate::types::DomainSettingsForUpdate>,
    /// <p>The default settings used to create a space within the Domain.</p>
    #[doc(hidden)]
    pub default_space_settings: ::std::option::Option<crate::types::DefaultSpaceSettings>,
    /// <p>The entity that creates and manages the required security groups for inter-app communication in <code>VPCOnly</code> mode. Required when <code>CreateDomain.AppNetworkAccessType</code> is <code>VPCOnly</code> and <code>DomainSettings.RStudioServerProDomainSettings.DomainExecutionRoleArn</code> is provided. If setting up the domain for use with RStudio, this value must be set to <code>Service</code>.</p>
    #[doc(hidden)]
    pub app_security_group_management:
        ::std::option::Option<crate::types::AppSecurityGroupManagement>,
}
impl UpdateDomainInput {
    /// <p>The ID of the domain to be updated.</p>
    pub fn domain_id(&self) -> ::std::option::Option<&str> {
        self.domain_id.as_deref()
    }
    /// <p>A collection of settings.</p>
    pub fn default_user_settings(&self) -> ::std::option::Option<&crate::types::UserSettings> {
        self.default_user_settings.as_ref()
    }
    /// <p>A collection of <code>DomainSettings</code> configuration values to update.</p>
    pub fn domain_settings_for_update(
        &self,
    ) -> ::std::option::Option<&crate::types::DomainSettingsForUpdate> {
        self.domain_settings_for_update.as_ref()
    }
    /// <p>The default settings used to create a space within the Domain.</p>
    pub fn default_space_settings(
        &self,
    ) -> ::std::option::Option<&crate::types::DefaultSpaceSettings> {
        self.default_space_settings.as_ref()
    }
    /// <p>The entity that creates and manages the required security groups for inter-app communication in <code>VPCOnly</code> mode. Required when <code>CreateDomain.AppNetworkAccessType</code> is <code>VPCOnly</code> and <code>DomainSettings.RStudioServerProDomainSettings.DomainExecutionRoleArn</code> is provided. If setting up the domain for use with RStudio, this value must be set to <code>Service</code>.</p>
    pub fn app_security_group_management(
        &self,
    ) -> ::std::option::Option<&crate::types::AppSecurityGroupManagement> {
        self.app_security_group_management.as_ref()
    }
}
impl UpdateDomainInput {
    /// Creates a new builder-style object to manufacture [`UpdateDomainInput`](crate::operation::update_domain::UpdateDomainInput).
    pub fn builder() -> crate::operation::update_domain::builders::UpdateDomainInputBuilder {
        crate::operation::update_domain::builders::UpdateDomainInputBuilder::default()
    }
}

/// A builder for [`UpdateDomainInput`](crate::operation::update_domain::UpdateDomainInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateDomainInputBuilder {
    pub(crate) domain_id: ::std::option::Option<::std::string::String>,
    pub(crate) default_user_settings: ::std::option::Option<crate::types::UserSettings>,
    pub(crate) domain_settings_for_update:
        ::std::option::Option<crate::types::DomainSettingsForUpdate>,
    pub(crate) default_space_settings: ::std::option::Option<crate::types::DefaultSpaceSettings>,
    pub(crate) app_security_group_management:
        ::std::option::Option<crate::types::AppSecurityGroupManagement>,
}
impl UpdateDomainInputBuilder {
    /// <p>The ID of the domain to be updated.</p>
    pub fn domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the domain to be updated.</p>
    pub fn set_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_id = input;
        self
    }
    /// <p>A collection of settings.</p>
    pub fn default_user_settings(mut self, input: crate::types::UserSettings) -> Self {
        self.default_user_settings = ::std::option::Option::Some(input);
        self
    }
    /// <p>A collection of settings.</p>
    pub fn set_default_user_settings(
        mut self,
        input: ::std::option::Option<crate::types::UserSettings>,
    ) -> Self {
        self.default_user_settings = input;
        self
    }
    /// <p>A collection of <code>DomainSettings</code> configuration values to update.</p>
    pub fn domain_settings_for_update(
        mut self,
        input: crate::types::DomainSettingsForUpdate,
    ) -> Self {
        self.domain_settings_for_update = ::std::option::Option::Some(input);
        self
    }
    /// <p>A collection of <code>DomainSettings</code> configuration values to update.</p>
    pub fn set_domain_settings_for_update(
        mut self,
        input: ::std::option::Option<crate::types::DomainSettingsForUpdate>,
    ) -> Self {
        self.domain_settings_for_update = input;
        self
    }
    /// <p>The default settings used to create a space within the Domain.</p>
    pub fn default_space_settings(mut self, input: crate::types::DefaultSpaceSettings) -> Self {
        self.default_space_settings = ::std::option::Option::Some(input);
        self
    }
    /// <p>The default settings used to create a space within the Domain.</p>
    pub fn set_default_space_settings(
        mut self,
        input: ::std::option::Option<crate::types::DefaultSpaceSettings>,
    ) -> Self {
        self.default_space_settings = input;
        self
    }
    /// <p>The entity that creates and manages the required security groups for inter-app communication in <code>VPCOnly</code> mode. Required when <code>CreateDomain.AppNetworkAccessType</code> is <code>VPCOnly</code> and <code>DomainSettings.RStudioServerProDomainSettings.DomainExecutionRoleArn</code> is provided. If setting up the domain for use with RStudio, this value must be set to <code>Service</code>.</p>
    pub fn app_security_group_management(
        mut self,
        input: crate::types::AppSecurityGroupManagement,
    ) -> Self {
        self.app_security_group_management = ::std::option::Option::Some(input);
        self
    }
    /// <p>The entity that creates and manages the required security groups for inter-app communication in <code>VPCOnly</code> mode. Required when <code>CreateDomain.AppNetworkAccessType</code> is <code>VPCOnly</code> and <code>DomainSettings.RStudioServerProDomainSettings.DomainExecutionRoleArn</code> is provided. If setting up the domain for use with RStudio, this value must be set to <code>Service</code>.</p>
    pub fn set_app_security_group_management(
        mut self,
        input: ::std::option::Option<crate::types::AppSecurityGroupManagement>,
    ) -> Self {
        self.app_security_group_management = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateDomainInput`](crate::operation::update_domain::UpdateDomainInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_domain::UpdateDomainInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_domain::UpdateDomainInput {
            domain_id: self.domain_id,
            default_user_settings: self.default_user_settings,
            domain_settings_for_update: self.domain_settings_for_update,
            default_space_settings: self.default_space_settings,
            app_security_group_management: self.app_security_group_management,
        })
    }
}
