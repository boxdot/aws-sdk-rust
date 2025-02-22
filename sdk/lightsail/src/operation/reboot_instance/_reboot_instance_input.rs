// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RebootInstanceInput {
    /// <p>The name of the instance to reboot.</p>
    #[doc(hidden)]
    pub instance_name: ::std::option::Option<::std::string::String>,
}
impl RebootInstanceInput {
    /// <p>The name of the instance to reboot.</p>
    pub fn instance_name(&self) -> ::std::option::Option<&str> {
        self.instance_name.as_deref()
    }
}
impl RebootInstanceInput {
    /// Creates a new builder-style object to manufacture [`RebootInstanceInput`](crate::operation::reboot_instance::RebootInstanceInput).
    pub fn builder() -> crate::operation::reboot_instance::builders::RebootInstanceInputBuilder {
        crate::operation::reboot_instance::builders::RebootInstanceInputBuilder::default()
    }
}

/// A builder for [`RebootInstanceInput`](crate::operation::reboot_instance::RebootInstanceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RebootInstanceInputBuilder {
    pub(crate) instance_name: ::std::option::Option<::std::string::String>,
}
impl RebootInstanceInputBuilder {
    /// <p>The name of the instance to reboot.</p>
    pub fn instance_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.instance_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the instance to reboot.</p>
    pub fn set_instance_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.instance_name = input;
        self
    }
    /// Consumes the builder and constructs a [`RebootInstanceInput`](crate::operation::reboot_instance::RebootInstanceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::reboot_instance::RebootInstanceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::reboot_instance::RebootInstanceInput {
            instance_name: self.instance_name,
        })
    }
}
