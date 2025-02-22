// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StopTrainingJobOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for StopTrainingJobOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StopTrainingJobOutput {
    /// Creates a new builder-style object to manufacture [`StopTrainingJobOutput`](crate::operation::stop_training_job::StopTrainingJobOutput).
    pub fn builder() -> crate::operation::stop_training_job::builders::StopTrainingJobOutputBuilder
    {
        crate::operation::stop_training_job::builders::StopTrainingJobOutputBuilder::default()
    }
}

/// A builder for [`StopTrainingJobOutput`](crate::operation::stop_training_job::StopTrainingJobOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StopTrainingJobOutputBuilder {
    _request_id: Option<String>,
}
impl StopTrainingJobOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StopTrainingJobOutput`](crate::operation::stop_training_job::StopTrainingJobOutput).
    pub fn build(self) -> crate::operation::stop_training_job::StopTrainingJobOutput {
        crate::operation::stop_training_job::StopTrainingJobOutput {
            _request_id: self._request_id,
        }
    }
}
