// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateEnvironmentAccountConnectionOutput {
    /// <p>The environment account connection detail data that's returned by Proton.</p>
    #[doc(hidden)]
    pub environment_account_connection:
        ::std::option::Option<crate::types::EnvironmentAccountConnection>,
    _request_id: Option<String>,
}
impl CreateEnvironmentAccountConnectionOutput {
    /// <p>The environment account connection detail data that's returned by Proton.</p>
    pub fn environment_account_connection(
        &self,
    ) -> ::std::option::Option<&crate::types::EnvironmentAccountConnection> {
        self.environment_account_connection.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for CreateEnvironmentAccountConnectionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateEnvironmentAccountConnectionOutput {
    /// Creates a new builder-style object to manufacture [`CreateEnvironmentAccountConnectionOutput`](crate::operation::create_environment_account_connection::CreateEnvironmentAccountConnectionOutput).
    pub fn builder() -> crate::operation::create_environment_account_connection::builders::CreateEnvironmentAccountConnectionOutputBuilder{
        crate::operation::create_environment_account_connection::builders::CreateEnvironmentAccountConnectionOutputBuilder::default()
    }
}

/// A builder for [`CreateEnvironmentAccountConnectionOutput`](crate::operation::create_environment_account_connection::CreateEnvironmentAccountConnectionOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateEnvironmentAccountConnectionOutputBuilder {
    pub(crate) environment_account_connection:
        ::std::option::Option<crate::types::EnvironmentAccountConnection>,
    _request_id: Option<String>,
}
impl CreateEnvironmentAccountConnectionOutputBuilder {
    /// <p>The environment account connection detail data that's returned by Proton.</p>
    pub fn environment_account_connection(
        mut self,
        input: crate::types::EnvironmentAccountConnection,
    ) -> Self {
        self.environment_account_connection = ::std::option::Option::Some(input);
        self
    }
    /// <p>The environment account connection detail data that's returned by Proton.</p>
    pub fn set_environment_account_connection(
        mut self,
        input: ::std::option::Option<crate::types::EnvironmentAccountConnection>,
    ) -> Self {
        self.environment_account_connection = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateEnvironmentAccountConnectionOutput`](crate::operation::create_environment_account_connection::CreateEnvironmentAccountConnectionOutput).
    pub fn build(self) -> crate::operation::create_environment_account_connection::CreateEnvironmentAccountConnectionOutput{
        crate::operation::create_environment_account_connection::CreateEnvironmentAccountConnectionOutput {
            environment_account_connection: self.environment_account_connection
            ,
            _request_id: self._request_id,
        }
    }
}
