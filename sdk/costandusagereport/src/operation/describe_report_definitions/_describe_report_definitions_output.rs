// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>If the action is successful, the service sends back an HTTP 200 response.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeReportDefinitionsOutput {
    /// <p>A list of AWS Cost and Usage reports owned by the account.</p>
    #[doc(hidden)]
    pub report_definitions: ::std::option::Option<::std::vec::Vec<crate::types::ReportDefinition>>,
    /// <p>A generic string.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeReportDefinitionsOutput {
    /// <p>A list of AWS Cost and Usage reports owned by the account.</p>
    pub fn report_definitions(&self) -> ::std::option::Option<&[crate::types::ReportDefinition]> {
        self.report_definitions.as_deref()
    }
    /// <p>A generic string.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeReportDefinitionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeReportDefinitionsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeReportDefinitionsOutput`](crate::operation::describe_report_definitions::DescribeReportDefinitionsOutput).
    pub fn builder() -> crate::operation::describe_report_definitions::builders::DescribeReportDefinitionsOutputBuilder{
        crate::operation::describe_report_definitions::builders::DescribeReportDefinitionsOutputBuilder::default()
    }
}

/// A builder for [`DescribeReportDefinitionsOutput`](crate::operation::describe_report_definitions::DescribeReportDefinitionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeReportDefinitionsOutputBuilder {
    pub(crate) report_definitions:
        ::std::option::Option<::std::vec::Vec<crate::types::ReportDefinition>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeReportDefinitionsOutputBuilder {
    /// Appends an item to `report_definitions`.
    ///
    /// To override the contents of this collection use [`set_report_definitions`](Self::set_report_definitions).
    ///
    /// <p>A list of AWS Cost and Usage reports owned by the account.</p>
    pub fn report_definitions(mut self, input: crate::types::ReportDefinition) -> Self {
        let mut v = self.report_definitions.unwrap_or_default();
        v.push(input);
        self.report_definitions = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of AWS Cost and Usage reports owned by the account.</p>
    pub fn set_report_definitions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ReportDefinition>>,
    ) -> Self {
        self.report_definitions = input;
        self
    }
    /// <p>A generic string.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A generic string.</p>
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
    /// Consumes the builder and constructs a [`DescribeReportDefinitionsOutput`](crate::operation::describe_report_definitions::DescribeReportDefinitionsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_report_definitions::DescribeReportDefinitionsOutput {
        crate::operation::describe_report_definitions::DescribeReportDefinitionsOutput {
            report_definitions: self.report_definitions,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
