// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A structure describing the name, data type, and geographic role of the columns.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ColumnGroupColumnSchema {
    /// <p>The name of the column group's column schema.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ColumnGroupColumnSchema {
    /// <p>The name of the column group's column schema.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl ColumnGroupColumnSchema {
    /// Creates a new builder-style object to manufacture [`ColumnGroupColumnSchema`](crate::types::ColumnGroupColumnSchema).
    pub fn builder() -> crate::types::builders::ColumnGroupColumnSchemaBuilder {
        crate::types::builders::ColumnGroupColumnSchemaBuilder::default()
    }
}

/// A builder for [`ColumnGroupColumnSchema`](crate::types::ColumnGroupColumnSchema).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ColumnGroupColumnSchemaBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl ColumnGroupColumnSchemaBuilder {
    /// <p>The name of the column group's column schema.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the column group's column schema.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`ColumnGroupColumnSchema`](crate::types::ColumnGroupColumnSchema).
    pub fn build(self) -> crate::types::ColumnGroupColumnSchema {
        crate::types::ColumnGroupColumnSchema { name: self.name }
    }
}
