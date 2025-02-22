// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The entity details. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct Entity {
    /// <p>The entity type.</p>
    #[doc(hidden)]
    pub entity_type: ::std::option::Option<::std::string::String>,
    /// <p>The entity ID. If you do not know the <code>entityId</code>, you can pass <code>unknown</code>, which is areserved string literal.</p>
    #[doc(hidden)]
    pub entity_id: ::std::option::Option<::std::string::String>,
}
impl Entity {
    /// <p>The entity type.</p>
    pub fn entity_type(&self) -> ::std::option::Option<&str> {
        self.entity_type.as_deref()
    }
    /// <p>The entity ID. If you do not know the <code>entityId</code>, you can pass <code>unknown</code>, which is areserved string literal.</p>
    pub fn entity_id(&self) -> ::std::option::Option<&str> {
        self.entity_id.as_deref()
    }
}
impl ::std::fmt::Debug for Entity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("Entity");
        formatter.field("entity_type", &"*** Sensitive Data Redacted ***");
        formatter.field("entity_id", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl Entity {
    /// Creates a new builder-style object to manufacture [`Entity`](crate::types::Entity).
    pub fn builder() -> crate::types::builders::EntityBuilder {
        crate::types::builders::EntityBuilder::default()
    }
}

/// A builder for [`Entity`](crate::types::Entity).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct EntityBuilder {
    pub(crate) entity_type: ::std::option::Option<::std::string::String>,
    pub(crate) entity_id: ::std::option::Option<::std::string::String>,
}
impl EntityBuilder {
    /// <p>The entity type.</p>
    pub fn entity_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.entity_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The entity type.</p>
    pub fn set_entity_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.entity_type = input;
        self
    }
    /// <p>The entity ID. If you do not know the <code>entityId</code>, you can pass <code>unknown</code>, which is areserved string literal.</p>
    pub fn entity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.entity_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The entity ID. If you do not know the <code>entityId</code>, you can pass <code>unknown</code>, which is areserved string literal.</p>
    pub fn set_entity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.entity_id = input;
        self
    }
    /// Consumes the builder and constructs a [`Entity`](crate::types::Entity).
    pub fn build(self) -> crate::types::Entity {
        crate::types::Entity {
            entity_type: self.entity_type,
            entity_id: self.entity_id,
        }
    }
}
impl ::std::fmt::Debug for EntityBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("EntityBuilder");
        formatter.field("entity_type", &"*** Sensitive Data Redacted ***");
        formatter.field("entity_id", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
