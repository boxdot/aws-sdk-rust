// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_endpoint::_update_endpoint_output::UpdateEndpointOutputBuilder;

pub use crate::operation::update_endpoint::_update_endpoint_input::UpdateEndpointInputBuilder;

/// Fluent builder constructing a request to `UpdateEndpoint`.
///
/// <p>Update an existing endpoint. For more information about global endpoints, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-global-endpoints.html">Making applications Regional-fault tolerant with global endpoints and event replication</a> in the Amazon EventBridge User Guide..</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateEndpointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_endpoint::builders::UpdateEndpointInputBuilder,
}
impl UpdateEndpointFluentBuilder {
    /// Creates a new `UpdateEndpoint`.
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
            crate::operation::update_endpoint::UpdateEndpoint,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_endpoint::UpdateEndpointError>,
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
        crate::operation::update_endpoint::UpdateEndpointOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_endpoint::UpdateEndpointError>,
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
        crate::operation::update_endpoint::UpdateEndpointOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_endpoint::UpdateEndpointError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_endpoint::UpdateEndpoint,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_endpoint::UpdateEndpointError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the endpoint you want to update.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the endpoint you want to update.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A description for the endpoint.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the endpoint.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Configure the routing policy, including the health check and secondary Region.</p>
    pub fn routing_config(mut self, input: crate::types::RoutingConfig) -> Self {
        self.inner = self.inner.routing_config(input);
        self
    }
    /// <p>Configure the routing policy, including the health check and secondary Region.</p>
    pub fn set_routing_config(
        mut self,
        input: ::std::option::Option<crate::types::RoutingConfig>,
    ) -> Self {
        self.inner = self.inner.set_routing_config(input);
        self
    }
    /// <p>Whether event replication was enabled or disabled by this request.</p>
    pub fn replication_config(mut self, input: crate::types::ReplicationConfig) -> Self {
        self.inner = self.inner.replication_config(input);
        self
    }
    /// <p>Whether event replication was enabled or disabled by this request.</p>
    pub fn set_replication_config(
        mut self,
        input: ::std::option::Option<crate::types::ReplicationConfig>,
    ) -> Self {
        self.inner = self.inner.set_replication_config(input);
        self
    }
    /// Appends an item to `EventBuses`.
    ///
    /// To override the contents of this collection use [`set_event_buses`](Self::set_event_buses).
    ///
    /// <p>Define event buses used for replication.</p>
    pub fn event_buses(mut self, input: crate::types::EndpointEventBus) -> Self {
        self.inner = self.inner.event_buses(input);
        self
    }
    /// <p>Define event buses used for replication.</p>
    pub fn set_event_buses(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EndpointEventBus>>,
    ) -> Self {
        self.inner = self.inner.set_event_buses(input);
        self
    }
    /// <p>The ARN of the role used by event replication for this request.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The ARN of the role used by event replication for this request.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
}
