// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteImagePipeline`](crate::operation::delete_image_pipeline::builders::DeleteImagePipelineFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`image_pipeline_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_image_pipeline::builders::DeleteImagePipelineFluentBuilder::image_pipeline_arn) / [`set_image_pipeline_arn(Option<String>)`](crate::operation::delete_image_pipeline::builders::DeleteImagePipelineFluentBuilder::set_image_pipeline_arn): <p>The Amazon Resource Name (ARN) of the image pipeline to delete.</p>
    /// - On success, responds with [`DeleteImagePipelineOutput`](crate::operation::delete_image_pipeline::DeleteImagePipelineOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::delete_image_pipeline::DeleteImagePipelineOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`image_pipeline_arn(Option<String>)`](crate::operation::delete_image_pipeline::DeleteImagePipelineOutput::image_pipeline_arn): <p>The Amazon Resource Name (ARN) of the image pipeline that was deleted.</p>
    /// - On failure, responds with [`SdkError<DeleteImagePipelineError>`](crate::operation::delete_image_pipeline::DeleteImagePipelineError)
    pub fn delete_image_pipeline(
        &self,
    ) -> crate::operation::delete_image_pipeline::builders::DeleteImagePipelineFluentBuilder {
        crate::operation::delete_image_pipeline::builders::DeleteImagePipelineFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
