// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTableDataImportJobOutput {
    /// <p> The current status of the import job. </p>
    #[doc(hidden)]
    pub job_status: ::std::option::Option<crate::types::TableDataImportJobStatus>,
    /// <p> A message providing more details about the current status of the import job. </p>
    #[doc(hidden)]
    pub message: ::std::option::Option<::std::string::String>,
    /// <p> The metadata about the job that was submitted for import. </p>
    #[doc(hidden)]
    pub job_metadata: ::std::option::Option<crate::types::TableDataImportJobMetadata>,
    /// <p> If job status is failed, error code to understand reason for the failure. </p>
    #[doc(hidden)]
    pub error_code: ::std::option::Option<crate::types::ErrorCode>,
    _request_id: Option<String>,
}
impl DescribeTableDataImportJobOutput {
    /// <p> The current status of the import job. </p>
    pub fn job_status(&self) -> ::std::option::Option<&crate::types::TableDataImportJobStatus> {
        self.job_status.as_ref()
    }
    /// <p> A message providing more details about the current status of the import job. </p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
    /// <p> The metadata about the job that was submitted for import. </p>
    pub fn job_metadata(&self) -> ::std::option::Option<&crate::types::TableDataImportJobMetadata> {
        self.job_metadata.as_ref()
    }
    /// <p> If job status is failed, error code to understand reason for the failure. </p>
    pub fn error_code(&self) -> ::std::option::Option<&crate::types::ErrorCode> {
        self.error_code.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeTableDataImportJobOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeTableDataImportJobOutput {
    /// Creates a new builder-style object to manufacture [`DescribeTableDataImportJobOutput`](crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput).
    pub fn builder() -> crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobOutputBuilder{
        crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobOutputBuilder::default()
    }
}

/// A builder for [`DescribeTableDataImportJobOutput`](crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeTableDataImportJobOutputBuilder {
    pub(crate) job_status: ::std::option::Option<crate::types::TableDataImportJobStatus>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
    pub(crate) job_metadata: ::std::option::Option<crate::types::TableDataImportJobMetadata>,
    pub(crate) error_code: ::std::option::Option<crate::types::ErrorCode>,
    _request_id: Option<String>,
}
impl DescribeTableDataImportJobOutputBuilder {
    /// <p> The current status of the import job. </p>
    pub fn job_status(mut self, input: crate::types::TableDataImportJobStatus) -> Self {
        self.job_status = ::std::option::Option::Some(input);
        self
    }
    /// <p> The current status of the import job. </p>
    pub fn set_job_status(
        mut self,
        input: ::std::option::Option<crate::types::TableDataImportJobStatus>,
    ) -> Self {
        self.job_status = input;
        self
    }
    /// <p> A message providing more details about the current status of the import job. </p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> A message providing more details about the current status of the import job. </p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p> The metadata about the job that was submitted for import. </p>
    pub fn job_metadata(mut self, input: crate::types::TableDataImportJobMetadata) -> Self {
        self.job_metadata = ::std::option::Option::Some(input);
        self
    }
    /// <p> The metadata about the job that was submitted for import. </p>
    pub fn set_job_metadata(
        mut self,
        input: ::std::option::Option<crate::types::TableDataImportJobMetadata>,
    ) -> Self {
        self.job_metadata = input;
        self
    }
    /// <p> If job status is failed, error code to understand reason for the failure. </p>
    pub fn error_code(mut self, input: crate::types::ErrorCode) -> Self {
        self.error_code = ::std::option::Option::Some(input);
        self
    }
    /// <p> If job status is failed, error code to understand reason for the failure. </p>
    pub fn set_error_code(mut self, input: ::std::option::Option<crate::types::ErrorCode>) -> Self {
        self.error_code = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeTableDataImportJobOutput`](crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput {
        crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput {
            job_status: self.job_status,
            message: self.message,
            job_metadata: self.job_metadata,
            error_code: self.error_code,
            _request_id: self._request_id,
        }
    }
}
