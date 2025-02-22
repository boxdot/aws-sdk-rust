// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// The priority you want to assign to a source. You can have a primary stream and a backup stream or two equally prioritized streams.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SourcePriority {
    /// The name of the source you choose as the primary source for this flow.
    #[doc(hidden)]
    pub primary_source: ::std::option::Option<::std::string::String>,
}
impl SourcePriority {
    /// The name of the source you choose as the primary source for this flow.
    pub fn primary_source(&self) -> ::std::option::Option<&str> {
        self.primary_source.as_deref()
    }
}
impl SourcePriority {
    /// Creates a new builder-style object to manufacture [`SourcePriority`](crate::types::SourcePriority).
    pub fn builder() -> crate::types::builders::SourcePriorityBuilder {
        crate::types::builders::SourcePriorityBuilder::default()
    }
}

/// A builder for [`SourcePriority`](crate::types::SourcePriority).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SourcePriorityBuilder {
    pub(crate) primary_source: ::std::option::Option<::std::string::String>,
}
impl SourcePriorityBuilder {
    /// The name of the source you choose as the primary source for this flow.
    pub fn primary_source(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.primary_source = ::std::option::Option::Some(input.into());
        self
    }
    /// The name of the source you choose as the primary source for this flow.
    pub fn set_primary_source(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.primary_source = input;
        self
    }
    /// Consumes the builder and constructs a [`SourcePriority`](crate::types::SourcePriority).
    pub fn build(self) -> crate::types::SourcePriority {
        crate::types::SourcePriority {
            primary_source: self.primary_source,
        }
    }
}
