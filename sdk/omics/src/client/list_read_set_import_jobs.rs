// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListReadSetImportJobs`](crate::operation::list_read_set_import_jobs::builders::ListReadSetImportJobsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_read_set_import_jobs::builders::ListReadSetImportJobsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_read_set_import_jobs::builders::ListReadSetImportJobsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_read_set_import_jobs::builders::ListReadSetImportJobsFluentBuilder::set_max_results): <p>The maximum number of jobs to return in one page of results.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_read_set_import_jobs::builders::ListReadSetImportJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_read_set_import_jobs::builders::ListReadSetImportJobsFluentBuilder::set_next_token): <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    ///   - [`sequence_store_id(impl ::std::convert::Into<String>)`](crate::operation::list_read_set_import_jobs::builders::ListReadSetImportJobsFluentBuilder::sequence_store_id) / [`set_sequence_store_id(Option<String>)`](crate::operation::list_read_set_import_jobs::builders::ListReadSetImportJobsFluentBuilder::set_sequence_store_id): <p>The jobs' sequence store ID.</p>
    ///   - [`filter(ImportReadSetFilter)`](crate::operation::list_read_set_import_jobs::builders::ListReadSetImportJobsFluentBuilder::filter) / [`set_filter(Option<ImportReadSetFilter>)`](crate::operation::list_read_set_import_jobs::builders::ListReadSetImportJobsFluentBuilder::set_filter): <p>A filter to apply to the list.</p>
    /// - On success, responds with [`ListReadSetImportJobsOutput`](crate::operation::list_read_set_import_jobs::ListReadSetImportJobsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_read_set_import_jobs::ListReadSetImportJobsOutput::next_token): <p>A pagination token that's included if more results are available.</p>
    ///   - [`import_jobs(Option<Vec<ImportReadSetJobItem>>)`](crate::operation::list_read_set_import_jobs::ListReadSetImportJobsOutput::import_jobs): <p>A list of jobs.</p>
    /// - On failure, responds with [`SdkError<ListReadSetImportJobsError>`](crate::operation::list_read_set_import_jobs::ListReadSetImportJobsError)
    pub fn list_read_set_import_jobs(
        &self,
    ) -> crate::operation::list_read_set_import_jobs::builders::ListReadSetImportJobsFluentBuilder
    {
        crate::operation::list_read_set_import_jobs::builders::ListReadSetImportJobsFluentBuilder::new(self.handle.clone())
    }
}
