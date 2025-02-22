// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_fleet_metric::_create_fleet_metric_output::CreateFleetMetricOutputBuilder;

pub use crate::operation::create_fleet_metric::_create_fleet_metric_input::CreateFleetMetricInputBuilder;

/// Fluent builder constructing a request to `CreateFleetMetric`.
///
/// <p>Creates a fleet metric.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">CreateFleetMetric</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateFleetMetricFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_fleet_metric::builders::CreateFleetMetricInputBuilder,
}
impl CreateFleetMetricFluentBuilder {
    /// Creates a new `CreateFleetMetric`.
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
            crate::operation::create_fleet_metric::CreateFleetMetric,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_fleet_metric::CreateFleetMetricError,
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
        crate::operation::create_fleet_metric::CreateFleetMetricOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_fleet_metric::CreateFleetMetricError,
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
        crate::operation::create_fleet_metric::CreateFleetMetricOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_fleet_metric::CreateFleetMetricError,
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
            crate::operation::create_fleet_metric::CreateFleetMetric,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_fleet_metric::CreateFleetMetricError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the fleet metric to create.</p>
    pub fn metric_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metric_name(input.into());
        self
    }
    /// <p>The name of the fleet metric to create.</p>
    pub fn set_metric_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_metric_name(input);
        self
    }
    /// <p>The search query string.</p>
    pub fn query_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.query_string(input.into());
        self
    }
    /// <p>The search query string.</p>
    pub fn set_query_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_query_string(input);
        self
    }
    /// <p>The type of the aggregation query.</p>
    pub fn aggregation_type(mut self, input: crate::types::AggregationType) -> Self {
        self.inner = self.inner.aggregation_type(input);
        self
    }
    /// <p>The type of the aggregation query.</p>
    pub fn set_aggregation_type(
        mut self,
        input: ::std::option::Option<crate::types::AggregationType>,
    ) -> Self {
        self.inner = self.inner.set_aggregation_type(input);
        self
    }
    /// <p>The time in seconds between fleet metric emissions. Range [60(1 min), 86400(1 day)] and must be multiple of 60.</p>
    pub fn period(mut self, input: i32) -> Self {
        self.inner = self.inner.period(input);
        self
    }
    /// <p>The time in seconds between fleet metric emissions. Range [60(1 min), 86400(1 day)] and must be multiple of 60.</p>
    pub fn set_period(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_period(input);
        self
    }
    /// <p>The field to aggregate.</p>
    pub fn aggregation_field(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.aggregation_field(input.into());
        self
    }
    /// <p>The field to aggregate.</p>
    pub fn set_aggregation_field(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_aggregation_field(input);
        self
    }
    /// <p>The fleet metric description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The fleet metric description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The query version.</p>
    pub fn query_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.query_version(input.into());
        self
    }
    /// <p>The query version.</p>
    pub fn set_query_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_query_version(input);
        self
    }
    /// <p>The name of the index to search.</p>
    pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.index_name(input.into());
        self
    }
    /// <p>The name of the index to search.</p>
    pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_index_name(input);
        self
    }
    /// <p>Used to support unit transformation such as milliseconds to seconds. The unit must be supported by <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_MetricDatum.html">CW metric</a>. Default to null.</p>
    pub fn unit(mut self, input: crate::types::FleetMetricUnit) -> Self {
        self.inner = self.inner.unit(input);
        self
    }
    /// <p>Used to support unit transformation such as milliseconds to seconds. The unit must be supported by <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_MetricDatum.html">CW metric</a>. Default to null.</p>
    pub fn set_unit(mut self, input: ::std::option::Option<crate::types::FleetMetricUnit>) -> Self {
        self.inner = self.inner.set_unit(input);
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Metadata, which can be used to manage the fleet metric.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Metadata, which can be used to manage the fleet metric.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
