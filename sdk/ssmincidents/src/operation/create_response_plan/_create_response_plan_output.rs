// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateResponsePlanOutput {
    /// <p>The Amazon Resource Name (ARN) of the response plan.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateResponsePlanOutput {
    /// <p>The Amazon Resource Name (ARN) of the response plan.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateResponsePlanOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateResponsePlanOutput {
    /// Creates a new builder-style object to manufacture [`CreateResponsePlanOutput`](crate::operation::create_response_plan::CreateResponsePlanOutput).
    pub fn builder(
    ) -> crate::operation::create_response_plan::builders::CreateResponsePlanOutputBuilder {
        crate::operation::create_response_plan::builders::CreateResponsePlanOutputBuilder::default()
    }
}

/// A builder for [`CreateResponsePlanOutput`](crate::operation::create_response_plan::CreateResponsePlanOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateResponsePlanOutputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateResponsePlanOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the response plan.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the response plan.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
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
    /// Consumes the builder and constructs a [`CreateResponsePlanOutput`](crate::operation::create_response_plan::CreateResponsePlanOutput).
    pub fn build(self) -> crate::operation::create_response_plan::CreateResponsePlanOutput {
        crate::operation::create_response_plan::CreateResponsePlanOutput {
            arn: self.arn,
            _request_id: self._request_id,
        }
    }
}
