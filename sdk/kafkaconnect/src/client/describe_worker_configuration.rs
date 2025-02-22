// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeWorkerConfiguration`](crate::operation::describe_worker_configuration::builders::DescribeWorkerConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`worker_configuration_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_worker_configuration::builders::DescribeWorkerConfigurationFluentBuilder::worker_configuration_arn) / [`set_worker_configuration_arn(Option<String>)`](crate::operation::describe_worker_configuration::builders::DescribeWorkerConfigurationFluentBuilder::set_worker_configuration_arn): <p>The Amazon Resource Name (ARN) of the worker configuration that you want to get information about.</p>
    /// - On success, responds with [`DescribeWorkerConfigurationOutput`](crate::operation::describe_worker_configuration::DescribeWorkerConfigurationOutput) with field(s):
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::describe_worker_configuration::DescribeWorkerConfigurationOutput::creation_time): <p>The time that the worker configuration was created.</p>
    ///   - [`description(Option<String>)`](crate::operation::describe_worker_configuration::DescribeWorkerConfigurationOutput::description): <p>The description of the worker configuration.</p>
    ///   - [`latest_revision(Option<WorkerConfigurationRevisionDescription>)`](crate::operation::describe_worker_configuration::DescribeWorkerConfigurationOutput::latest_revision): <p>The latest revision of the custom configuration.</p>
    ///   - [`name(Option<String>)`](crate::operation::describe_worker_configuration::DescribeWorkerConfigurationOutput::name): <p>The name of the worker configuration.</p>
    ///   - [`worker_configuration_arn(Option<String>)`](crate::operation::describe_worker_configuration::DescribeWorkerConfigurationOutput::worker_configuration_arn): <p>The Amazon Resource Name (ARN) of the custom configuration.</p>
    /// - On failure, responds with [`SdkError<DescribeWorkerConfigurationError>`](crate::operation::describe_worker_configuration::DescribeWorkerConfigurationError)
    pub fn describe_worker_configuration(&self) -> crate::operation::describe_worker_configuration::builders::DescribeWorkerConfigurationFluentBuilder{
        crate::operation::describe_worker_configuration::builders::DescribeWorkerConfigurationFluentBuilder::new(self.handle.clone())
    }
}
