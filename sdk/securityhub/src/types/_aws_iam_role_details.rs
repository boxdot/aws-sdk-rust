// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about an IAM role, including all of the role's policies.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsIamRoleDetails {
    /// <p>The trust policy that grants permission to assume the role.</p>
    #[doc(hidden)]
    pub assume_role_policy_document: ::std::option::Option<::std::string::String>,
    /// <p>The list of the managed policies that are attached to the role.</p>
    #[doc(hidden)]
    pub attached_managed_policies:
        ::std::option::Option<::std::vec::Vec<crate::types::AwsIamAttachedManagedPolicy>>,
    /// <p>Indicates when the role was created.</p>
    /// <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces, and date and time should be separated by <code>T</code>. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    #[doc(hidden)]
    pub create_date: ::std::option::Option<::std::string::String>,
    /// <p>The list of instance profiles that contain this role.</p>
    #[doc(hidden)]
    pub instance_profile_list:
        ::std::option::Option<::std::vec::Vec<crate::types::AwsIamInstanceProfile>>,
    /// <p>Information about the policy used to set the permissions boundary for an IAM principal.</p>
    #[doc(hidden)]
    pub permissions_boundary: ::std::option::Option<crate::types::AwsIamPermissionsBoundary>,
    /// <p>The stable and unique string identifying the role.</p>
    #[doc(hidden)]
    pub role_id: ::std::option::Option<::std::string::String>,
    /// <p>The friendly name that identifies the role.</p>
    #[doc(hidden)]
    pub role_name: ::std::option::Option<::std::string::String>,
    /// <p>The list of inline policies that are embedded in the role.</p>
    #[doc(hidden)]
    pub role_policy_list: ::std::option::Option<::std::vec::Vec<crate::types::AwsIamRolePolicy>>,
    /// <p>The maximum session duration (in seconds) that you want to set for the specified role.</p>
    #[doc(hidden)]
    pub max_session_duration: i32,
    /// <p>The path to the role.</p>
    #[doc(hidden)]
    pub path: ::std::option::Option<::std::string::String>,
}
impl AwsIamRoleDetails {
    /// <p>The trust policy that grants permission to assume the role.</p>
    pub fn assume_role_policy_document(&self) -> ::std::option::Option<&str> {
        self.assume_role_policy_document.as_deref()
    }
    /// <p>The list of the managed policies that are attached to the role.</p>
    pub fn attached_managed_policies(
        &self,
    ) -> ::std::option::Option<&[crate::types::AwsIamAttachedManagedPolicy]> {
        self.attached_managed_policies.as_deref()
    }
    /// <p>Indicates when the role was created.</p>
    /// <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces, and date and time should be separated by <code>T</code>. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    pub fn create_date(&self) -> ::std::option::Option<&str> {
        self.create_date.as_deref()
    }
    /// <p>The list of instance profiles that contain this role.</p>
    pub fn instance_profile_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::AwsIamInstanceProfile]> {
        self.instance_profile_list.as_deref()
    }
    /// <p>Information about the policy used to set the permissions boundary for an IAM principal.</p>
    pub fn permissions_boundary(
        &self,
    ) -> ::std::option::Option<&crate::types::AwsIamPermissionsBoundary> {
        self.permissions_boundary.as_ref()
    }
    /// <p>The stable and unique string identifying the role.</p>
    pub fn role_id(&self) -> ::std::option::Option<&str> {
        self.role_id.as_deref()
    }
    /// <p>The friendly name that identifies the role.</p>
    pub fn role_name(&self) -> ::std::option::Option<&str> {
        self.role_name.as_deref()
    }
    /// <p>The list of inline policies that are embedded in the role.</p>
    pub fn role_policy_list(&self) -> ::std::option::Option<&[crate::types::AwsIamRolePolicy]> {
        self.role_policy_list.as_deref()
    }
    /// <p>The maximum session duration (in seconds) that you want to set for the specified role.</p>
    pub fn max_session_duration(&self) -> i32 {
        self.max_session_duration
    }
    /// <p>The path to the role.</p>
    pub fn path(&self) -> ::std::option::Option<&str> {
        self.path.as_deref()
    }
}
impl AwsIamRoleDetails {
    /// Creates a new builder-style object to manufacture [`AwsIamRoleDetails`](crate::types::AwsIamRoleDetails).
    pub fn builder() -> crate::types::builders::AwsIamRoleDetailsBuilder {
        crate::types::builders::AwsIamRoleDetailsBuilder::default()
    }
}

/// A builder for [`AwsIamRoleDetails`](crate::types::AwsIamRoleDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsIamRoleDetailsBuilder {
    pub(crate) assume_role_policy_document: ::std::option::Option<::std::string::String>,
    pub(crate) attached_managed_policies:
        ::std::option::Option<::std::vec::Vec<crate::types::AwsIamAttachedManagedPolicy>>,
    pub(crate) create_date: ::std::option::Option<::std::string::String>,
    pub(crate) instance_profile_list:
        ::std::option::Option<::std::vec::Vec<crate::types::AwsIamInstanceProfile>>,
    pub(crate) permissions_boundary: ::std::option::Option<crate::types::AwsIamPermissionsBoundary>,
    pub(crate) role_id: ::std::option::Option<::std::string::String>,
    pub(crate) role_name: ::std::option::Option<::std::string::String>,
    pub(crate) role_policy_list:
        ::std::option::Option<::std::vec::Vec<crate::types::AwsIamRolePolicy>>,
    pub(crate) max_session_duration: ::std::option::Option<i32>,
    pub(crate) path: ::std::option::Option<::std::string::String>,
}
impl AwsIamRoleDetailsBuilder {
    /// <p>The trust policy that grants permission to assume the role.</p>
    pub fn assume_role_policy_document(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.assume_role_policy_document = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The trust policy that grants permission to assume the role.</p>
    pub fn set_assume_role_policy_document(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.assume_role_policy_document = input;
        self
    }
    /// Appends an item to `attached_managed_policies`.
    ///
    /// To override the contents of this collection use [`set_attached_managed_policies`](Self::set_attached_managed_policies).
    ///
    /// <p>The list of the managed policies that are attached to the role.</p>
    pub fn attached_managed_policies(
        mut self,
        input: crate::types::AwsIamAttachedManagedPolicy,
    ) -> Self {
        let mut v = self.attached_managed_policies.unwrap_or_default();
        v.push(input);
        self.attached_managed_policies = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of the managed policies that are attached to the role.</p>
    pub fn set_attached_managed_policies(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AwsIamAttachedManagedPolicy>>,
    ) -> Self {
        self.attached_managed_policies = input;
        self
    }
    /// <p>Indicates when the role was created.</p>
    /// <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces, and date and time should be separated by <code>T</code>. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    pub fn create_date(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.create_date = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Indicates when the role was created.</p>
    /// <p>Uses the <code>date-time</code> format specified in <a href="https://tools.ietf.org/html/rfc3339#section-5.6">RFC 3339 section 5.6, Internet Date/Time Format</a>. The value cannot contain spaces, and date and time should be separated by <code>T</code>. For example, <code>2020-03-22T13:22:13.933Z</code>.</p>
    pub fn set_create_date(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.create_date = input;
        self
    }
    /// Appends an item to `instance_profile_list`.
    ///
    /// To override the contents of this collection use [`set_instance_profile_list`](Self::set_instance_profile_list).
    ///
    /// <p>The list of instance profiles that contain this role.</p>
    pub fn instance_profile_list(mut self, input: crate::types::AwsIamInstanceProfile) -> Self {
        let mut v = self.instance_profile_list.unwrap_or_default();
        v.push(input);
        self.instance_profile_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of instance profiles that contain this role.</p>
    pub fn set_instance_profile_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AwsIamInstanceProfile>>,
    ) -> Self {
        self.instance_profile_list = input;
        self
    }
    /// <p>Information about the policy used to set the permissions boundary for an IAM principal.</p>
    pub fn permissions_boundary(mut self, input: crate::types::AwsIamPermissionsBoundary) -> Self {
        self.permissions_boundary = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the policy used to set the permissions boundary for an IAM principal.</p>
    pub fn set_permissions_boundary(
        mut self,
        input: ::std::option::Option<crate::types::AwsIamPermissionsBoundary>,
    ) -> Self {
        self.permissions_boundary = input;
        self
    }
    /// <p>The stable and unique string identifying the role.</p>
    pub fn role_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The stable and unique string identifying the role.</p>
    pub fn set_role_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_id = input;
        self
    }
    /// <p>The friendly name that identifies the role.</p>
    pub fn role_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The friendly name that identifies the role.</p>
    pub fn set_role_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_name = input;
        self
    }
    /// Appends an item to `role_policy_list`.
    ///
    /// To override the contents of this collection use [`set_role_policy_list`](Self::set_role_policy_list).
    ///
    /// <p>The list of inline policies that are embedded in the role.</p>
    pub fn role_policy_list(mut self, input: crate::types::AwsIamRolePolicy) -> Self {
        let mut v = self.role_policy_list.unwrap_or_default();
        v.push(input);
        self.role_policy_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of inline policies that are embedded in the role.</p>
    pub fn set_role_policy_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AwsIamRolePolicy>>,
    ) -> Self {
        self.role_policy_list = input;
        self
    }
    /// <p>The maximum session duration (in seconds) that you want to set for the specified role.</p>
    pub fn max_session_duration(mut self, input: i32) -> Self {
        self.max_session_duration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum session duration (in seconds) that you want to set for the specified role.</p>
    pub fn set_max_session_duration(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_session_duration = input;
        self
    }
    /// <p>The path to the role.</p>
    pub fn path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The path to the role.</p>
    pub fn set_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.path = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsIamRoleDetails`](crate::types::AwsIamRoleDetails).
    pub fn build(self) -> crate::types::AwsIamRoleDetails {
        crate::types::AwsIamRoleDetails {
            assume_role_policy_document: self.assume_role_policy_document,
            attached_managed_policies: self.attached_managed_policies,
            create_date: self.create_date,
            instance_profile_list: self.instance_profile_list,
            permissions_boundary: self.permissions_boundary,
            role_id: self.role_id,
            role_name: self.role_name,
            role_policy_list: self.role_policy_list,
            max_session_duration: self.max_session_duration.unwrap_or_default(),
            path: self.path,
        }
    }
}
