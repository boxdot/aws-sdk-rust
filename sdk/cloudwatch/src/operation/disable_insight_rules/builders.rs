// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_insight_rules::_disable_insight_rules_output::DisableInsightRulesOutputBuilder;

pub use crate::operation::disable_insight_rules::_disable_insight_rules_input::DisableInsightRulesInputBuilder;

/// Fluent builder constructing a request to `DisableInsightRules`.
///
/// <p>Disables the specified Contributor Insights rules. When rules are disabled, they do not analyze log groups and do not incur costs.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisableInsightRulesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_insight_rules::builders::DisableInsightRulesInputBuilder,
}
impl DisableInsightRulesFluentBuilder {
    /// Creates a new `DisableInsightRules`.
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
            crate::operation::disable_insight_rules::DisableInsightRules,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_insight_rules::DisableInsightRulesError,
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
        crate::operation::disable_insight_rules::DisableInsightRulesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_insight_rules::DisableInsightRulesError,
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
        crate::operation::disable_insight_rules::DisableInsightRulesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_insight_rules::DisableInsightRulesError,
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
            crate::operation::disable_insight_rules::DisableInsightRules,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_insight_rules::DisableInsightRulesError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `RuleNames`.
    ///
    /// To override the contents of this collection use [`set_rule_names`](Self::set_rule_names).
    ///
    /// <p>An array of the rule names to disable. If you need to find out the names of your rules, use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeInsightRules.html">DescribeInsightRules</a>.</p>
    pub fn rule_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rule_names(input.into());
        self
    }
    /// <p>An array of the rule names to disable. If you need to find out the names of your rules, use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeInsightRules.html">DescribeInsightRules</a>.</p>
    pub fn set_rule_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_rule_names(input);
        self
    }
}
