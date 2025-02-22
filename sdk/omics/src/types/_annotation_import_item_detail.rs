// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about an imported annotation item.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AnnotationImportItemDetail {
    /// <p>The source file's location in Amazon S3.</p>
    #[doc(hidden)]
    pub source: ::std::option::Option<::std::string::String>,
    /// <p>The item's job status.</p>
    #[doc(hidden)]
    pub job_status: ::std::option::Option<crate::types::JobStatus>,
}
impl AnnotationImportItemDetail {
    /// <p>The source file's location in Amazon S3.</p>
    pub fn source(&self) -> ::std::option::Option<&str> {
        self.source.as_deref()
    }
    /// <p>The item's job status.</p>
    pub fn job_status(&self) -> ::std::option::Option<&crate::types::JobStatus> {
        self.job_status.as_ref()
    }
}
impl AnnotationImportItemDetail {
    /// Creates a new builder-style object to manufacture [`AnnotationImportItemDetail`](crate::types::AnnotationImportItemDetail).
    pub fn builder() -> crate::types::builders::AnnotationImportItemDetailBuilder {
        crate::types::builders::AnnotationImportItemDetailBuilder::default()
    }
}

/// A builder for [`AnnotationImportItemDetail`](crate::types::AnnotationImportItemDetail).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AnnotationImportItemDetailBuilder {
    pub(crate) source: ::std::option::Option<::std::string::String>,
    pub(crate) job_status: ::std::option::Option<crate::types::JobStatus>,
}
impl AnnotationImportItemDetailBuilder {
    /// <p>The source file's location in Amazon S3.</p>
    pub fn source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source file's location in Amazon S3.</p>
    pub fn set_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source = input;
        self
    }
    /// <p>The item's job status.</p>
    pub fn job_status(mut self, input: crate::types::JobStatus) -> Self {
        self.job_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The item's job status.</p>
    pub fn set_job_status(mut self, input: ::std::option::Option<crate::types::JobStatus>) -> Self {
        self.job_status = input;
        self
    }
    /// Consumes the builder and constructs a [`AnnotationImportItemDetail`](crate::types::AnnotationImportItemDetail).
    pub fn build(self) -> crate::types::AnnotationImportItemDetail {
        crate::types::AnnotationImportItemDetail {
            source: self.source,
            job_status: self.job_status,
        }
    }
}
