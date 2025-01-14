// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_recommendations::_start_recommendations_output::StartRecommendationsOutputBuilder;

pub use crate::operation::start_recommendations::_start_recommendations_input::StartRecommendationsInputBuilder;

/// Fluent builder constructing a request to `StartRecommendations`.
///
/// <p>Starts the analysis of your source database to provide recommendations of target engines.</p>
/// <p>You can create recommendations for multiple source databases using <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_BatchStartRecommendations.html">BatchStartRecommendations</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartRecommendationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_recommendations::builders::StartRecommendationsInputBuilder,
}
impl StartRecommendationsFluentBuilder {
    /// Creates a new `StartRecommendations`.
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
            crate::operation::start_recommendations::StartRecommendations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_recommendations::StartRecommendationsError,
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
        crate::operation::start_recommendations::StartRecommendationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_recommendations::StartRecommendationsError,
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
        crate::operation::start_recommendations::StartRecommendationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_recommendations::StartRecommendationsError,
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
            crate::operation::start_recommendations::StartRecommendations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_recommendations::StartRecommendationsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The identifier of the source database to analyze and provide recommendations for.</p>
    pub fn database_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.database_id(input.into());
        self
    }
    /// <p>The identifier of the source database to analyze and provide recommendations for.</p>
    pub fn set_database_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_database_id(input);
        self
    }
    /// <p>The settings in JSON format that Fleet Advisor uses to determine target engine recommendations. These parameters include target instance sizing and availability and durability settings. For target instance sizing, Fleet Advisor supports the following two options: total capacity and resource utilization. For availability and durability, Fleet Advisor supports the following two options: production (Multi-AZ deployments) and Dev/Test (Single-AZ deployments).</p>
    pub fn settings(mut self, input: crate::types::RecommendationSettings) -> Self {
        self.inner = self.inner.settings(input);
        self
    }
    /// <p>The settings in JSON format that Fleet Advisor uses to determine target engine recommendations. These parameters include target instance sizing and availability and durability settings. For target instance sizing, Fleet Advisor supports the following two options: total capacity and resource utilization. For availability and durability, Fleet Advisor supports the following two options: production (Multi-AZ deployments) and Dev/Test (Single-AZ deployments).</p>
    pub fn set_settings(
        mut self,
        input: ::std::option::Option<crate::types::RecommendationSettings>,
    ) -> Self {
        self.inner = self.inner.set_settings(input);
        self
    }
}
