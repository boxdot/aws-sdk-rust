// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_launch_profile::_update_launch_profile_output::UpdateLaunchProfileOutputBuilder;

pub use crate::operation::update_launch_profile::_update_launch_profile_input::UpdateLaunchProfileInputBuilder;

/// Fluent builder constructing a request to `UpdateLaunchProfile`.
///
/// <p>Update a launch profile.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateLaunchProfileFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_launch_profile::builders::UpdateLaunchProfileInputBuilder,
}
impl UpdateLaunchProfileFluentBuilder {
    /// Creates a new `UpdateLaunchProfile`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_launch_profile::UpdateLaunchProfile,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_launch_profile::UpdateLaunchProfileError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_launch_profile::UpdateLaunchProfileOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_launch_profile::UpdateLaunchProfileError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_launch_profile::UpdateLaunchProfileOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_launch_profile::UpdateLaunchProfileError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_launch_profile::UpdateLaunchProfile,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_launch_profile::UpdateLaunchProfileError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The ID of the launch profile used to control access from the streaming session.</p>
    pub fn launch_profile_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.launch_profile_id(input.into());
        self
    }
    /// <p>The ID of the launch profile used to control access from the streaming session.</p>
    pub fn set_launch_profile_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_launch_profile_id(input);
        self
    }
    /// Appends an item to `launchProfileProtocolVersions`.
    ///
    /// To override the contents of this collection use [`set_launch_profile_protocol_versions`](Self::set_launch_profile_protocol_versions).
    ///
    /// <p>The version number of the protocol that is used by the launch profile. The only valid version is "2021-03-31".</p>
    pub fn launch_profile_protocol_versions(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.launch_profile_protocol_versions(input.into());
        self
    }
    /// <p>The version number of the protocol that is used by the launch profile. The only valid version is "2021-03-31".</p>
    pub fn set_launch_profile_protocol_versions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_launch_profile_protocol_versions(input);
        self
    }
    /// <p>The name for the launch profile.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name for the launch profile.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A configuration for a streaming session.</p>
    pub fn stream_configuration(mut self, input: crate::types::StreamConfigurationCreate) -> Self {
        self.inner = self.inner.stream_configuration(input);
        self
    }
    /// <p>A configuration for a streaming session.</p>
    pub fn set_stream_configuration(
        mut self,
        input: ::std::option::Option<crate::types::StreamConfigurationCreate>,
    ) -> Self {
        self.inner = self.inner.set_stream_configuration(input);
        self
    }
    /// Appends an item to `studioComponentIds`.
    ///
    /// To override the contents of this collection use [`set_studio_component_ids`](Self::set_studio_component_ids).
    ///
    /// <p>Unique identifiers for a collection of studio components that can be used with this launch profile.</p>
    pub fn studio_component_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.studio_component_ids(input.into());
        self
    }
    /// <p>Unique identifiers for a collection of studio components that can be used with this launch profile.</p>
    pub fn set_studio_component_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_studio_component_ids(input);
        self
    }
    /// <p>The studio ID. </p>
    pub fn studio_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.studio_id(input.into());
        self
    }
    /// <p>The studio ID. </p>
    pub fn set_studio_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_studio_id(input);
        self
    }
}
