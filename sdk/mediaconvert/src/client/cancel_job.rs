// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelJob`](crate::operation::cancel_job::builders::CancelJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::cancel_job::builders::CancelJobFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::cancel_job::builders::CancelJobFluentBuilder::set_id): The Job ID of the job to be cancelled.
    /// - On success, responds with [`CancelJobOutput`](crate::operation::cancel_job::CancelJobOutput)
    /// - On failure, responds with [`SdkError<CancelJobError>`](crate::operation::cancel_job::CancelJobError)
    pub fn cancel_job(&self) -> crate::operation::cancel_job::builders::CancelJobFluentBuilder {
        crate::operation::cancel_job::builders::CancelJobFluentBuilder::new(self.handle.clone())
    }
}
