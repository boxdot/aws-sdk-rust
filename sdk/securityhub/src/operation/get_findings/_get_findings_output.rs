// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetFindingsOutput {
    /// <p>The findings that matched the filters specified in the request.</p>
    #[doc(hidden)]
    pub findings: ::std::option::Option<::std::vec::Vec<crate::types::AwsSecurityFinding>>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetFindingsOutput {
    /// <p>The findings that matched the filters specified in the request.</p>
    pub fn findings(&self) -> ::std::option::Option<&[crate::types::AwsSecurityFinding]> {
        self.findings.as_deref()
    }
    /// <p>The pagination token to use to request the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetFindingsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetFindingsOutput {
    /// Creates a new builder-style object to manufacture [`GetFindingsOutput`](crate::operation::get_findings::GetFindingsOutput).
    pub fn builder() -> crate::operation::get_findings::builders::GetFindingsOutputBuilder {
        crate::operation::get_findings::builders::GetFindingsOutputBuilder::default()
    }
}

/// A builder for [`GetFindingsOutput`](crate::operation::get_findings::GetFindingsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetFindingsOutputBuilder {
    pub(crate) findings: ::std::option::Option<::std::vec::Vec<crate::types::AwsSecurityFinding>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetFindingsOutputBuilder {
    /// Appends an item to `findings`.
    ///
    /// To override the contents of this collection use [`set_findings`](Self::set_findings).
    ///
    /// <p>The findings that matched the filters specified in the request.</p>
    pub fn findings(mut self, input: crate::types::AwsSecurityFinding) -> Self {
        let mut v = self.findings.unwrap_or_default();
        v.push(input);
        self.findings = ::std::option::Option::Some(v);
        self
    }
    /// <p>The findings that matched the filters specified in the request.</p>
    pub fn set_findings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AwsSecurityFinding>>,
    ) -> Self {
        self.findings = input;
        self
    }
    /// <p>The pagination token to use to request the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token to use to request the next page of results.</p>
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
    /// Consumes the builder and constructs a [`GetFindingsOutput`](crate::operation::get_findings::GetFindingsOutput).
    pub fn build(self) -> crate::operation::get_findings::GetFindingsOutput {
        crate::operation::get_findings::GetFindingsOutput {
            findings: self.findings,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
