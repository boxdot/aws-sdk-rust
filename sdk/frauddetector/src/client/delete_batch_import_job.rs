// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBatchImportJob`](crate::operation::delete_batch_import_job::builders::DeleteBatchImportJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`job_id(impl ::std::convert::Into<String>)`](crate::operation::delete_batch_import_job::builders::DeleteBatchImportJobFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::delete_batch_import_job::builders::DeleteBatchImportJobFluentBuilder::set_job_id): <p>The ID of the batch import job to delete. </p>
    /// - On success, responds with [`DeleteBatchImportJobOutput`](crate::operation::delete_batch_import_job::DeleteBatchImportJobOutput)
    /// - On failure, responds with [`SdkError<DeleteBatchImportJobError>`](crate::operation::delete_batch_import_job::DeleteBatchImportJobError)
    pub fn delete_batch_import_job(
        &self,
    ) -> crate::operation::delete_batch_import_job::builders::DeleteBatchImportJobFluentBuilder
    {
        crate::operation::delete_batch_import_job::builders::DeleteBatchImportJobFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
