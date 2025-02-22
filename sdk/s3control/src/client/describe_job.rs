// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeJob`](crate::operation::describe_job::builders::DescribeJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::describe_job::builders::DescribeJobFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::describe_job::builders::DescribeJobFluentBuilder::set_account_id): <p>The Amazon Web Services account ID associated with the S3 Batch Operations job.</p>
    ///   - [`job_id(impl ::std::convert::Into<String>)`](crate::operation::describe_job::builders::DescribeJobFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::describe_job::builders::DescribeJobFluentBuilder::set_job_id): <p>The ID for the job whose information you want to retrieve.</p>
    /// - On success, responds with [`DescribeJobOutput`](crate::operation::describe_job::DescribeJobOutput) with field(s):
    ///   - [`job(Option<JobDescriptor>)`](crate::operation::describe_job::DescribeJobOutput::job): <p>Contains the configuration parameters and status for the job specified in the <code>Describe Job</code> request.</p>
    /// - On failure, responds with [`SdkError<DescribeJobError>`](crate::operation::describe_job::DescribeJobError)
    pub fn describe_job(
        &self,
    ) -> crate::operation::describe_job::builders::DescribeJobFluentBuilder {
        crate::operation::describe_job::builders::DescribeJobFluentBuilder::new(self.handle.clone())
    }
}
