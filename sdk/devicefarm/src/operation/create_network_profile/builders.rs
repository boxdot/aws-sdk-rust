// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_network_profile::_create_network_profile_output::CreateNetworkProfileOutputBuilder;

pub use crate::operation::create_network_profile::_create_network_profile_input::CreateNetworkProfileInputBuilder;

/// Fluent builder constructing a request to `CreateNetworkProfile`.
///
/// <p>Creates a network profile.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateNetworkProfileFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_network_profile::builders::CreateNetworkProfileInputBuilder,
}
impl CreateNetworkProfileFluentBuilder {
    /// Creates a new `CreateNetworkProfile`.
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
            crate::operation::create_network_profile::CreateNetworkProfile,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_network_profile::CreateNetworkProfileError,
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
        crate::operation::create_network_profile::CreateNetworkProfileOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_network_profile::CreateNetworkProfileError,
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
        crate::operation::create_network_profile::CreateNetworkProfileOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_network_profile::CreateNetworkProfileError,
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
            crate::operation::create_network_profile::CreateNetworkProfile,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_network_profile::CreateNetworkProfileError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to create a network profile.</p>
    pub fn project_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to create a network profile.</p>
    pub fn set_project_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_arn(input);
        self
    }
    /// <p>The name for the new network profile.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name for the new network profile.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The description of the network profile.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the network profile.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The type of network profile to create. Valid values are listed here.</p>
    pub fn r#type(mut self, input: crate::types::NetworkProfileType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of network profile to create. Valid values are listed here.</p>
    pub fn set_type(
        mut self,
        input: ::std::option::Option<crate::types::NetworkProfileType>,
    ) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    pub fn uplink_bandwidth_bits(mut self, input: i64) -> Self {
        self.inner = self.inner.uplink_bandwidth_bits(input);
        self
    }
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    pub fn set_uplink_bandwidth_bits(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_uplink_bandwidth_bits(input);
        self
    }
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    pub fn downlink_bandwidth_bits(mut self, input: i64) -> Self {
        self.inner = self.inner.downlink_bandwidth_bits(input);
        self
    }
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    pub fn set_downlink_bandwidth_bits(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_downlink_bandwidth_bits(input);
        self
    }
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    pub fn uplink_delay_ms(mut self, input: i64) -> Self {
        self.inner = self.inner.uplink_delay_ms(input);
        self
    }
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    pub fn set_uplink_delay_ms(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_uplink_delay_ms(input);
        self
    }
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    pub fn downlink_delay_ms(mut self, input: i64) -> Self {
        self.inner = self.inner.downlink_delay_ms(input);
        self
    }
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    pub fn set_downlink_delay_ms(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_downlink_delay_ms(input);
        self
    }
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    pub fn uplink_jitter_ms(mut self, input: i64) -> Self {
        self.inner = self.inner.uplink_jitter_ms(input);
        self
    }
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    pub fn set_uplink_jitter_ms(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_uplink_jitter_ms(input);
        self
    }
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    pub fn downlink_jitter_ms(mut self, input: i64) -> Self {
        self.inner = self.inner.downlink_jitter_ms(input);
        self
    }
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    pub fn set_downlink_jitter_ms(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_downlink_jitter_ms(input);
        self
    }
    /// <p>Proportion of transmitted packets that fail to arrive from 0 to 100 percent.</p>
    pub fn uplink_loss_percent(mut self, input: i32) -> Self {
        self.inner = self.inner.uplink_loss_percent(input);
        self
    }
    /// <p>Proportion of transmitted packets that fail to arrive from 0 to 100 percent.</p>
    pub fn set_uplink_loss_percent(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_uplink_loss_percent(input);
        self
    }
    /// <p>Proportion of received packets that fail to arrive from 0 to 100 percent.</p>
    pub fn downlink_loss_percent(mut self, input: i32) -> Self {
        self.inner = self.inner.downlink_loss_percent(input);
        self
    }
    /// <p>Proportion of received packets that fail to arrive from 0 to 100 percent.</p>
    pub fn set_downlink_loss_percent(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_downlink_loss_percent(input);
        self
    }
}
