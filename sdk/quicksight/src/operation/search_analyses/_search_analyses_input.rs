// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SearchAnalysesInput {
    /// <p>The ID of the Amazon Web Services account that contains the analyses that you're searching for.</p>
    #[doc(hidden)]
    pub aws_account_id: ::std::option::Option<::std::string::String>,
    /// <p>The structure for the search filters that you want to apply to your search. </p>
    #[doc(hidden)]
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::AnalysisSearchFilter>>,
    /// <p>A pagination token that can be used in a subsequent request.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
}
impl SearchAnalysesInput {
    /// <p>The ID of the Amazon Web Services account that contains the analyses that you're searching for.</p>
    pub fn aws_account_id(&self) -> ::std::option::Option<&str> {
        self.aws_account_id.as_deref()
    }
    /// <p>The structure for the search filters that you want to apply to your search. </p>
    pub fn filters(&self) -> ::std::option::Option<&[crate::types::AnalysisSearchFilter]> {
        self.filters.as_deref()
    }
    /// <p>A pagination token that can be used in a subsequent request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl SearchAnalysesInput {
    /// Creates a new builder-style object to manufacture [`SearchAnalysesInput`](crate::operation::search_analyses::SearchAnalysesInput).
    pub fn builder() -> crate::operation::search_analyses::builders::SearchAnalysesInputBuilder {
        crate::operation::search_analyses::builders::SearchAnalysesInputBuilder::default()
    }
}

/// A builder for [`SearchAnalysesInput`](crate::operation::search_analyses::SearchAnalysesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SearchAnalysesInputBuilder {
    pub(crate) aws_account_id: ::std::option::Option<::std::string::String>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::AnalysisSearchFilter>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl SearchAnalysesInputBuilder {
    /// <p>The ID of the Amazon Web Services account that contains the analyses that you're searching for.</p>
    pub fn aws_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.aws_account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the analyses that you're searching for.</p>
    pub fn set_aws_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.aws_account_id = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The structure for the search filters that you want to apply to your search. </p>
    pub fn filters(mut self, input: crate::types::AnalysisSearchFilter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>The structure for the search filters that you want to apply to your search. </p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AnalysisSearchFilter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>A pagination token that can be used in a subsequent request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A pagination token that can be used in a subsequent request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`SearchAnalysesInput`](crate::operation::search_analyses::SearchAnalysesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::search_analyses::SearchAnalysesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::search_analyses::SearchAnalysesInput {
            aws_account_id: self.aws_account_id,
            filters: self.filters,
            next_token: self.next_token,
            max_results: self.max_results,
        })
    }
}
