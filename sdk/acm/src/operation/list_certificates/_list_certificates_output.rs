// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListCertificatesOutput {
    /// <p>When the list is truncated, this value is present and contains the value to use for the <code>NextToken</code> parameter in a subsequent pagination request.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>A list of ACM certificates.</p>
    #[doc(hidden)]
    pub certificate_summary_list:
        ::std::option::Option<::std::vec::Vec<crate::types::CertificateSummary>>,
    _request_id: Option<String>,
}
impl ListCertificatesOutput {
    /// <p>When the list is truncated, this value is present and contains the value to use for the <code>NextToken</code> parameter in a subsequent pagination request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>A list of ACM certificates.</p>
    pub fn certificate_summary_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::CertificateSummary]> {
        self.certificate_summary_list.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListCertificatesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListCertificatesOutput {
    /// Creates a new builder-style object to manufacture [`ListCertificatesOutput`](crate::operation::list_certificates::ListCertificatesOutput).
    pub fn builder() -> crate::operation::list_certificates::builders::ListCertificatesOutputBuilder
    {
        crate::operation::list_certificates::builders::ListCertificatesOutputBuilder::default()
    }
}

/// A builder for [`ListCertificatesOutput`](crate::operation::list_certificates::ListCertificatesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListCertificatesOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) certificate_summary_list:
        ::std::option::Option<::std::vec::Vec<crate::types::CertificateSummary>>,
    _request_id: Option<String>,
}
impl ListCertificatesOutputBuilder {
    /// <p>When the list is truncated, this value is present and contains the value to use for the <code>NextToken</code> parameter in a subsequent pagination request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When the list is truncated, this value is present and contains the value to use for the <code>NextToken</code> parameter in a subsequent pagination request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `certificate_summary_list`.
    ///
    /// To override the contents of this collection use [`set_certificate_summary_list`](Self::set_certificate_summary_list).
    ///
    /// <p>A list of ACM certificates.</p>
    pub fn certificate_summary_list(mut self, input: crate::types::CertificateSummary) -> Self {
        let mut v = self.certificate_summary_list.unwrap_or_default();
        v.push(input);
        self.certificate_summary_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of ACM certificates.</p>
    pub fn set_certificate_summary_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CertificateSummary>>,
    ) -> Self {
        self.certificate_summary_list = input;
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
    /// Consumes the builder and constructs a [`ListCertificatesOutput`](crate::operation::list_certificates::ListCertificatesOutput).
    pub fn build(self) -> crate::operation::list_certificates::ListCertificatesOutput {
        crate::operation::list_certificates::ListCertificatesOutput {
            next_token: self.next_token,
            certificate_summary_list: self.certificate_summary_list,
            _request_id: self._request_id,
        }
    }
}
