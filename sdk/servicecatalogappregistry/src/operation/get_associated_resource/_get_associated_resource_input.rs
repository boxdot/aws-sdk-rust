// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAssociatedResourceInput {
    /// <p> The name, ID, or ARN of the application. </p>
    #[doc(hidden)]
    pub application: ::std::option::Option<::std::string::String>,
    /// <p>The type of resource associated with the application.</p>
    #[doc(hidden)]
    pub resource_type: ::std::option::Option<crate::types::ResourceType>,
    /// <p>The name or ID of the resource associated with the application.</p>
    #[doc(hidden)]
    pub resource: ::std::option::Option<::std::string::String>,
}
impl GetAssociatedResourceInput {
    /// <p> The name, ID, or ARN of the application. </p>
    pub fn application(&self) -> ::std::option::Option<&str> {
        self.application.as_deref()
    }
    /// <p>The type of resource associated with the application.</p>
    pub fn resource_type(&self) -> ::std::option::Option<&crate::types::ResourceType> {
        self.resource_type.as_ref()
    }
    /// <p>The name or ID of the resource associated with the application.</p>
    pub fn resource(&self) -> ::std::option::Option<&str> {
        self.resource.as_deref()
    }
}
impl GetAssociatedResourceInput {
    /// Creates a new builder-style object to manufacture [`GetAssociatedResourceInput`](crate::operation::get_associated_resource::GetAssociatedResourceInput).
    pub fn builder(
    ) -> crate::operation::get_associated_resource::builders::GetAssociatedResourceInputBuilder
    {
        crate::operation::get_associated_resource::builders::GetAssociatedResourceInputBuilder::default()
    }
}

/// A builder for [`GetAssociatedResourceInput`](crate::operation::get_associated_resource::GetAssociatedResourceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetAssociatedResourceInputBuilder {
    pub(crate) application: ::std::option::Option<::std::string::String>,
    pub(crate) resource_type: ::std::option::Option<crate::types::ResourceType>,
    pub(crate) resource: ::std::option::Option<::std::string::String>,
}
impl GetAssociatedResourceInputBuilder {
    /// <p> The name, ID, or ARN of the application. </p>
    pub fn application(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.application = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name, ID, or ARN of the application. </p>
    pub fn set_application(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.application = input;
        self
    }
    /// <p>The type of resource associated with the application.</p>
    pub fn resource_type(mut self, input: crate::types::ResourceType) -> Self {
        self.resource_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of resource associated with the application.</p>
    pub fn set_resource_type(
        mut self,
        input: ::std::option::Option<crate::types::ResourceType>,
    ) -> Self {
        self.resource_type = input;
        self
    }
    /// <p>The name or ID of the resource associated with the application.</p>
    pub fn resource(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name or ID of the resource associated with the application.</p>
    pub fn set_resource(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource = input;
        self
    }
    /// Consumes the builder and constructs a [`GetAssociatedResourceInput`](crate::operation::get_associated_resource::GetAssociatedResourceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_associated_resource::GetAssociatedResourceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_associated_resource::GetAssociatedResourceInput {
                application: self.application,
                resource_type: self.resource_type,
                resource: self.resource,
            },
        )
    }
}
