// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchSuspendUserOutput {
    /// <p>If the <code>BatchSuspendUser</code> action fails for one or more of the user IDs in the request, a list of the user IDs is returned, along with error codes and error messages.</p>
    #[doc(hidden)]
    pub user_errors: ::std::option::Option<::std::vec::Vec<crate::types::UserError>>,
    _request_id: Option<String>,
}
impl BatchSuspendUserOutput {
    /// <p>If the <code>BatchSuspendUser</code> action fails for one or more of the user IDs in the request, a list of the user IDs is returned, along with error codes and error messages.</p>
    pub fn user_errors(&self) -> ::std::option::Option<&[crate::types::UserError]> {
        self.user_errors.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for BatchSuspendUserOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchSuspendUserOutput {
    /// Creates a new builder-style object to manufacture [`BatchSuspendUserOutput`](crate::operation::batch_suspend_user::BatchSuspendUserOutput).
    pub fn builder() -> crate::operation::batch_suspend_user::builders::BatchSuspendUserOutputBuilder
    {
        crate::operation::batch_suspend_user::builders::BatchSuspendUserOutputBuilder::default()
    }
}

/// A builder for [`BatchSuspendUserOutput`](crate::operation::batch_suspend_user::BatchSuspendUserOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchSuspendUserOutputBuilder {
    pub(crate) user_errors: ::std::option::Option<::std::vec::Vec<crate::types::UserError>>,
    _request_id: Option<String>,
}
impl BatchSuspendUserOutputBuilder {
    /// Appends an item to `user_errors`.
    ///
    /// To override the contents of this collection use [`set_user_errors`](Self::set_user_errors).
    ///
    /// <p>If the <code>BatchSuspendUser</code> action fails for one or more of the user IDs in the request, a list of the user IDs is returned, along with error codes and error messages.</p>
    pub fn user_errors(mut self, input: crate::types::UserError) -> Self {
        let mut v = self.user_errors.unwrap_or_default();
        v.push(input);
        self.user_errors = ::std::option::Option::Some(v);
        self
    }
    /// <p>If the <code>BatchSuspendUser</code> action fails for one or more of the user IDs in the request, a list of the user IDs is returned, along with error codes and error messages.</p>
    pub fn set_user_errors(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::UserError>>,
    ) -> Self {
        self.user_errors = input;
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
    /// Consumes the builder and constructs a [`BatchSuspendUserOutput`](crate::operation::batch_suspend_user::BatchSuspendUserOutput).
    pub fn build(self) -> crate::operation::batch_suspend_user::BatchSuspendUserOutput {
        crate::operation::batch_suspend_user::BatchSuspendUserOutput {
            user_errors: self.user_errors,
            _request_id: self._request_id,
        }
    }
}
