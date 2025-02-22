// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_metrics::_list_metrics_output::ListMetricsOutputBuilder;

pub use crate::operation::list_metrics::_list_metrics_input::ListMetricsInputBuilder;

/// Fluent builder constructing a request to `ListMetrics`.
///
/// <p>List the specified metrics. You can use the returned metrics with <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetMetricData.html">GetMetricData</a> or <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetMetricStatistics.html">GetMetricStatistics</a> to get statistical data.</p>
/// <p>Up to 500 results are returned for any one call. To retrieve additional results, use the returned token with subsequent calls.</p>
/// <p>After you create a metric, allow up to 15 minutes for the metric to appear. To see metric statistics sooner, use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetMetricData.html">GetMetricData</a> or <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetMetricStatistics.html">GetMetricStatistics</a>.</p>
/// <p>If you are using CloudWatch cross-account observability, you can use this operation in a monitoring account and view metrics from the linked source accounts. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Unified-Cross-Account.html">CloudWatch cross-account observability</a>.</p>
/// <p> <code>ListMetrics</code> doesn't return information about metrics if those metrics haven't reported data in the past two weeks. To retrieve those metrics, use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetMetricData.html">GetMetricData</a> or <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetMetricStatistics.html">GetMetricStatistics</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListMetricsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_metrics::builders::ListMetricsInputBuilder,
}
impl ListMetricsFluentBuilder {
    /// Creates a new `ListMetrics`.
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
            crate::operation::list_metrics::ListMetrics,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_metrics::ListMetricsError>,
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
        crate::operation::list_metrics::ListMetricsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_metrics::ListMetricsError>,
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
        crate::operation::list_metrics::ListMetricsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_metrics::ListMetricsError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_metrics::ListMetrics,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_metrics::ListMetricsError>,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_metrics::paginator::ListMetricsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_metrics::paginator::ListMetricsPaginator {
        crate::operation::list_metrics::paginator::ListMetricsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The metric namespace to filter against. Only the namespace that matches exactly will be returned.</p>
    pub fn namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.namespace(input.into());
        self
    }
    /// <p>The metric namespace to filter against. Only the namespace that matches exactly will be returned.</p>
    pub fn set_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_namespace(input);
        self
    }
    /// <p>The name of the metric to filter against. Only the metrics with names that match exactly will be returned.</p>
    pub fn metric_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metric_name(input.into());
        self
    }
    /// <p>The name of the metric to filter against. Only the metrics with names that match exactly will be returned.</p>
    pub fn set_metric_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_metric_name(input);
        self
    }
    /// Appends an item to `Dimensions`.
    ///
    /// To override the contents of this collection use [`set_dimensions`](Self::set_dimensions).
    ///
    /// <p>The dimensions to filter against. Only the dimensions that match exactly will be returned.</p>
    pub fn dimensions(mut self, input: crate::types::DimensionFilter) -> Self {
        self.inner = self.inner.dimensions(input);
        self
    }
    /// <p>The dimensions to filter against. Only the dimensions that match exactly will be returned.</p>
    pub fn set_dimensions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DimensionFilter>>,
    ) -> Self {
        self.inner = self.inner.set_dimensions(input);
        self
    }
    /// <p>The token returned by a previous call to indicate that there is more data available.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token returned by a previous call to indicate that there is more data available.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>To filter the results to show only metrics that have had data points published in the past three hours, specify this parameter with a value of <code>PT3H</code>. This is the only valid value for this parameter.</p>
    /// <p>The results that are returned are an approximation of the value you specify. There is a low probability that the returned results include metrics with last published data as much as 40 minutes more than the specified time interval.</p>
    pub fn recently_active(mut self, input: crate::types::RecentlyActive) -> Self {
        self.inner = self.inner.recently_active(input);
        self
    }
    /// <p>To filter the results to show only metrics that have had data points published in the past three hours, specify this parameter with a value of <code>PT3H</code>. This is the only valid value for this parameter.</p>
    /// <p>The results that are returned are an approximation of the value you specify. There is a low probability that the returned results include metrics with last published data as much as 40 minutes more than the specified time interval.</p>
    pub fn set_recently_active(
        mut self,
        input: ::std::option::Option<crate::types::RecentlyActive>,
    ) -> Self {
        self.inner = self.inner.set_recently_active(input);
        self
    }
    /// <p>If you are using this operation in a monitoring account, specify <code>true</code> to include metrics from source accounts in the returned data.</p>
    /// <p>The default is <code>false</code>.</p>
    pub fn include_linked_accounts(mut self, input: bool) -> Self {
        self.inner = self.inner.include_linked_accounts(input);
        self
    }
    /// <p>If you are using this operation in a monitoring account, specify <code>true</code> to include metrics from source accounts in the returned data.</p>
    /// <p>The default is <code>false</code>.</p>
    pub fn set_include_linked_accounts(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_include_linked_accounts(input);
        self
    }
    /// <p>When you use this operation in a monitoring account, use this field to return metrics only from one source account. To do so, specify that source account ID in this field, and also specify <code>true</code> for <code>IncludeLinkedAccounts</code>.</p>
    pub fn owning_account(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.owning_account(input.into());
        self
    }
    /// <p>When you use this operation in a monitoring account, use this field to return metrics only from one source account. To do so, specify that source account ID in this field, and also specify <code>true</code> for <code>IncludeLinkedAccounts</code>.</p>
    pub fn set_owning_account(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_owning_account(input);
        self
    }
}
