// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdatePipelineExecution`](crate::operation::update_pipeline_execution::builders::UpdatePipelineExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`pipeline_execution_arn(impl ::std::convert::Into<String>)`](crate::operation::update_pipeline_execution::builders::UpdatePipelineExecutionFluentBuilder::pipeline_execution_arn) / [`set_pipeline_execution_arn(Option<String>)`](crate::operation::update_pipeline_execution::builders::UpdatePipelineExecutionFluentBuilder::set_pipeline_execution_arn): <p>The Amazon Resource Name (ARN) of the pipeline execution.</p>
    ///   - [`pipeline_execution_description(impl ::std::convert::Into<String>)`](crate::operation::update_pipeline_execution::builders::UpdatePipelineExecutionFluentBuilder::pipeline_execution_description) / [`set_pipeline_execution_description(Option<String>)`](crate::operation::update_pipeline_execution::builders::UpdatePipelineExecutionFluentBuilder::set_pipeline_execution_description): <p>The description of the pipeline execution.</p>
    ///   - [`pipeline_execution_display_name(impl ::std::convert::Into<String>)`](crate::operation::update_pipeline_execution::builders::UpdatePipelineExecutionFluentBuilder::pipeline_execution_display_name) / [`set_pipeline_execution_display_name(Option<String>)`](crate::operation::update_pipeline_execution::builders::UpdatePipelineExecutionFluentBuilder::set_pipeline_execution_display_name): <p>The display name of the pipeline execution.</p>
    ///   - [`parallelism_configuration(ParallelismConfiguration)`](crate::operation::update_pipeline_execution::builders::UpdatePipelineExecutionFluentBuilder::parallelism_configuration) / [`set_parallelism_configuration(Option<ParallelismConfiguration>)`](crate::operation::update_pipeline_execution::builders::UpdatePipelineExecutionFluentBuilder::set_parallelism_configuration): <p>This configuration, if specified, overrides the parallelism configuration of the parent pipeline for this specific run.</p>
    /// - On success, responds with [`UpdatePipelineExecutionOutput`](crate::operation::update_pipeline_execution::UpdatePipelineExecutionOutput) with field(s):
    ///   - [`pipeline_execution_arn(Option<String>)`](crate::operation::update_pipeline_execution::UpdatePipelineExecutionOutput::pipeline_execution_arn): <p>The Amazon Resource Name (ARN) of the updated pipeline execution.</p>
    /// - On failure, responds with [`SdkError<UpdatePipelineExecutionError>`](crate::operation::update_pipeline_execution::UpdatePipelineExecutionError)
    pub fn update_pipeline_execution(
        &self,
    ) -> crate::operation::update_pipeline_execution::builders::UpdatePipelineExecutionFluentBuilder
    {
        crate::operation::update_pipeline_execution::builders::UpdatePipelineExecutionFluentBuilder::new(self.handle.clone())
    }
}
