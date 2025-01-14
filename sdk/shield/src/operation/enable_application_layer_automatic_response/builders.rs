// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_application_layer_automatic_response::_enable_application_layer_automatic_response_output::EnableApplicationLayerAutomaticResponseOutputBuilder;

pub use crate::operation::enable_application_layer_automatic_response::_enable_application_layer_automatic_response_input::EnableApplicationLayerAutomaticResponseInputBuilder;

/// Fluent builder constructing a request to `EnableApplicationLayerAutomaticResponse`.
///
/// <p>Enable the Shield Advanced automatic application layer DDoS mitigation for the protected resource. </p> <note>
/// <p>This feature is available for Amazon CloudFront distributions and Application Load Balancers only.</p>
/// </note>
/// <p>This causes Shield Advanced to create, verify, and apply WAF rules for DDoS attacks that it detects for the resource. Shield Advanced applies the rules in a Shield rule group inside the web ACL that you've associated with the resource. For information about how automatic mitigation works and the requirements for using it, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/ddos-advanced-automatic-app-layer-response.html">Shield Advanced automatic application layer DDoS mitigation</a>.</p> <note>
/// <p>Don't use this action to make changes to automatic mitigation settings when it's already enabled for a resource. Instead, use <code>UpdateApplicationLayerAutomaticResponse</code>.</p>
/// </note>
/// <p>To use this feature, you must associate a web ACL with the protected resource. The web ACL must be created using the latest version of WAF (v2). You can associate the web ACL through the Shield Advanced console at <a href="https://console.aws.amazon.com/wafv2/shieldv2#/">https://console.aws.amazon.com/wafv2/shieldv2#/</a>. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/getting-started-ddos.html">Getting Started with Shield Advanced</a>. You can also associate the web ACL to the resource through the WAF console or the WAF API, but you must manage Shield Advanced automatic mitigation through Shield Advanced. For information about WAF, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">WAF Developer Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EnableApplicationLayerAutomaticResponseFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::enable_application_layer_automatic_response::builders::EnableApplicationLayerAutomaticResponseInputBuilder,
}
impl EnableApplicationLayerAutomaticResponseFluentBuilder {
    /// Creates a new `EnableApplicationLayerAutomaticResponse`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::enable_application_layer_automatic_response::EnableApplicationLayerAutomaticResponse, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::enable_application_layer_automatic_response::EnableApplicationLayerAutomaticResponseError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::enable_application_layer_automatic_response::EnableApplicationLayerAutomaticResponseOutput, ::aws_smithy_http::result::SdkError<crate::operation::enable_application_layer_automatic_response::EnableApplicationLayerAutomaticResponseError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::enable_application_layer_automatic_response::EnableApplicationLayerAutomaticResponseOutput, ::aws_smithy_http::result::SdkError<crate::operation::enable_application_layer_automatic_response::EnableApplicationLayerAutomaticResponseError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::enable_application_layer_automatic_response::EnableApplicationLayerAutomaticResponse, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::enable_application_layer_automatic_response::EnableApplicationLayerAutomaticResponseError>
    >{
        self.customize_middleware().await
    }
    /// <p>The ARN (Amazon Resource Name) of the protected resource.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The ARN (Amazon Resource Name) of the protected resource.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>Specifies the action setting that Shield Advanced should use in the WAF rules that it creates on behalf of the protected resource in response to DDoS attacks. You specify this as part of the configuration for the automatic application layer DDoS mitigation feature, when you enable or update automatic mitigation. Shield Advanced creates the WAF rules in a Shield Advanced-managed rule group, inside the web ACL that you have associated with the resource. </p>
    pub fn action(mut self, input: crate::types::ResponseAction) -> Self {
        self.inner = self.inner.action(input);
        self
    }
    /// <p>Specifies the action setting that Shield Advanced should use in the WAF rules that it creates on behalf of the protected resource in response to DDoS attacks. You specify this as part of the configuration for the automatic application layer DDoS mitigation feature, when you enable or update automatic mitigation. Shield Advanced creates the WAF rules in a Shield Advanced-managed rule group, inside the web ACL that you have associated with the resource. </p>
    pub fn set_action(
        mut self,
        input: ::std::option::Option<crate::types::ResponseAction>,
    ) -> Self {
        self.inner = self.inner.set_action(input);
        self
    }
}
