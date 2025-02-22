// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateConferenceProviderOutput {
    /// <p>The ARN of the newly-created conference provider.</p>
    #[doc(hidden)]
    pub conference_provider_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateConferenceProviderOutput {
    /// <p>The ARN of the newly-created conference provider.</p>
    pub fn conference_provider_arn(&self) -> ::std::option::Option<&str> {
        self.conference_provider_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateConferenceProviderOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateConferenceProviderOutput {
    /// Creates a new builder-style object to manufacture [`CreateConferenceProviderOutput`](crate::operation::create_conference_provider::CreateConferenceProviderOutput).
    pub fn builder(
    ) -> crate::operation::create_conference_provider::builders::CreateConferenceProviderOutputBuilder
    {
        crate::operation::create_conference_provider::builders::CreateConferenceProviderOutputBuilder::default()
    }
}

/// A builder for [`CreateConferenceProviderOutput`](crate::operation::create_conference_provider::CreateConferenceProviderOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateConferenceProviderOutputBuilder {
    pub(crate) conference_provider_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateConferenceProviderOutputBuilder {
    /// <p>The ARN of the newly-created conference provider.</p>
    pub fn conference_provider_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.conference_provider_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the newly-created conference provider.</p>
    pub fn set_conference_provider_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.conference_provider_arn = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateConferenceProviderOutput`](crate::operation::create_conference_provider::CreateConferenceProviderOutput).
    pub fn build(
        self,
    ) -> crate::operation::create_conference_provider::CreateConferenceProviderOutput {
        crate::operation::create_conference_provider::CreateConferenceProviderOutput {
            conference_provider_arn: self.conference_provider_arn,
            _request_id: self._request_id,
        }
    }
}
