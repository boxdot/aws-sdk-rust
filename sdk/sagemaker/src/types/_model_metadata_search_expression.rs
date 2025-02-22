// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>One or more filters that searches for the specified resource or resources in a search. All resource objects that satisfy the expression's condition are included in the search results</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModelMetadataSearchExpression {
    /// <p>A list of filter objects.</p>
    #[doc(hidden)]
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::ModelMetadataFilter>>,
}
impl ModelMetadataSearchExpression {
    /// <p>A list of filter objects.</p>
    pub fn filters(&self) -> ::std::option::Option<&[crate::types::ModelMetadataFilter]> {
        self.filters.as_deref()
    }
}
impl ModelMetadataSearchExpression {
    /// Creates a new builder-style object to manufacture [`ModelMetadataSearchExpression`](crate::types::ModelMetadataSearchExpression).
    pub fn builder() -> crate::types::builders::ModelMetadataSearchExpressionBuilder {
        crate::types::builders::ModelMetadataSearchExpressionBuilder::default()
    }
}

/// A builder for [`ModelMetadataSearchExpression`](crate::types::ModelMetadataSearchExpression).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ModelMetadataSearchExpressionBuilder {
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::ModelMetadataFilter>>,
}
impl ModelMetadataSearchExpressionBuilder {
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>A list of filter objects.</p>
    pub fn filters(mut self, input: crate::types::ModelMetadataFilter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of filter objects.</p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ModelMetadataFilter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// Consumes the builder and constructs a [`ModelMetadataSearchExpression`](crate::types::ModelMetadataSearchExpression).
    pub fn build(self) -> crate::types::ModelMetadataSearchExpression {
        crate::types::ModelMetadataSearchExpression {
            filters: self.filters,
        }
    }
}
