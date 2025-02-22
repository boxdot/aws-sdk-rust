// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetEnvironment`](crate::operation::get_environment::builders::GetEnvironmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::get_environment::builders::GetEnvironmentFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::get_environment::builders::GetEnvironmentFluentBuilder::set_name): <p>The name of the environment that you want to get the detailed data for.</p>
    /// - On success, responds with [`GetEnvironmentOutput`](crate::operation::get_environment::GetEnvironmentOutput) with field(s):
    ///   - [`environment(Option<Environment>)`](crate::operation::get_environment::GetEnvironmentOutput::environment): <p>The detailed data of the requested environment.</p>
    /// - On failure, responds with [`SdkError<GetEnvironmentError>`](crate::operation::get_environment::GetEnvironmentError)
    pub fn get_environment(
        &self,
    ) -> crate::operation::get_environment::builders::GetEnvironmentFluentBuilder {
        crate::operation::get_environment::builders::GetEnvironmentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
