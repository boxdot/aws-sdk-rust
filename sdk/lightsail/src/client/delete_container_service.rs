// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteContainerService`](crate::operation::delete_container_service::builders::DeleteContainerServiceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`service_name(impl ::std::convert::Into<String>)`](crate::operation::delete_container_service::builders::DeleteContainerServiceFluentBuilder::service_name) / [`set_service_name(Option<String>)`](crate::operation::delete_container_service::builders::DeleteContainerServiceFluentBuilder::set_service_name): <p>The name of the container service to delete.</p>
    /// - On success, responds with [`DeleteContainerServiceOutput`](crate::operation::delete_container_service::DeleteContainerServiceOutput)
    /// - On failure, responds with [`SdkError<DeleteContainerServiceError>`](crate::operation::delete_container_service::DeleteContainerServiceError)
    pub fn delete_container_service(
        &self,
    ) -> crate::operation::delete_container_service::builders::DeleteContainerServiceFluentBuilder
    {
        crate::operation::delete_container_service::builders::DeleteContainerServiceFluentBuilder::new(self.handle.clone())
    }
}
