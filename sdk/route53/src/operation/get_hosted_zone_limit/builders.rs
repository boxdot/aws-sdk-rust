// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_hosted_zone_limit::_get_hosted_zone_limit_output::GetHostedZoneLimitOutputBuilder;

pub use crate::operation::get_hosted_zone_limit::_get_hosted_zone_limit_input::GetHostedZoneLimitInputBuilder;

/// Fluent builder constructing a request to `GetHostedZoneLimit`.
///
/// <p>Gets the specified limit for a specified hosted zone, for example, the maximum number of records that you can create in the hosted zone. </p>
/// <p>For the default limit, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>. To request a higher limit, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-route53">open a case</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetHostedZoneLimitFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_hosted_zone_limit::builders::GetHostedZoneLimitInputBuilder,
}
impl GetHostedZoneLimitFluentBuilder {
    /// Creates a new `GetHostedZoneLimit`.
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
            crate::operation::get_hosted_zone_limit::GetHostedZoneLimit,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError,
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
        crate::operation::get_hosted_zone_limit::GetHostedZoneLimitOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError,
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
        crate::operation::get_hosted_zone_limit::GetHostedZoneLimitOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError,
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
            crate::operation::get_hosted_zone_limit::GetHostedZoneLimit,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The limit that you want to get. Valid values include the following:</p>
    /// <ul>
    /// <li> <p> <b>MAX_RRSETS_BY_ZONE</b>: The maximum number of records that you can create in the specified hosted zone.</p> </li>
    /// <li> <p> <b>MAX_VPCS_ASSOCIATED_BY_ZONE</b>: The maximum number of Amazon VPCs that you can associate with the specified private hosted zone.</p> </li>
    /// </ul>
    pub fn r#type(mut self, input: crate::types::HostedZoneLimitType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The limit that you want to get. Valid values include the following:</p>
    /// <ul>
    /// <li> <p> <b>MAX_RRSETS_BY_ZONE</b>: The maximum number of records that you can create in the specified hosted zone.</p> </li>
    /// <li> <p> <b>MAX_VPCS_ASSOCIATED_BY_ZONE</b>: The maximum number of Amazon VPCs that you can associate with the specified private hosted zone.</p> </li>
    /// </ul>
    pub fn set_type(
        mut self,
        input: ::std::option::Option<crate::types::HostedZoneLimitType>,
    ) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The ID of the hosted zone that you want to get a limit for.</p>
    pub fn hosted_zone_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.hosted_zone_id(input.into());
        self
    }
    /// <p>The ID of the hosted zone that you want to get a limit for.</p>
    pub fn set_hosted_zone_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_hosted_zone_id(input);
        self
    }
}
