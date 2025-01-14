// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An option group membership.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsRdsDbOptionGroupMembership {
    /// <p>The name of the option group.</p>
    #[doc(hidden)]
    pub option_group_name: ::std::option::Option<::std::string::String>,
    /// <p>The status of the option group membership.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<::std::string::String>,
}
impl AwsRdsDbOptionGroupMembership {
    /// <p>The name of the option group.</p>
    pub fn option_group_name(&self) -> ::std::option::Option<&str> {
        self.option_group_name.as_deref()
    }
    /// <p>The status of the option group membership.</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
}
impl AwsRdsDbOptionGroupMembership {
    /// Creates a new builder-style object to manufacture [`AwsRdsDbOptionGroupMembership`](crate::types::AwsRdsDbOptionGroupMembership).
    pub fn builder() -> crate::types::builders::AwsRdsDbOptionGroupMembershipBuilder {
        crate::types::builders::AwsRdsDbOptionGroupMembershipBuilder::default()
    }
}

/// A builder for [`AwsRdsDbOptionGroupMembership`](crate::types::AwsRdsDbOptionGroupMembership).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsRdsDbOptionGroupMembershipBuilder {
    pub(crate) option_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
}
impl AwsRdsDbOptionGroupMembershipBuilder {
    /// <p>The name of the option group.</p>
    pub fn option_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.option_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the option group.</p>
    pub fn set_option_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.option_group_name = input;
        self
    }
    /// <p>The status of the option group membership.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The status of the option group membership.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsRdsDbOptionGroupMembership`](crate::types::AwsRdsDbOptionGroupMembership).
    pub fn build(self) -> crate::types::AwsRdsDbOptionGroupMembership {
        crate::types::AwsRdsDbOptionGroupMembership {
            option_group_name: self.option_group_name,
            status: self.status,
        }
    }
}
