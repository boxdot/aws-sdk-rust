// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListVariantStoresInput {
    /// <p>The maximum number of stores to return in one page of results.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>A list of store IDs.</p>
    #[doc(hidden)]
    pub ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>A filter to apply to the list.</p>
    #[doc(hidden)]
    pub filter: ::std::option::Option<crate::types::ListVariantStoresFilter>,
}
impl ListVariantStoresInput {
    /// <p>The maximum number of stores to return in one page of results.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>A list of store IDs.</p>
    pub fn ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.ids.as_deref()
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>A filter to apply to the list.</p>
    pub fn filter(&self) -> ::std::option::Option<&crate::types::ListVariantStoresFilter> {
        self.filter.as_ref()
    }
}
impl ListVariantStoresInput {
    /// Creates a new builder-style object to manufacture [`ListVariantStoresInput`](crate::operation::list_variant_stores::ListVariantStoresInput).
    pub fn builder(
    ) -> crate::operation::list_variant_stores::builders::ListVariantStoresInputBuilder {
        crate::operation::list_variant_stores::builders::ListVariantStoresInputBuilder::default()
    }
}

/// A builder for [`ListVariantStoresInput`](crate::operation::list_variant_stores::ListVariantStoresInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListVariantStoresInputBuilder {
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) filter: ::std::option::Option<crate::types::ListVariantStoresFilter>,
}
impl ListVariantStoresInputBuilder {
    /// <p>The maximum number of stores to return in one page of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of stores to return in one page of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Appends an item to `ids`.
    ///
    /// To override the contents of this collection use [`set_ids`](Self::set_ids).
    ///
    /// <p>A list of store IDs.</p>
    pub fn ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.ids.unwrap_or_default();
        v.push(input.into());
        self.ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of store IDs.</p>
    pub fn set_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.ids = input;
        self
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>A filter to apply to the list.</p>
    pub fn filter(mut self, input: crate::types::ListVariantStoresFilter) -> Self {
        self.filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>A filter to apply to the list.</p>
    pub fn set_filter(
        mut self,
        input: ::std::option::Option<crate::types::ListVariantStoresFilter>,
    ) -> Self {
        self.filter = input;
        self
    }
    /// Consumes the builder and constructs a [`ListVariantStoresInput`](crate::operation::list_variant_stores::ListVariantStoresInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_variant_stores::ListVariantStoresInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_variant_stores::ListVariantStoresInput {
                max_results: self.max_results,
                ids: self.ids,
                next_token: self.next_token,
                filter: self.filter,
            },
        )
    }
}
