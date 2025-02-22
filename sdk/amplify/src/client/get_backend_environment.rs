// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBackendEnvironment`](crate::operation::get_backend_environment::builders::GetBackendEnvironmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl ::std::convert::Into<String>)`](crate::operation::get_backend_environment::builders::GetBackendEnvironmentFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::get_backend_environment::builders::GetBackendEnvironmentFluentBuilder::set_app_id): <p> The unique id for an Amplify app. </p>
    ///   - [`environment_name(impl ::std::convert::Into<String>)`](crate::operation::get_backend_environment::builders::GetBackendEnvironmentFluentBuilder::environment_name) / [`set_environment_name(Option<String>)`](crate::operation::get_backend_environment::builders::GetBackendEnvironmentFluentBuilder::set_environment_name): <p> The name for the backend environment. </p>
    /// - On success, responds with [`GetBackendEnvironmentOutput`](crate::operation::get_backend_environment::GetBackendEnvironmentOutput) with field(s):
    ///   - [`backend_environment(Option<BackendEnvironment>)`](crate::operation::get_backend_environment::GetBackendEnvironmentOutput::backend_environment): <p> Describes the backend environment for an Amplify app. </p>
    /// - On failure, responds with [`SdkError<GetBackendEnvironmentError>`](crate::operation::get_backend_environment::GetBackendEnvironmentError)
    pub fn get_backend_environment(
        &self,
    ) -> crate::operation::get_backend_environment::builders::GetBackendEnvironmentFluentBuilder
    {
        crate::operation::get_backend_environment::builders::GetBackendEnvironmentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
