// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPortalServiceProviderMetadataInput {
    /// <p>The ARN of the web portal.</p>
    #[doc(hidden)]
    pub portal_arn: ::std::option::Option<::std::string::String>,
}
impl GetPortalServiceProviderMetadataInput {
    /// <p>The ARN of the web portal.</p>
    pub fn portal_arn(&self) -> ::std::option::Option<&str> {
        self.portal_arn.as_deref()
    }
}
impl GetPortalServiceProviderMetadataInput {
    /// Creates a new builder-style object to manufacture [`GetPortalServiceProviderMetadataInput`](crate::operation::get_portal_service_provider_metadata::GetPortalServiceProviderMetadataInput).
    pub fn builder() -> crate::operation::get_portal_service_provider_metadata::builders::GetPortalServiceProviderMetadataInputBuilder{
        crate::operation::get_portal_service_provider_metadata::builders::GetPortalServiceProviderMetadataInputBuilder::default()
    }
}

/// A builder for [`GetPortalServiceProviderMetadataInput`](crate::operation::get_portal_service_provider_metadata::GetPortalServiceProviderMetadataInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetPortalServiceProviderMetadataInputBuilder {
    pub(crate) portal_arn: ::std::option::Option<::std::string::String>,
}
impl GetPortalServiceProviderMetadataInputBuilder {
    /// <p>The ARN of the web portal.</p>
    pub fn portal_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.portal_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the web portal.</p>
    pub fn set_portal_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.portal_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`GetPortalServiceProviderMetadataInput`](crate::operation::get_portal_service_provider_metadata::GetPortalServiceProviderMetadataInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_portal_service_provider_metadata::GetPortalServiceProviderMetadataInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::get_portal_service_provider_metadata::GetPortalServiceProviderMetadataInput {
                portal_arn: self.portal_arn
                ,
            }
        )
    }
}
