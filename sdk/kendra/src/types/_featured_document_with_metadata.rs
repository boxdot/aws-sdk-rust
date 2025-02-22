// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A featured document with its metadata information. This document is displayed at the top of the search results page, placed above all other results for certain queries. If there's an exact match of a query, then the document is featured in the search results.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FeaturedDocumentWithMetadata {
    /// <p>The identifier of the featured document with its metadata. You can use the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_Query.html">Query</a> API to search for specific documents with their document IDs included in the result items, or you can use the console.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The main title of the featured document.</p>
    #[doc(hidden)]
    pub title: ::std::option::Option<::std::string::String>,
    /// <p>The source URI location of the featured document.</p>
    #[doc(hidden)]
    pub uri: ::std::option::Option<::std::string::String>,
}
impl FeaturedDocumentWithMetadata {
    /// <p>The identifier of the featured document with its metadata. You can use the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_Query.html">Query</a> API to search for specific documents with their document IDs included in the result items, or you can use the console.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The main title of the featured document.</p>
    pub fn title(&self) -> ::std::option::Option<&str> {
        self.title.as_deref()
    }
    /// <p>The source URI location of the featured document.</p>
    pub fn uri(&self) -> ::std::option::Option<&str> {
        self.uri.as_deref()
    }
}
impl FeaturedDocumentWithMetadata {
    /// Creates a new builder-style object to manufacture [`FeaturedDocumentWithMetadata`](crate::types::FeaturedDocumentWithMetadata).
    pub fn builder() -> crate::types::builders::FeaturedDocumentWithMetadataBuilder {
        crate::types::builders::FeaturedDocumentWithMetadataBuilder::default()
    }
}

/// A builder for [`FeaturedDocumentWithMetadata`](crate::types::FeaturedDocumentWithMetadata).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FeaturedDocumentWithMetadataBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) title: ::std::option::Option<::std::string::String>,
    pub(crate) uri: ::std::option::Option<::std::string::String>,
}
impl FeaturedDocumentWithMetadataBuilder {
    /// <p>The identifier of the featured document with its metadata. You can use the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_Query.html">Query</a> API to search for specific documents with their document IDs included in the result items, or you can use the console.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the featured document with its metadata. You can use the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_Query.html">Query</a> API to search for specific documents with their document IDs included in the result items, or you can use the console.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The main title of the featured document.</p>
    pub fn title(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.title = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The main title of the featured document.</p>
    pub fn set_title(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.title = input;
        self
    }
    /// <p>The source URI location of the featured document.</p>
    pub fn uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.uri = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source URI location of the featured document.</p>
    pub fn set_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.uri = input;
        self
    }
    /// Consumes the builder and constructs a [`FeaturedDocumentWithMetadata`](crate::types::FeaturedDocumentWithMetadata).
    pub fn build(self) -> crate::types::FeaturedDocumentWithMetadata {
        crate::types::FeaturedDocumentWithMetadata {
            id: self.id,
            title: self.title,
            uri: self.uri,
        }
    }
}
