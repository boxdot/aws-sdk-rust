// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The value input pf the numeric range filter.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NumericRangeFilterValue {
    /// <p>The static value of the numeric range filter.</p>
    #[doc(hidden)]
    pub static_value: ::std::option::Option<f64>,
    /// <p>The parameter that is used in the numeric range.</p>
    #[doc(hidden)]
    pub parameter: ::std::option::Option<::std::string::String>,
}
impl NumericRangeFilterValue {
    /// <p>The static value of the numeric range filter.</p>
    pub fn static_value(&self) -> ::std::option::Option<f64> {
        self.static_value
    }
    /// <p>The parameter that is used in the numeric range.</p>
    pub fn parameter(&self) -> ::std::option::Option<&str> {
        self.parameter.as_deref()
    }
}
impl NumericRangeFilterValue {
    /// Creates a new builder-style object to manufacture [`NumericRangeFilterValue`](crate::types::NumericRangeFilterValue).
    pub fn builder() -> crate::types::builders::NumericRangeFilterValueBuilder {
        crate::types::builders::NumericRangeFilterValueBuilder::default()
    }
}

/// A builder for [`NumericRangeFilterValue`](crate::types::NumericRangeFilterValue).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct NumericRangeFilterValueBuilder {
    pub(crate) static_value: ::std::option::Option<f64>,
    pub(crate) parameter: ::std::option::Option<::std::string::String>,
}
impl NumericRangeFilterValueBuilder {
    /// <p>The static value of the numeric range filter.</p>
    pub fn static_value(mut self, input: f64) -> Self {
        self.static_value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The static value of the numeric range filter.</p>
    pub fn set_static_value(mut self, input: ::std::option::Option<f64>) -> Self {
        self.static_value = input;
        self
    }
    /// <p>The parameter that is used in the numeric range.</p>
    pub fn parameter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.parameter = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The parameter that is used in the numeric range.</p>
    pub fn set_parameter(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.parameter = input;
        self
    }
    /// Consumes the builder and constructs a [`NumericRangeFilterValue`](crate::types::NumericRangeFilterValue).
    pub fn build(self) -> crate::types::NumericRangeFilterValue {
        crate::types::NumericRangeFilterValue {
            static_value: self.static_value,
            parameter: self.parameter,
        }
    }
}
