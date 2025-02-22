// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTokensOutput {
    /// <p>Received token details.</p>
    #[doc(hidden)]
    pub tokens: ::std::option::Option<::std::vec::Vec<crate::types::TokenData>>,
    /// <p>Token for the next set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTokensOutput {
    /// <p>Received token details.</p>
    pub fn tokens(&self) -> ::std::option::Option<&[crate::types::TokenData]> {
        self.tokens.as_deref()
    }
    /// <p>Token for the next set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListTokensOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTokensOutput {
    /// Creates a new builder-style object to manufacture [`ListTokensOutput`](crate::operation::list_tokens::ListTokensOutput).
    pub fn builder() -> crate::operation::list_tokens::builders::ListTokensOutputBuilder {
        crate::operation::list_tokens::builders::ListTokensOutputBuilder::default()
    }
}

/// A builder for [`ListTokensOutput`](crate::operation::list_tokens::ListTokensOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListTokensOutputBuilder {
    pub(crate) tokens: ::std::option::Option<::std::vec::Vec<crate::types::TokenData>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTokensOutputBuilder {
    /// Appends an item to `tokens`.
    ///
    /// To override the contents of this collection use [`set_tokens`](Self::set_tokens).
    ///
    /// <p>Received token details.</p>
    pub fn tokens(mut self, input: crate::types::TokenData) -> Self {
        let mut v = self.tokens.unwrap_or_default();
        v.push(input);
        self.tokens = ::std::option::Option::Some(v);
        self
    }
    /// <p>Received token details.</p>
    pub fn set_tokens(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TokenData>>,
    ) -> Self {
        self.tokens = input;
        self
    }
    /// <p>Token for the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Token for the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`ListTokensOutput`](crate::operation::list_tokens::ListTokensOutput).
    pub fn build(self) -> crate::operation::list_tokens::ListTokensOutput {
        crate::operation::list_tokens::ListTokensOutput {
            tokens: self.tokens,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
