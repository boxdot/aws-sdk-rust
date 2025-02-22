// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListAppsInput {
    /// <p>The name of the simulation that you want to list apps for.</p>
    #[doc(hidden)]
    pub simulation: ::std::option::Option<::std::string::String>,
    /// <p>The name of the domain that you want to list apps for.</p>
    #[doc(hidden)]
    pub domain: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of apps to list.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>If SimSpace Weaver returns <code>nextToken</code>, then there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. To retrieve the next page, call the operation again using the returned token. Keep all other arguments unchanged. If no results remain, then <code>nextToken</code> is set to <code>null</code>. Each pagination token expires after 24 hours. If you provide a token that isn't valid, then you receive an <i>HTTP 400 ValidationException</i> error.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListAppsInput {
    /// <p>The name of the simulation that you want to list apps for.</p>
    pub fn simulation(&self) -> ::std::option::Option<&str> {
        self.simulation.as_deref()
    }
    /// <p>The name of the domain that you want to list apps for.</p>
    pub fn domain(&self) -> ::std::option::Option<&str> {
        self.domain.as_deref()
    }
    /// <p>The maximum number of apps to list.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>If SimSpace Weaver returns <code>nextToken</code>, then there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. To retrieve the next page, call the operation again using the returned token. Keep all other arguments unchanged. If no results remain, then <code>nextToken</code> is set to <code>null</code>. Each pagination token expires after 24 hours. If you provide a token that isn't valid, then you receive an <i>HTTP 400 ValidationException</i> error.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListAppsInput {
    /// Creates a new builder-style object to manufacture [`ListAppsInput`](crate::operation::list_apps::ListAppsInput).
    pub fn builder() -> crate::operation::list_apps::builders::ListAppsInputBuilder {
        crate::operation::list_apps::builders::ListAppsInputBuilder::default()
    }
}

/// A builder for [`ListAppsInput`](crate::operation::list_apps::ListAppsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListAppsInputBuilder {
    pub(crate) simulation: ::std::option::Option<::std::string::String>,
    pub(crate) domain: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListAppsInputBuilder {
    /// <p>The name of the simulation that you want to list apps for.</p>
    pub fn simulation(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.simulation = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the simulation that you want to list apps for.</p>
    pub fn set_simulation(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.simulation = input;
        self
    }
    /// <p>The name of the domain that you want to list apps for.</p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the domain that you want to list apps for.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain = input;
        self
    }
    /// <p>The maximum number of apps to list.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of apps to list.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>If SimSpace Weaver returns <code>nextToken</code>, then there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. To retrieve the next page, call the operation again using the returned token. Keep all other arguments unchanged. If no results remain, then <code>nextToken</code> is set to <code>null</code>. Each pagination token expires after 24 hours. If you provide a token that isn't valid, then you receive an <i>HTTP 400 ValidationException</i> error.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If SimSpace Weaver returns <code>nextToken</code>, then there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. To retrieve the next page, call the operation again using the returned token. Keep all other arguments unchanged. If no results remain, then <code>nextToken</code> is set to <code>null</code>. Each pagination token expires after 24 hours. If you provide a token that isn't valid, then you receive an <i>HTTP 400 ValidationException</i> error.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListAppsInput`](crate::operation::list_apps::ListAppsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_apps::ListAppsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_apps::ListAppsInput {
            simulation: self.simulation,
            domain: self.domain,
            max_results: self.max_results,
            next_token: self.next_token,
        })
    }
}
