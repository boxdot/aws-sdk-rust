// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Account level Launch Configuration Template.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct LaunchConfigurationTemplate {
    /// <p>ID of the Launch Configuration Template.</p>
    #[doc(hidden)]
    pub launch_configuration_template_id: ::std::option::Option<::std::string::String>,
    /// <p>ARN of the Launch Configuration Template.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>Tags of the Launch Configuration Template.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>Launch disposition.</p>
    #[doc(hidden)]
    pub launch_disposition: ::std::option::Option<crate::types::LaunchDisposition>,
    /// <p>Target instance type right-sizing method.</p>
    #[doc(hidden)]
    pub target_instance_type_right_sizing_method:
        ::std::option::Option<crate::types::TargetInstanceTypeRightSizingMethod>,
    /// <p>Copy private IP.</p>
    #[doc(hidden)]
    pub copy_private_ip: ::std::option::Option<bool>,
    /// <p>Copy tags.</p>
    #[doc(hidden)]
    pub copy_tags: ::std::option::Option<bool>,
    /// <p>Licensing.</p>
    #[doc(hidden)]
    pub licensing: ::std::option::Option<crate::types::Licensing>,
}
impl LaunchConfigurationTemplate {
    /// <p>ID of the Launch Configuration Template.</p>
    pub fn launch_configuration_template_id(&self) -> ::std::option::Option<&str> {
        self.launch_configuration_template_id.as_deref()
    }
    /// <p>ARN of the Launch Configuration Template.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>Tags of the Launch Configuration Template.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
    /// <p>Launch disposition.</p>
    pub fn launch_disposition(&self) -> ::std::option::Option<&crate::types::LaunchDisposition> {
        self.launch_disposition.as_ref()
    }
    /// <p>Target instance type right-sizing method.</p>
    pub fn target_instance_type_right_sizing_method(
        &self,
    ) -> ::std::option::Option<&crate::types::TargetInstanceTypeRightSizingMethod> {
        self.target_instance_type_right_sizing_method.as_ref()
    }
    /// <p>Copy private IP.</p>
    pub fn copy_private_ip(&self) -> ::std::option::Option<bool> {
        self.copy_private_ip
    }
    /// <p>Copy tags.</p>
    pub fn copy_tags(&self) -> ::std::option::Option<bool> {
        self.copy_tags
    }
    /// <p>Licensing.</p>
    pub fn licensing(&self) -> ::std::option::Option<&crate::types::Licensing> {
        self.licensing.as_ref()
    }
}
impl ::std::fmt::Debug for LaunchConfigurationTemplate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("LaunchConfigurationTemplate");
        formatter.field(
            "launch_configuration_template_id",
            &self.launch_configuration_template_id,
        );
        formatter.field("arn", &self.arn);
        formatter.field("tags", &"*** Sensitive Data Redacted ***");
        formatter.field("launch_disposition", &self.launch_disposition);
        formatter.field(
            "target_instance_type_right_sizing_method",
            &self.target_instance_type_right_sizing_method,
        );
        formatter.field("copy_private_ip", &self.copy_private_ip);
        formatter.field("copy_tags", &self.copy_tags);
        formatter.field("licensing", &self.licensing);
        formatter.finish()
    }
}
impl LaunchConfigurationTemplate {
    /// Creates a new builder-style object to manufacture [`LaunchConfigurationTemplate`](crate::types::LaunchConfigurationTemplate).
    pub fn builder() -> crate::types::builders::LaunchConfigurationTemplateBuilder {
        crate::types::builders::LaunchConfigurationTemplateBuilder::default()
    }
}

/// A builder for [`LaunchConfigurationTemplate`](crate::types::LaunchConfigurationTemplate).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct LaunchConfigurationTemplateBuilder {
    pub(crate) launch_configuration_template_id: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) launch_disposition: ::std::option::Option<crate::types::LaunchDisposition>,
    pub(crate) target_instance_type_right_sizing_method:
        ::std::option::Option<crate::types::TargetInstanceTypeRightSizingMethod>,
    pub(crate) copy_private_ip: ::std::option::Option<bool>,
    pub(crate) copy_tags: ::std::option::Option<bool>,
    pub(crate) licensing: ::std::option::Option<crate::types::Licensing>,
}
impl LaunchConfigurationTemplateBuilder {
    /// <p>ID of the Launch Configuration Template.</p>
    pub fn launch_configuration_template_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.launch_configuration_template_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ID of the Launch Configuration Template.</p>
    pub fn set_launch_configuration_template_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.launch_configuration_template_id = input;
        self
    }
    /// <p>ARN of the Launch Configuration Template.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ARN of the Launch Configuration Template.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags of the Launch Configuration Template.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Tags of the Launch Configuration Template.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>Launch disposition.</p>
    pub fn launch_disposition(mut self, input: crate::types::LaunchDisposition) -> Self {
        self.launch_disposition = ::std::option::Option::Some(input);
        self
    }
    /// <p>Launch disposition.</p>
    pub fn set_launch_disposition(
        mut self,
        input: ::std::option::Option<crate::types::LaunchDisposition>,
    ) -> Self {
        self.launch_disposition = input;
        self
    }
    /// <p>Target instance type right-sizing method.</p>
    pub fn target_instance_type_right_sizing_method(
        mut self,
        input: crate::types::TargetInstanceTypeRightSizingMethod,
    ) -> Self {
        self.target_instance_type_right_sizing_method = ::std::option::Option::Some(input);
        self
    }
    /// <p>Target instance type right-sizing method.</p>
    pub fn set_target_instance_type_right_sizing_method(
        mut self,
        input: ::std::option::Option<crate::types::TargetInstanceTypeRightSizingMethod>,
    ) -> Self {
        self.target_instance_type_right_sizing_method = input;
        self
    }
    /// <p>Copy private IP.</p>
    pub fn copy_private_ip(mut self, input: bool) -> Self {
        self.copy_private_ip = ::std::option::Option::Some(input);
        self
    }
    /// <p>Copy private IP.</p>
    pub fn set_copy_private_ip(mut self, input: ::std::option::Option<bool>) -> Self {
        self.copy_private_ip = input;
        self
    }
    /// <p>Copy tags.</p>
    pub fn copy_tags(mut self, input: bool) -> Self {
        self.copy_tags = ::std::option::Option::Some(input);
        self
    }
    /// <p>Copy tags.</p>
    pub fn set_copy_tags(mut self, input: ::std::option::Option<bool>) -> Self {
        self.copy_tags = input;
        self
    }
    /// <p>Licensing.</p>
    pub fn licensing(mut self, input: crate::types::Licensing) -> Self {
        self.licensing = ::std::option::Option::Some(input);
        self
    }
    /// <p>Licensing.</p>
    pub fn set_licensing(mut self, input: ::std::option::Option<crate::types::Licensing>) -> Self {
        self.licensing = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchConfigurationTemplate`](crate::types::LaunchConfigurationTemplate).
    pub fn build(self) -> crate::types::LaunchConfigurationTemplate {
        crate::types::LaunchConfigurationTemplate {
            launch_configuration_template_id: self.launch_configuration_template_id,
            arn: self.arn,
            tags: self.tags,
            launch_disposition: self.launch_disposition,
            target_instance_type_right_sizing_method: self.target_instance_type_right_sizing_method,
            copy_private_ip: self.copy_private_ip,
            copy_tags: self.copy_tags,
            licensing: self.licensing,
        }
    }
}
impl ::std::fmt::Debug for LaunchConfigurationTemplateBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("LaunchConfigurationTemplateBuilder");
        formatter.field(
            "launch_configuration_template_id",
            &self.launch_configuration_template_id,
        );
        formatter.field("arn", &self.arn);
        formatter.field("tags", &"*** Sensitive Data Redacted ***");
        formatter.field("launch_disposition", &self.launch_disposition);
        formatter.field(
            "target_instance_type_right_sizing_method",
            &self.target_instance_type_right_sizing_method,
        );
        formatter.field("copy_private_ip", &self.copy_private_ip);
        formatter.field("copy_tags", &self.copy_tags);
        formatter.field("licensing", &self.licensing);
        formatter.finish()
    }
}
