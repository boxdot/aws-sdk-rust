// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A location in a policy that is represented as a path through the JSON representation and a corresponding span.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Location {
    /// <p>A path in a policy, represented as a sequence of path elements.</p>
    #[doc(hidden)]
    pub path: ::std::option::Option<::std::vec::Vec<crate::types::PathElement>>,
    /// <p>A span in a policy.</p>
    #[doc(hidden)]
    pub span: ::std::option::Option<crate::types::Span>,
}
impl Location {
    /// <p>A path in a policy, represented as a sequence of path elements.</p>
    pub fn path(&self) -> ::std::option::Option<&[crate::types::PathElement]> {
        self.path.as_deref()
    }
    /// <p>A span in a policy.</p>
    pub fn span(&self) -> ::std::option::Option<&crate::types::Span> {
        self.span.as_ref()
    }
}
impl Location {
    /// Creates a new builder-style object to manufacture [`Location`](crate::types::Location).
    pub fn builder() -> crate::types::builders::LocationBuilder {
        crate::types::builders::LocationBuilder::default()
    }
}

/// A builder for [`Location`](crate::types::Location).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LocationBuilder {
    pub(crate) path: ::std::option::Option<::std::vec::Vec<crate::types::PathElement>>,
    pub(crate) span: ::std::option::Option<crate::types::Span>,
}
impl LocationBuilder {
    /// Appends an item to `path`.
    ///
    /// To override the contents of this collection use [`set_path`](Self::set_path).
    ///
    /// <p>A path in a policy, represented as a sequence of path elements.</p>
    pub fn path(mut self, input: crate::types::PathElement) -> Self {
        let mut v = self.path.unwrap_or_default();
        v.push(input);
        self.path = ::std::option::Option::Some(v);
        self
    }
    /// <p>A path in a policy, represented as a sequence of path elements.</p>
    pub fn set_path(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PathElement>>,
    ) -> Self {
        self.path = input;
        self
    }
    /// <p>A span in a policy.</p>
    pub fn span(mut self, input: crate::types::Span) -> Self {
        self.span = ::std::option::Option::Some(input);
        self
    }
    /// <p>A span in a policy.</p>
    pub fn set_span(mut self, input: ::std::option::Option<crate::types::Span>) -> Self {
        self.span = input;
        self
    }
    /// Consumes the builder and constructs a [`Location`](crate::types::Location).
    pub fn build(self) -> crate::types::Location {
        crate::types::Location {
            path: self.path,
            span: self.span,
        }
    }
}
