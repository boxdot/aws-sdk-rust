// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListReportPlansInput {
    /// <p>The number of desired results from 1 to 1000. Optional. If unspecified, the query will return 1 MB of data.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListReportPlansInput {
    /// <p>The number of desired results from 1 to 1000. Optional. If unspecified, the query will return 1 MB of data.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListReportPlansInput {
    /// Creates a new builder-style object to manufacture [`ListReportPlansInput`](crate::operation::list_report_plans::ListReportPlansInput).
    pub fn builder() -> crate::operation::list_report_plans::builders::ListReportPlansInputBuilder {
        crate::operation::list_report_plans::builders::ListReportPlansInputBuilder::default()
    }
}

/// A builder for [`ListReportPlansInput`](crate::operation::list_report_plans::ListReportPlansInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListReportPlansInputBuilder {
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListReportPlansInputBuilder {
    /// <p>The number of desired results from 1 to 1000. Optional. If unspecified, the query will return 1 MB of data.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of desired results from 1 to 1000. Optional. If unspecified, the query will return 1 MB of data.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListReportPlansInput`](crate::operation::list_report_plans::ListReportPlansInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_report_plans::ListReportPlansInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_report_plans::ListReportPlansInput {
            max_results: self.max_results,
            next_token: self.next_token,
        })
    }
}
