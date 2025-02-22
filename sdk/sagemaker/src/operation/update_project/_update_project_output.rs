// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateProjectOutput {
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    #[doc(hidden)]
    pub project_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateProjectOutput {
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    pub fn project_arn(&self) -> ::std::option::Option<&str> {
        self.project_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for UpdateProjectOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateProjectOutput {
    /// Creates a new builder-style object to manufacture [`UpdateProjectOutput`](crate::operation::update_project::UpdateProjectOutput).
    pub fn builder() -> crate::operation::update_project::builders::UpdateProjectOutputBuilder {
        crate::operation::update_project::builders::UpdateProjectOutputBuilder::default()
    }
}

/// A builder for [`UpdateProjectOutput`](crate::operation::update_project::UpdateProjectOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateProjectOutputBuilder {
    pub(crate) project_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateProjectOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    pub fn project_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    pub fn set_project_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project_arn = input;
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
    /// Consumes the builder and constructs a [`UpdateProjectOutput`](crate::operation::update_project::UpdateProjectOutput).
    pub fn build(self) -> crate::operation::update_project::UpdateProjectOutput {
        crate::operation::update_project::UpdateProjectOutput {
            project_arn: self.project_arn,
            _request_id: self._request_id,
        }
    }
}
