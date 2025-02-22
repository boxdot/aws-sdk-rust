// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A value that indicates the AMI information.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CustomDbEngineVersionAmi {
    /// <p>A value that indicates the ID of the AMI.</p>
    #[doc(hidden)]
    pub image_id: ::std::option::Option<::std::string::String>,
    /// <p>A value that indicates the status of a custom engine version (CEV).</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<::std::string::String>,
}
impl CustomDbEngineVersionAmi {
    /// <p>A value that indicates the ID of the AMI.</p>
    pub fn image_id(&self) -> ::std::option::Option<&str> {
        self.image_id.as_deref()
    }
    /// <p>A value that indicates the status of a custom engine version (CEV).</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
}
impl CustomDbEngineVersionAmi {
    /// Creates a new builder-style object to manufacture [`CustomDbEngineVersionAmi`](crate::types::CustomDbEngineVersionAmi).
    pub fn builder() -> crate::types::builders::CustomDbEngineVersionAmiBuilder {
        crate::types::builders::CustomDbEngineVersionAmiBuilder::default()
    }
}

/// A builder for [`CustomDbEngineVersionAmi`](crate::types::CustomDbEngineVersionAmi).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CustomDbEngineVersionAmiBuilder {
    pub(crate) image_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
}
impl CustomDbEngineVersionAmiBuilder {
    /// <p>A value that indicates the ID of the AMI.</p>
    pub fn image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.image_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A value that indicates the ID of the AMI.</p>
    pub fn set_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.image_id = input;
        self
    }
    /// <p>A value that indicates the status of a custom engine version (CEV).</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A value that indicates the status of a custom engine version (CEV).</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`CustomDbEngineVersionAmi`](crate::types::CustomDbEngineVersionAmi).
    pub fn build(self) -> crate::types::CustomDbEngineVersionAmi {
        crate::types::CustomDbEngineVersionAmi {
            image_id: self.image_id,
            status: self.status,
        }
    }
}
