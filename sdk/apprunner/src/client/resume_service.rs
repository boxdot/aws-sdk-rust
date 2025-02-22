// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ResumeService`](crate::operation::resume_service::builders::ResumeServiceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`service_arn(impl ::std::convert::Into<String>)`](crate::operation::resume_service::builders::ResumeServiceFluentBuilder::service_arn) / [`set_service_arn(Option<String>)`](crate::operation::resume_service::builders::ResumeServiceFluentBuilder::set_service_arn): <p>The Amazon Resource Name (ARN) of the App Runner service that you want to resume.</p>
    /// - On success, responds with [`ResumeServiceOutput`](crate::operation::resume_service::ResumeServiceOutput) with field(s):
    ///   - [`service(Option<Service>)`](crate::operation::resume_service::ResumeServiceOutput::service): <p>A description of the App Runner service that this request just resumed.</p>
    ///   - [`operation_id(Option<String>)`](crate::operation::resume_service::ResumeServiceOutput::operation_id): <p>The unique ID of the asynchronous operation that this request started. You can use it combined with the <code>ListOperations</code> call to track the operation's progress.</p>
    /// - On failure, responds with [`SdkError<ResumeServiceError>`](crate::operation::resume_service::ResumeServiceError)
    pub fn resume_service(
        &self,
    ) -> crate::operation::resume_service::builders::ResumeServiceFluentBuilder {
        crate::operation::resume_service::builders::ResumeServiceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
