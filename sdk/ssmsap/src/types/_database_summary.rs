// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The summary of the database.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DatabaseSummary {
    /// <p>The ID of the application.</p>
    #[doc(hidden)]
    pub application_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the component.</p>
    #[doc(hidden)]
    pub component_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the database.</p>
    #[doc(hidden)]
    pub database_id: ::std::option::Option<::std::string::String>,
    /// <p>The type of the database.</p>
    #[doc(hidden)]
    pub database_type: ::std::option::Option<crate::types::DatabaseType>,
    /// <p>The Amazon Resource Name (ARN) of the database.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The tags of the database.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl DatabaseSummary {
    /// <p>The ID of the application.</p>
    pub fn application_id(&self) -> ::std::option::Option<&str> {
        self.application_id.as_deref()
    }
    /// <p>The ID of the component.</p>
    pub fn component_id(&self) -> ::std::option::Option<&str> {
        self.component_id.as_deref()
    }
    /// <p>The ID of the database.</p>
    pub fn database_id(&self) -> ::std::option::Option<&str> {
        self.database_id.as_deref()
    }
    /// <p>The type of the database.</p>
    pub fn database_type(&self) -> ::std::option::Option<&crate::types::DatabaseType> {
        self.database_type.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the database.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The tags of the database.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl DatabaseSummary {
    /// Creates a new builder-style object to manufacture [`DatabaseSummary`](crate::types::DatabaseSummary).
    pub fn builder() -> crate::types::builders::DatabaseSummaryBuilder {
        crate::types::builders::DatabaseSummaryBuilder::default()
    }
}

/// A builder for [`DatabaseSummary`](crate::types::DatabaseSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DatabaseSummaryBuilder {
    pub(crate) application_id: ::std::option::Option<::std::string::String>,
    pub(crate) component_id: ::std::option::Option<::std::string::String>,
    pub(crate) database_id: ::std::option::Option<::std::string::String>,
    pub(crate) database_type: ::std::option::Option<crate::types::DatabaseType>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl DatabaseSummaryBuilder {
    /// <p>The ID of the application.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the application.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_id = input;
        self
    }
    /// <p>The ID of the component.</p>
    pub fn component_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.component_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the component.</p>
    pub fn set_component_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.component_id = input;
        self
    }
    /// <p>The ID of the database.</p>
    pub fn database_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.database_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the database.</p>
    pub fn set_database_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.database_id = input;
        self
    }
    /// <p>The type of the database.</p>
    pub fn database_type(mut self, input: crate::types::DatabaseType) -> Self {
        self.database_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the database.</p>
    pub fn set_database_type(
        mut self,
        input: ::std::option::Option<crate::types::DatabaseType>,
    ) -> Self {
        self.database_type = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the database.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the database.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags of the database.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The tags of the database.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`DatabaseSummary`](crate::types::DatabaseSummary).
    pub fn build(self) -> crate::types::DatabaseSummary {
        crate::types::DatabaseSummary {
            application_id: self.application_id,
            component_id: self.component_id,
            database_id: self.database_id,
            database_type: self.database_type,
            arn: self.arn,
            tags: self.tags,
        }
    }
}
