// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The launch template to use. You must specify either the launch template ID or launch template name in the request, but not both.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LaunchTemplateSpecification {
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    #[doc(hidden)]
    pub launch_template_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    #[doc(hidden)]
    pub launch_template_name: ::std::option::Option<::std::string::String>,
    /// <p>The launch template version number, <code>$Latest</code>, or <code>$Default</code>.</p>
    /// <p>If the value is <code>$Latest</code>, Amazon EC2 uses the latest version of the launch template.</p>
    /// <p>If the value is <code>$Default</code>, Amazon EC2 uses the default version of the launch template.</p>
    /// <p>Default: The default version of the launch template.</p>
    #[doc(hidden)]
    pub version: ::std::option::Option<::std::string::String>,
}
impl LaunchTemplateSpecification {
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    pub fn launch_template_id(&self) -> ::std::option::Option<&str> {
        self.launch_template_id.as_deref()
    }
    /// <p>The name of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    pub fn launch_template_name(&self) -> ::std::option::Option<&str> {
        self.launch_template_name.as_deref()
    }
    /// <p>The launch template version number, <code>$Latest</code>, or <code>$Default</code>.</p>
    /// <p>If the value is <code>$Latest</code>, Amazon EC2 uses the latest version of the launch template.</p>
    /// <p>If the value is <code>$Default</code>, Amazon EC2 uses the default version of the launch template.</p>
    /// <p>Default: The default version of the launch template.</p>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
}
impl LaunchTemplateSpecification {
    /// Creates a new builder-style object to manufacture [`LaunchTemplateSpecification`](crate::types::LaunchTemplateSpecification).
    pub fn builder() -> crate::types::builders::LaunchTemplateSpecificationBuilder {
        crate::types::builders::LaunchTemplateSpecificationBuilder::default()
    }
}

/// A builder for [`LaunchTemplateSpecification`](crate::types::LaunchTemplateSpecification).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LaunchTemplateSpecificationBuilder {
    pub(crate) launch_template_id: ::std::option::Option<::std::string::String>,
    pub(crate) launch_template_name: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
}
impl LaunchTemplateSpecificationBuilder {
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    pub fn launch_template_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.launch_template_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    pub fn set_launch_template_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.launch_template_id = input;
        self
    }
    /// <p>The name of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    pub fn launch_template_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.launch_template_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    pub fn set_launch_template_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.launch_template_name = input;
        self
    }
    /// <p>The launch template version number, <code>$Latest</code>, or <code>$Default</code>.</p>
    /// <p>If the value is <code>$Latest</code>, Amazon EC2 uses the latest version of the launch template.</p>
    /// <p>If the value is <code>$Default</code>, Amazon EC2 uses the default version of the launch template.</p>
    /// <p>Default: The default version of the launch template.</p>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The launch template version number, <code>$Latest</code>, or <code>$Default</code>.</p>
    /// <p>If the value is <code>$Latest</code>, Amazon EC2 uses the latest version of the launch template.</p>
    /// <p>If the value is <code>$Default</code>, Amazon EC2 uses the default version of the launch template.</p>
    /// <p>Default: The default version of the launch template.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchTemplateSpecification`](crate::types::LaunchTemplateSpecification).
    pub fn build(self) -> crate::types::LaunchTemplateSpecification {
        crate::types::LaunchTemplateSpecification {
            launch_template_id: self.launch_template_id,
            launch_template_name: self.launch_template_name,
            version: self.version,
        }
    }
}
