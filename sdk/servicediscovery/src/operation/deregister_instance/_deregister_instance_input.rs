// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeregisterInstanceInput {
    /// <p>The ID of the service that the instance is associated with.</p>
    #[doc(hidden)]
    pub service_id: ::std::option::Option<::std::string::String>,
    /// <p>The value that you specified for <code>Id</code> in the <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html">RegisterInstance</a> request.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
}
impl DeregisterInstanceInput {
    /// <p>The ID of the service that the instance is associated with.</p>
    pub fn service_id(&self) -> ::std::option::Option<&str> {
        self.service_id.as_deref()
    }
    /// <p>The value that you specified for <code>Id</code> in the <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html">RegisterInstance</a> request.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
}
impl DeregisterInstanceInput {
    /// Creates a new builder-style object to manufacture [`DeregisterInstanceInput`](crate::operation::deregister_instance::DeregisterInstanceInput).
    pub fn builder(
    ) -> crate::operation::deregister_instance::builders::DeregisterInstanceInputBuilder {
        crate::operation::deregister_instance::builders::DeregisterInstanceInputBuilder::default()
    }
}

/// A builder for [`DeregisterInstanceInput`](crate::operation::deregister_instance::DeregisterInstanceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeregisterInstanceInputBuilder {
    pub(crate) service_id: ::std::option::Option<::std::string::String>,
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
}
impl DeregisterInstanceInputBuilder {
    /// <p>The ID of the service that the instance is associated with.</p>
    pub fn service_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the service that the instance is associated with.</p>
    pub fn set_service_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_id = input;
        self
    }
    /// <p>The value that you specified for <code>Id</code> in the <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html">RegisterInstance</a> request.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value that you specified for <code>Id</code> in the <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html">RegisterInstance</a> request.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeregisterInstanceInput`](crate::operation::deregister_instance::DeregisterInstanceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::deregister_instance::DeregisterInstanceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::deregister_instance::DeregisterInstanceInput {
                service_id: self.service_id,
                instance_id: self.instance_id,
            },
        )
    }
}
