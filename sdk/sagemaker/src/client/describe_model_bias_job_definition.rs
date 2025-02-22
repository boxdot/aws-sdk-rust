// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeModelBiasJobDefinition`](crate::operation::describe_model_bias_job_definition::builders::DescribeModelBiasJobDefinitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`job_definition_name(impl ::std::convert::Into<String>)`](crate::operation::describe_model_bias_job_definition::builders::DescribeModelBiasJobDefinitionFluentBuilder::job_definition_name) / [`set_job_definition_name(Option<String>)`](crate::operation::describe_model_bias_job_definition::builders::DescribeModelBiasJobDefinitionFluentBuilder::set_job_definition_name): <p>The name of the model bias job definition. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p>
    /// - On success, responds with [`DescribeModelBiasJobDefinitionOutput`](crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionOutput) with field(s):
    ///   - [`job_definition_arn(Option<String>)`](crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionOutput::job_definition_arn): <p>The Amazon Resource Name (ARN) of the model bias job.</p>
    ///   - [`job_definition_name(Option<String>)`](crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionOutput::job_definition_name): <p>The name of the bias job definition. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionOutput::creation_time): <p>The time at which the model bias job was created.</p>
    ///   - [`model_bias_baseline_config(Option<ModelBiasBaselineConfig>)`](crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionOutput::model_bias_baseline_config): <p>The baseline configuration for a model bias job.</p>
    ///   - [`model_bias_app_specification(Option<ModelBiasAppSpecification>)`](crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionOutput::model_bias_app_specification): <p>Configures the model bias job to run a specified Docker container image.</p>
    ///   - [`model_bias_job_input(Option<ModelBiasJobInput>)`](crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionOutput::model_bias_job_input): <p>Inputs for the model bias job.</p>
    ///   - [`model_bias_job_output_config(Option<MonitoringOutputConfig>)`](crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionOutput::model_bias_job_output_config): <p>The output configuration for monitoring jobs.</p>
    ///   - [`job_resources(Option<MonitoringResources>)`](crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionOutput::job_resources): <p>Identifies the resources to deploy for a monitoring job.</p>
    ///   - [`network_config(Option<MonitoringNetworkConfig>)`](crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionOutput::network_config): <p>Networking options for a model bias job.</p>
    ///   - [`role_arn(Option<String>)`](crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionOutput::role_arn): <p>The Amazon Resource Name (ARN) of the Amazon Web Services Identity and Access Management (IAM) role that has read permission to the input data location and write permission to the output data location in Amazon S3.</p>
    ///   - [`stopping_condition(Option<MonitoringStoppingCondition>)`](crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionOutput::stopping_condition): <p>A time limit for how long the monitoring job is allowed to run before stopping.</p>
    /// - On failure, responds with [`SdkError<DescribeModelBiasJobDefinitionError>`](crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionError)
    pub fn describe_model_bias_job_definition(&self) -> crate::operation::describe_model_bias_job_definition::builders::DescribeModelBiasJobDefinitionFluentBuilder{
        crate::operation::describe_model_bias_job_definition::builders::DescribeModelBiasJobDefinitionFluentBuilder::new(self.handle.clone())
    }
}
