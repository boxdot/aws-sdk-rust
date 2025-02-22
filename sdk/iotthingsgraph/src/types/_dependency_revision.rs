// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that contains the ID and revision number of a workflow or system that is part of a deployment.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DependencyRevision {
    /// <p>The ID of the workflow or system.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The revision number of the workflow or system.</p>
    #[doc(hidden)]
    pub revision_number: ::std::option::Option<i64>,
}
impl DependencyRevision {
    /// <p>The ID of the workflow or system.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The revision number of the workflow or system.</p>
    pub fn revision_number(&self) -> ::std::option::Option<i64> {
        self.revision_number
    }
}
impl DependencyRevision {
    /// Creates a new builder-style object to manufacture [`DependencyRevision`](crate::types::DependencyRevision).
    pub fn builder() -> crate::types::builders::DependencyRevisionBuilder {
        crate::types::builders::DependencyRevisionBuilder::default()
    }
}

/// A builder for [`DependencyRevision`](crate::types::DependencyRevision).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DependencyRevisionBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) revision_number: ::std::option::Option<i64>,
}
impl DependencyRevisionBuilder {
    /// <p>The ID of the workflow or system.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the workflow or system.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The revision number of the workflow or system.</p>
    pub fn revision_number(mut self, input: i64) -> Self {
        self.revision_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>The revision number of the workflow or system.</p>
    pub fn set_revision_number(mut self, input: ::std::option::Option<i64>) -> Self {
        self.revision_number = input;
        self
    }
    /// Consumes the builder and constructs a [`DependencyRevision`](crate::types::DependencyRevision).
    pub fn build(self) -> crate::types::DependencyRevision {
        crate::types::DependencyRevision {
            id: self.id,
            revision_number: self.revision_number,
        }
    }
}
