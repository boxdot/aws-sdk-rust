// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The metric you want to retain. Dimensions are optional.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MetricToRetain {
    /// <p>What is measured by the behavior.</p>
    #[doc(hidden)]
    pub metric: ::std::option::Option<::std::string::String>,
    /// <p>The dimension of a metric. This can't be used with custom metrics.</p>
    #[doc(hidden)]
    pub metric_dimension: ::std::option::Option<crate::types::MetricDimension>,
}
impl MetricToRetain {
    /// <p>What is measured by the behavior.</p>
    pub fn metric(&self) -> ::std::option::Option<&str> {
        self.metric.as_deref()
    }
    /// <p>The dimension of a metric. This can't be used with custom metrics.</p>
    pub fn metric_dimension(&self) -> ::std::option::Option<&crate::types::MetricDimension> {
        self.metric_dimension.as_ref()
    }
}
impl MetricToRetain {
    /// Creates a new builder-style object to manufacture [`MetricToRetain`](crate::types::MetricToRetain).
    pub fn builder() -> crate::types::builders::MetricToRetainBuilder {
        crate::types::builders::MetricToRetainBuilder::default()
    }
}

/// A builder for [`MetricToRetain`](crate::types::MetricToRetain).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MetricToRetainBuilder {
    pub(crate) metric: ::std::option::Option<::std::string::String>,
    pub(crate) metric_dimension: ::std::option::Option<crate::types::MetricDimension>,
}
impl MetricToRetainBuilder {
    /// <p>What is measured by the behavior.</p>
    pub fn metric(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.metric = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>What is measured by the behavior.</p>
    pub fn set_metric(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.metric = input;
        self
    }
    /// <p>The dimension of a metric. This can't be used with custom metrics.</p>
    pub fn metric_dimension(mut self, input: crate::types::MetricDimension) -> Self {
        self.metric_dimension = ::std::option::Option::Some(input);
        self
    }
    /// <p>The dimension of a metric. This can't be used with custom metrics.</p>
    pub fn set_metric_dimension(
        mut self,
        input: ::std::option::Option<crate::types::MetricDimension>,
    ) -> Self {
        self.metric_dimension = input;
        self
    }
    /// Consumes the builder and constructs a [`MetricToRetain`](crate::types::MetricToRetain).
    pub fn build(self) -> crate::types::MetricToRetain {
        crate::types::MetricToRetain {
            metric: self.metric,
            metric_dimension: self.metric_dimension,
        }
    }
}
