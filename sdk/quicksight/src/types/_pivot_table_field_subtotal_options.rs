// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The optional configuration of subtotals cells.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PivotTableFieldSubtotalOptions {
    /// <p>The field ID of the subtotal options.</p>
    #[doc(hidden)]
    pub field_id: ::std::option::Option<::std::string::String>,
}
impl PivotTableFieldSubtotalOptions {
    /// <p>The field ID of the subtotal options.</p>
    pub fn field_id(&self) -> ::std::option::Option<&str> {
        self.field_id.as_deref()
    }
}
impl PivotTableFieldSubtotalOptions {
    /// Creates a new builder-style object to manufacture [`PivotTableFieldSubtotalOptions`](crate::types::PivotTableFieldSubtotalOptions).
    pub fn builder() -> crate::types::builders::PivotTableFieldSubtotalOptionsBuilder {
        crate::types::builders::PivotTableFieldSubtotalOptionsBuilder::default()
    }
}

/// A builder for [`PivotTableFieldSubtotalOptions`](crate::types::PivotTableFieldSubtotalOptions).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PivotTableFieldSubtotalOptionsBuilder {
    pub(crate) field_id: ::std::option::Option<::std::string::String>,
}
impl PivotTableFieldSubtotalOptionsBuilder {
    /// <p>The field ID of the subtotal options.</p>
    pub fn field_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.field_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The field ID of the subtotal options.</p>
    pub fn set_field_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.field_id = input;
        self
    }
    /// Consumes the builder and constructs a [`PivotTableFieldSubtotalOptions`](crate::types::PivotTableFieldSubtotalOptions).
    pub fn build(self) -> crate::types::PivotTableFieldSubtotalOptions {
        crate::types::PivotTableFieldSubtotalOptions {
            field_id: self.field_id,
        }
    }
}
