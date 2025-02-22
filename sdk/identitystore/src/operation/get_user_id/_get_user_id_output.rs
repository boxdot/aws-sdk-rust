// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetUserIdOutput {
    /// <p>The identifier for a user in the identity store.</p>
    #[doc(hidden)]
    pub user_id: ::std::option::Option<::std::string::String>,
    /// <p>The globally unique identifier for the identity store.</p>
    #[doc(hidden)]
    pub identity_store_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetUserIdOutput {
    /// <p>The identifier for a user in the identity store.</p>
    pub fn user_id(&self) -> ::std::option::Option<&str> {
        self.user_id.as_deref()
    }
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn identity_store_id(&self) -> ::std::option::Option<&str> {
        self.identity_store_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetUserIdOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetUserIdOutput {
    /// Creates a new builder-style object to manufacture [`GetUserIdOutput`](crate::operation::get_user_id::GetUserIdOutput).
    pub fn builder() -> crate::operation::get_user_id::builders::GetUserIdOutputBuilder {
        crate::operation::get_user_id::builders::GetUserIdOutputBuilder::default()
    }
}

/// A builder for [`GetUserIdOutput`](crate::operation::get_user_id::GetUserIdOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetUserIdOutputBuilder {
    pub(crate) user_id: ::std::option::Option<::std::string::String>,
    pub(crate) identity_store_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetUserIdOutputBuilder {
    /// <p>The identifier for a user in the identity store.</p>
    pub fn user_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for a user in the identity store.</p>
    pub fn set_user_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_id = input;
        self
    }
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn identity_store_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.identity_store_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn set_identity_store_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.identity_store_id = input;
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
    /// Consumes the builder and constructs a [`GetUserIdOutput`](crate::operation::get_user_id::GetUserIdOutput).
    pub fn build(self) -> crate::operation::get_user_id::GetUserIdOutput {
        crate::operation::get_user_id::GetUserIdOutput {
            user_id: self.user_id,
            identity_store_id: self.identity_store_id,
            _request_id: self._request_id,
        }
    }
}
