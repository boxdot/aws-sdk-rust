// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteResourceTree`](crate::operation::delete_resource_tree::builders::DeleteResourceTreeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_resource_tree::builders::DeleteResourceTreeFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::delete_resource_tree::builders::DeleteResourceTreeFluentBuilder::set_resource_arn): <p>The Amazon Resource Name (ARN) of the parent resource to delete. All child resources of the parent resource will also be deleted.</p>
    /// - On success, responds with [`DeleteResourceTreeOutput`](crate::operation::delete_resource_tree::DeleteResourceTreeOutput)
    /// - On failure, responds with [`SdkError<DeleteResourceTreeError>`](crate::operation::delete_resource_tree::DeleteResourceTreeError)
    pub fn delete_resource_tree(
        &self,
    ) -> crate::operation::delete_resource_tree::builders::DeleteResourceTreeFluentBuilder {
        crate::operation::delete_resource_tree::builders::DeleteResourceTreeFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
