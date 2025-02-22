// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A name-values pair of elements you can use to filter the results when querying your import tasks. Currently, wildcards are not supported for filters.</p> <note>
/// <p>When filtering by import status, all other filter values are ignored.</p>
/// </note>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportTaskFilter {
    /// <p>The name, status, or import task ID for a specific import task.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<crate::types::ImportTaskFilterName>,
    /// <p>An array of strings that you can provide to match against a specific name, status, or import task ID to filter the results for your import task queries.</p>
    #[doc(hidden)]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ImportTaskFilter {
    /// <p>The name, status, or import task ID for a specific import task.</p>
    pub fn name(&self) -> ::std::option::Option<&crate::types::ImportTaskFilterName> {
        self.name.as_ref()
    }
    /// <p>An array of strings that you can provide to match against a specific name, status, or import task ID to filter the results for your import task queries.</p>
    pub fn values(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.values.as_deref()
    }
}
impl ImportTaskFilter {
    /// Creates a new builder-style object to manufacture [`ImportTaskFilter`](crate::types::ImportTaskFilter).
    pub fn builder() -> crate::types::builders::ImportTaskFilterBuilder {
        crate::types::builders::ImportTaskFilterBuilder::default()
    }
}

/// A builder for [`ImportTaskFilter`](crate::types::ImportTaskFilter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ImportTaskFilterBuilder {
    pub(crate) name: ::std::option::Option<crate::types::ImportTaskFilterName>,
    pub(crate) values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ImportTaskFilterBuilder {
    /// <p>The name, status, or import task ID for a specific import task.</p>
    pub fn name(mut self, input: crate::types::ImportTaskFilterName) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name, status, or import task ID for a specific import task.</p>
    pub fn set_name(
        mut self,
        input: ::std::option::Option<crate::types::ImportTaskFilterName>,
    ) -> Self {
        self.name = input;
        self
    }
    /// Appends an item to `values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p>An array of strings that you can provide to match against a specific name, status, or import task ID to filter the results for your import task queries.</p>
    pub fn values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.values.unwrap_or_default();
        v.push(input.into());
        self.values = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of strings that you can provide to match against a specific name, status, or import task ID to filter the results for your import task queries.</p>
    pub fn set_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.values = input;
        self
    }
    /// Consumes the builder and constructs a [`ImportTaskFilter`](crate::types::ImportTaskFilter).
    pub fn build(self) -> crate::types::ImportTaskFilter {
        crate::types::ImportTaskFilter {
            name: self.name,
            values: self.values,
        }
    }
}
