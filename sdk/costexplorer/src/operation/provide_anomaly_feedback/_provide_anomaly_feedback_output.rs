// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProvideAnomalyFeedbackOutput {
    /// <p>The ID of the modified cost anomaly. </p>
    #[doc(hidden)]
    pub anomaly_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ProvideAnomalyFeedbackOutput {
    /// <p>The ID of the modified cost anomaly. </p>
    pub fn anomaly_id(&self) -> ::std::option::Option<&str> {
        self.anomaly_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ProvideAnomalyFeedbackOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ProvideAnomalyFeedbackOutput {
    /// Creates a new builder-style object to manufacture [`ProvideAnomalyFeedbackOutput`](crate::operation::provide_anomaly_feedback::ProvideAnomalyFeedbackOutput).
    pub fn builder(
    ) -> crate::operation::provide_anomaly_feedback::builders::ProvideAnomalyFeedbackOutputBuilder
    {
        crate::operation::provide_anomaly_feedback::builders::ProvideAnomalyFeedbackOutputBuilder::default()
    }
}

/// A builder for [`ProvideAnomalyFeedbackOutput`](crate::operation::provide_anomaly_feedback::ProvideAnomalyFeedbackOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ProvideAnomalyFeedbackOutputBuilder {
    pub(crate) anomaly_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ProvideAnomalyFeedbackOutputBuilder {
    /// <p>The ID of the modified cost anomaly. </p>
    pub fn anomaly_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.anomaly_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the modified cost anomaly. </p>
    pub fn set_anomaly_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.anomaly_id = input;
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
    /// Consumes the builder and constructs a [`ProvideAnomalyFeedbackOutput`](crate::operation::provide_anomaly_feedback::ProvideAnomalyFeedbackOutput).
    pub fn build(self) -> crate::operation::provide_anomaly_feedback::ProvideAnomalyFeedbackOutput {
        crate::operation::provide_anomaly_feedback::ProvideAnomalyFeedbackOutput {
            anomaly_id: self.anomaly_id,
            _request_id: self._request_id,
        }
    }
}
