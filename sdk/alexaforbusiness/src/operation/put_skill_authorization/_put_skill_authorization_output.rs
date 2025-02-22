// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutSkillAuthorizationOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for PutSkillAuthorizationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutSkillAuthorizationOutput {
    /// Creates a new builder-style object to manufacture [`PutSkillAuthorizationOutput`](crate::operation::put_skill_authorization::PutSkillAuthorizationOutput).
    pub fn builder(
    ) -> crate::operation::put_skill_authorization::builders::PutSkillAuthorizationOutputBuilder
    {
        crate::operation::put_skill_authorization::builders::PutSkillAuthorizationOutputBuilder::default()
    }
}

/// A builder for [`PutSkillAuthorizationOutput`](crate::operation::put_skill_authorization::PutSkillAuthorizationOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutSkillAuthorizationOutputBuilder {
    _request_id: Option<String>,
}
impl PutSkillAuthorizationOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`PutSkillAuthorizationOutput`](crate::operation::put_skill_authorization::PutSkillAuthorizationOutput).
    pub fn build(self) -> crate::operation::put_skill_authorization::PutSkillAuthorizationOutput {
        crate::operation::put_skill_authorization::PutSkillAuthorizationOutput {
            _request_id: self._request_id,
        }
    }
}
