// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about an AMI that is currently in the Recycle Bin.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImageRecycleBinInfo {
    /// <p>The ID of the AMI.</p>
    #[doc(hidden)]
    pub image_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the AMI.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The description of the AMI.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The date and time when the AMI entered the Recycle Bin.</p>
    #[doc(hidden)]
    pub recycle_bin_enter_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date and time when the AMI is to be permanently deleted from the Recycle Bin.</p>
    #[doc(hidden)]
    pub recycle_bin_exit_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ImageRecycleBinInfo {
    /// <p>The ID of the AMI.</p>
    pub fn image_id(&self) -> ::std::option::Option<&str> {
        self.image_id.as_deref()
    }
    /// <p>The name of the AMI.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The description of the AMI.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The date and time when the AMI entered the Recycle Bin.</p>
    pub fn recycle_bin_enter_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.recycle_bin_enter_time.as_ref()
    }
    /// <p>The date and time when the AMI is to be permanently deleted from the Recycle Bin.</p>
    pub fn recycle_bin_exit_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.recycle_bin_exit_time.as_ref()
    }
}
impl ImageRecycleBinInfo {
    /// Creates a new builder-style object to manufacture [`ImageRecycleBinInfo`](crate::types::ImageRecycleBinInfo).
    pub fn builder() -> crate::types::builders::ImageRecycleBinInfoBuilder {
        crate::types::builders::ImageRecycleBinInfoBuilder::default()
    }
}

/// A builder for [`ImageRecycleBinInfo`](crate::types::ImageRecycleBinInfo).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ImageRecycleBinInfoBuilder {
    pub(crate) image_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) recycle_bin_enter_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) recycle_bin_exit_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ImageRecycleBinInfoBuilder {
    /// <p>The ID of the AMI.</p>
    pub fn image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.image_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the AMI.</p>
    pub fn set_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.image_id = input;
        self
    }
    /// <p>The name of the AMI.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the AMI.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The description of the AMI.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the AMI.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The date and time when the AMI entered the Recycle Bin.</p>
    pub fn recycle_bin_enter_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.recycle_bin_enter_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the AMI entered the Recycle Bin.</p>
    pub fn set_recycle_bin_enter_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.recycle_bin_enter_time = input;
        self
    }
    /// <p>The date and time when the AMI is to be permanently deleted from the Recycle Bin.</p>
    pub fn recycle_bin_exit_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.recycle_bin_exit_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the AMI is to be permanently deleted from the Recycle Bin.</p>
    pub fn set_recycle_bin_exit_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.recycle_bin_exit_time = input;
        self
    }
    /// Consumes the builder and constructs a [`ImageRecycleBinInfo`](crate::types::ImageRecycleBinInfo).
    pub fn build(self) -> crate::types::ImageRecycleBinInfo {
        crate::types::ImageRecycleBinInfo {
            image_id: self.image_id,
            name: self.name,
            description: self.description,
            recycle_bin_enter_time: self.recycle_bin_enter_time,
            recycle_bin_exit_time: self.recycle_bin_exit_time,
        }
    }
}
