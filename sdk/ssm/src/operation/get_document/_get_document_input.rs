// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetDocumentInput {
    /// <p>The name of the SSM document.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>An optional field specifying the version of the artifact associated with the document. For example, "Release 12, Update 6". This value is unique across all versions of a document and can't be changed.</p>
    #[doc(hidden)]
    pub version_name: ::std::option::Option<::std::string::String>,
    /// <p>The document version for which you want information.</p>
    #[doc(hidden)]
    pub document_version: ::std::option::Option<::std::string::String>,
    /// <p>Returns the document in the specified format. The document format can be either JSON or YAML. JSON is the default format.</p>
    #[doc(hidden)]
    pub document_format: ::std::option::Option<crate::types::DocumentFormat>,
}
impl GetDocumentInput {
    /// <p>The name of the SSM document.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>An optional field specifying the version of the artifact associated with the document. For example, "Release 12, Update 6". This value is unique across all versions of a document and can't be changed.</p>
    pub fn version_name(&self) -> ::std::option::Option<&str> {
        self.version_name.as_deref()
    }
    /// <p>The document version for which you want information.</p>
    pub fn document_version(&self) -> ::std::option::Option<&str> {
        self.document_version.as_deref()
    }
    /// <p>Returns the document in the specified format. The document format can be either JSON or YAML. JSON is the default format.</p>
    pub fn document_format(&self) -> ::std::option::Option<&crate::types::DocumentFormat> {
        self.document_format.as_ref()
    }
}
impl GetDocumentInput {
    /// Creates a new builder-style object to manufacture [`GetDocumentInput`](crate::operation::get_document::GetDocumentInput).
    pub fn builder() -> crate::operation::get_document::builders::GetDocumentInputBuilder {
        crate::operation::get_document::builders::GetDocumentInputBuilder::default()
    }
}

/// A builder for [`GetDocumentInput`](crate::operation::get_document::GetDocumentInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetDocumentInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) version_name: ::std::option::Option<::std::string::String>,
    pub(crate) document_version: ::std::option::Option<::std::string::String>,
    pub(crate) document_format: ::std::option::Option<crate::types::DocumentFormat>,
}
impl GetDocumentInputBuilder {
    /// <p>The name of the SSM document.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the SSM document.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>An optional field specifying the version of the artifact associated with the document. For example, "Release 12, Update 6". This value is unique across all versions of a document and can't be changed.</p>
    pub fn version_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An optional field specifying the version of the artifact associated with the document. For example, "Release 12, Update 6". This value is unique across all versions of a document and can't be changed.</p>
    pub fn set_version_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version_name = input;
        self
    }
    /// <p>The document version for which you want information.</p>
    pub fn document_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.document_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The document version for which you want information.</p>
    pub fn set_document_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.document_version = input;
        self
    }
    /// <p>Returns the document in the specified format. The document format can be either JSON or YAML. JSON is the default format.</p>
    pub fn document_format(mut self, input: crate::types::DocumentFormat) -> Self {
        self.document_format = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns the document in the specified format. The document format can be either JSON or YAML. JSON is the default format.</p>
    pub fn set_document_format(
        mut self,
        input: ::std::option::Option<crate::types::DocumentFormat>,
    ) -> Self {
        self.document_format = input;
        self
    }
    /// Consumes the builder and constructs a [`GetDocumentInput`](crate::operation::get_document::GetDocumentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_document::GetDocumentInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_document::GetDocumentInput {
            name: self.name,
            version_name: self.version_name,
            document_version: self.document_version,
            document_format: self.document_format,
        })
    }
}
