// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A specific result obtained by specifying the name, value, and operator. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Filter {
    /// <p>The name of the filter. Filter names are case-sensitive. </p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The filter values. Filter values are case-sensitive. If you specify multiple values for a filter, the values are joined with an OR, and the request returns all results that match any of the specified values</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
    /// <p>The operator for the filter. </p>
    #[doc(hidden)]
    pub operator: ::std::option::Option<crate::types::FilterOperator>,
}
impl Filter {
    /// <p>The name of the filter. Filter names are case-sensitive. </p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The filter values. Filter values are case-sensitive. If you specify multiple values for a filter, the values are joined with an OR, and the request returns all results that match any of the specified values</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
    /// <p>The operator for the filter. </p>
    pub fn operator(&self) -> ::std::option::Option<&crate::types::FilterOperator> {
        self.operator.as_ref()
    }
}
impl Filter {
    /// Creates a new builder-style object to manufacture [`Filter`](crate::types::Filter).
    pub fn builder() -> crate::types::builders::FilterBuilder {
        crate::types::builders::FilterBuilder::default()
    }
}

/// A builder for [`Filter`](crate::types::Filter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FilterBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
    pub(crate) operator: ::std::option::Option<crate::types::FilterOperator>,
}
impl FilterBuilder {
    /// <p>The name of the filter. Filter names are case-sensitive. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the filter. Filter names are case-sensitive. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The filter values. Filter values are case-sensitive. If you specify multiple values for a filter, the values are joined with an OR, and the request returns all results that match any of the specified values</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The filter values. Filter values are case-sensitive. If you specify multiple values for a filter, the values are joined with an OR, and the request returns all results that match any of the specified values</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// <p>The operator for the filter. </p>
    pub fn operator(mut self, input: crate::types::FilterOperator) -> Self {
        self.operator = ::std::option::Option::Some(input);
        self
    }
    /// <p>The operator for the filter. </p>
    pub fn set_operator(
        mut self,
        input: ::std::option::Option<crate::types::FilterOperator>,
    ) -> Self {
        self.operator = input;
        self
    }
    /// Consumes the builder and constructs a [`Filter`](crate::types::Filter).
    pub fn build(self) -> crate::types::Filter {
        crate::types::Filter {
            name: self.name,
            value: self.value,
            operator: self.operator,
        }
    }
}
