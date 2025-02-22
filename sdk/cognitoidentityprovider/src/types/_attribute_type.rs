// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies whether the attribute is standard or custom.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct AttributeType {
    /// <p>The name of the attribute.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The value of the attribute.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl AttributeType {
    /// <p>The name of the attribute.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The value of the attribute.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl ::std::fmt::Debug for AttributeType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AttributeType");
        formatter.field("name", &self.name);
        formatter.field("value", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl AttributeType {
    /// Creates a new builder-style object to manufacture [`AttributeType`](crate::types::AttributeType).
    pub fn builder() -> crate::types::builders::AttributeTypeBuilder {
        crate::types::builders::AttributeTypeBuilder::default()
    }
}

/// A builder for [`AttributeType`](crate::types::AttributeType).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct AttributeTypeBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl AttributeTypeBuilder {
    /// <p>The name of the attribute.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the attribute.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The value of the attribute.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value of the attribute.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`AttributeType`](crate::types::AttributeType).
    pub fn build(self) -> crate::types::AttributeType {
        crate::types::AttributeType {
            name: self.name,
            value: self.value,
        }
    }
}
impl ::std::fmt::Debug for AttributeTypeBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AttributeTypeBuilder");
        formatter.field("name", &self.name);
        formatter.field("value", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
