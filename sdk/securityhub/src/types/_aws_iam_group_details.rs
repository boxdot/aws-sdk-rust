// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains details about an IAM group.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsIamGroupDetails {
    /// <p>A list of the managed policies that are attached to the IAM group.</p>
    #[doc(hidden)]
    pub attached_managed_policies:
        ::std::option::Option<::std::vec::Vec<crate::types::AwsIamAttachedManagedPolicy>>,
    /// <p>Indicates when the IAM group was created.</p>
    /// <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces, and date and time should be separated by <code>T</code>. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[doc(hidden)]
    pub create_date: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the IAM group.</p>
    #[doc(hidden)]
    pub group_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the IAM group.</p>
    #[doc(hidden)]
    pub group_name: ::std::option::Option<::std::string::String>,
    /// <p>The list of inline policies that are embedded in the group.</p>
    #[doc(hidden)]
    pub group_policy_list: ::std::option::Option<::std::vec::Vec<crate::types::AwsIamGroupPolicy>>,
    /// <p>The path to the group.</p>
    #[doc(hidden)]
    pub path: ::std::option::Option<::std::string::String>,
}
impl AwsIamGroupDetails {
    /// <p>A list of the managed policies that are attached to the IAM group.</p>
    pub fn attached_managed_policies(
        &self,
    ) -> ::std::option::Option<&[crate::types::AwsIamAttachedManagedPolicy]> {
        self.attached_managed_policies.as_deref()
    }
    /// <p>Indicates when the IAM group was created.</p>
    /// <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces, and date and time should be separated by <code>T</code>. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    pub fn create_date(&self) -> ::std::option::Option<&str> {
        self.create_date.as_deref()
    }
    /// <p>The identifier of the IAM group.</p>
    pub fn group_id(&self) -> ::std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>The name of the IAM group.</p>
    pub fn group_name(&self) -> ::std::option::Option<&str> {
        self.group_name.as_deref()
    }
    /// <p>The list of inline policies that are embedded in the group.</p>
    pub fn group_policy_list(&self) -> ::std::option::Option<&[crate::types::AwsIamGroupPolicy]> {
        self.group_policy_list.as_deref()
    }
    /// <p>The path to the group.</p>
    pub fn path(&self) -> ::std::option::Option<&str> {
        self.path.as_deref()
    }
}
impl AwsIamGroupDetails {
    /// Creates a new builder-style object to manufacture [`AwsIamGroupDetails`](crate::types::AwsIamGroupDetails).
    pub fn builder() -> crate::types::builders::AwsIamGroupDetailsBuilder {
        crate::types::builders::AwsIamGroupDetailsBuilder::default()
    }
}

/// A builder for [`AwsIamGroupDetails`](crate::types::AwsIamGroupDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsIamGroupDetailsBuilder {
    pub(crate) attached_managed_policies:
        ::std::option::Option<::std::vec::Vec<crate::types::AwsIamAttachedManagedPolicy>>,
    pub(crate) create_date: ::std::option::Option<::std::string::String>,
    pub(crate) group_id: ::std::option::Option<::std::string::String>,
    pub(crate) group_name: ::std::option::Option<::std::string::String>,
    pub(crate) group_policy_list:
        ::std::option::Option<::std::vec::Vec<crate::types::AwsIamGroupPolicy>>,
    pub(crate) path: ::std::option::Option<::std::string::String>,
}
impl AwsIamGroupDetailsBuilder {
    /// Appends an item to `attached_managed_policies`.
    ///
    /// To override the contents of this collection use [`set_attached_managed_policies`](Self::set_attached_managed_policies).
    ///
    /// <p>A list of the managed policies that are attached to the IAM group.</p>
    pub fn attached_managed_policies(
        mut self,
        input: crate::types::AwsIamAttachedManagedPolicy,
    ) -> Self {
        let mut v = self.attached_managed_policies.unwrap_or_default();
        v.push(input);
        self.attached_managed_policies = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the managed policies that are attached to the IAM group.</p>
    pub fn set_attached_managed_policies(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AwsIamAttachedManagedPolicy>>,
    ) -> Self {
        self.attached_managed_policies = input;
        self
    }
    /// <p>Indicates when the IAM group was created.</p>
    /// <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces, and date and time should be separated by <code>T</code>. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    pub fn create_date(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.create_date = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Indicates when the IAM group was created.</p>
    /// <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces, and date and time should be separated by <code>T</code>. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    pub fn set_create_date(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.create_date = input;
        self
    }
    /// <p>The identifier of the IAM group.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the IAM group.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_id = input;
        self
    }
    /// <p>The name of the IAM group.</p>
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the IAM group.</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// Appends an item to `group_policy_list`.
    ///
    /// To override the contents of this collection use [`set_group_policy_list`](Self::set_group_policy_list).
    ///
    /// <p>The list of inline policies that are embedded in the group.</p>
    pub fn group_policy_list(mut self, input: crate::types::AwsIamGroupPolicy) -> Self {
        let mut v = self.group_policy_list.unwrap_or_default();
        v.push(input);
        self.group_policy_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of inline policies that are embedded in the group.</p>
    pub fn set_group_policy_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AwsIamGroupPolicy>>,
    ) -> Self {
        self.group_policy_list = input;
        self
    }
    /// <p>The path to the group.</p>
    pub fn path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The path to the group.</p>
    pub fn set_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.path = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsIamGroupDetails`](crate::types::AwsIamGroupDetails).
    pub fn build(self) -> crate::types::AwsIamGroupDetails {
        crate::types::AwsIamGroupDetails {
            attached_managed_policies: self.attached_managed_policies,
            create_date: self.create_date,
            group_id: self.group_id,
            group_name: self.group_name,
            group_policy_list: self.group_policy_list,
            path: self.path,
        }
    }
}
