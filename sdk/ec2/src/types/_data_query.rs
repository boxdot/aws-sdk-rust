// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A query used for retrieving network health data. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DataQuery {
    /// <p>A user-defined ID associated with a data query that's returned in the <code>dataResponse</code> identifying the query. For example, if you set the Id to <code>MyQuery01</code>in the query, the <code>dataResponse</code> identifies the query as <code>MyQuery01</code>.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The Region or Availability Zone that's the source for the data query. For example, <code>us-east-1</code>.</p>
    #[doc(hidden)]
    pub source: ::std::option::Option<::std::string::String>,
    /// <p>The Region or Availability Zone that's the target for the data query. For example, <code>eu-north-1</code>.</p>
    #[doc(hidden)]
    pub destination: ::std::option::Option<::std::string::String>,
    /// <p>The metric, <code>aggregation-latency</code>, indicating that network latency is aggregated for the query. This is the only supported metric.</p>
    #[doc(hidden)]
    pub metric: ::std::option::Option<crate::types::MetricType>,
    /// <p>The metric data aggregation period, <code>p50</code>, between the specified <code>startDate</code> and <code>endDate</code>. For example, a metric of <code>five_minutes</code> is the median of all the data points gathered within those five minutes. <code>p50</code> is the only supported metric.</p>
    #[doc(hidden)]
    pub statistic: ::std::option::Option<crate::types::StatisticType>,
    /// <p>The aggregation period used for the data query.</p>
    #[doc(hidden)]
    pub period: ::std::option::Option<crate::types::PeriodType>,
}
impl DataQuery {
    /// <p>A user-defined ID associated with a data query that's returned in the <code>dataResponse</code> identifying the query. For example, if you set the Id to <code>MyQuery01</code>in the query, the <code>dataResponse</code> identifies the query as <code>MyQuery01</code>.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The Region or Availability Zone that's the source for the data query. For example, <code>us-east-1</code>.</p>
    pub fn source(&self) -> ::std::option::Option<&str> {
        self.source.as_deref()
    }
    /// <p>The Region or Availability Zone that's the target for the data query. For example, <code>eu-north-1</code>.</p>
    pub fn destination(&self) -> ::std::option::Option<&str> {
        self.destination.as_deref()
    }
    /// <p>The metric, <code>aggregation-latency</code>, indicating that network latency is aggregated for the query. This is the only supported metric.</p>
    pub fn metric(&self) -> ::std::option::Option<&crate::types::MetricType> {
        self.metric.as_ref()
    }
    /// <p>The metric data aggregation period, <code>p50</code>, between the specified <code>startDate</code> and <code>endDate</code>. For example, a metric of <code>five_minutes</code> is the median of all the data points gathered within those five minutes. <code>p50</code> is the only supported metric.</p>
    pub fn statistic(&self) -> ::std::option::Option<&crate::types::StatisticType> {
        self.statistic.as_ref()
    }
    /// <p>The aggregation period used for the data query.</p>
    pub fn period(&self) -> ::std::option::Option<&crate::types::PeriodType> {
        self.period.as_ref()
    }
}
impl DataQuery {
    /// Creates a new builder-style object to manufacture [`DataQuery`](crate::types::DataQuery).
    pub fn builder() -> crate::types::builders::DataQueryBuilder {
        crate::types::builders::DataQueryBuilder::default()
    }
}

/// A builder for [`DataQuery`](crate::types::DataQuery).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DataQueryBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) source: ::std::option::Option<::std::string::String>,
    pub(crate) destination: ::std::option::Option<::std::string::String>,
    pub(crate) metric: ::std::option::Option<crate::types::MetricType>,
    pub(crate) statistic: ::std::option::Option<crate::types::StatisticType>,
    pub(crate) period: ::std::option::Option<crate::types::PeriodType>,
}
impl DataQueryBuilder {
    /// <p>A user-defined ID associated with a data query that's returned in the <code>dataResponse</code> identifying the query. For example, if you set the Id to <code>MyQuery01</code>in the query, the <code>dataResponse</code> identifies the query as <code>MyQuery01</code>.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A user-defined ID associated with a data query that's returned in the <code>dataResponse</code> identifying the query. For example, if you set the Id to <code>MyQuery01</code>in the query, the <code>dataResponse</code> identifies the query as <code>MyQuery01</code>.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The Region or Availability Zone that's the source for the data query. For example, <code>us-east-1</code>.</p>
    pub fn source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Region or Availability Zone that's the source for the data query. For example, <code>us-east-1</code>.</p>
    pub fn set_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source = input;
        self
    }
    /// <p>The Region or Availability Zone that's the target for the data query. For example, <code>eu-north-1</code>.</p>
    pub fn destination(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Region or Availability Zone that's the target for the data query. For example, <code>eu-north-1</code>.</p>
    pub fn set_destination(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination = input;
        self
    }
    /// <p>The metric, <code>aggregation-latency</code>, indicating that network latency is aggregated for the query. This is the only supported metric.</p>
    pub fn metric(mut self, input: crate::types::MetricType) -> Self {
        self.metric = ::std::option::Option::Some(input);
        self
    }
    /// <p>The metric, <code>aggregation-latency</code>, indicating that network latency is aggregated for the query. This is the only supported metric.</p>
    pub fn set_metric(mut self, input: ::std::option::Option<crate::types::MetricType>) -> Self {
        self.metric = input;
        self
    }
    /// <p>The metric data aggregation period, <code>p50</code>, between the specified <code>startDate</code> and <code>endDate</code>. For example, a metric of <code>five_minutes</code> is the median of all the data points gathered within those five minutes. <code>p50</code> is the only supported metric.</p>
    pub fn statistic(mut self, input: crate::types::StatisticType) -> Self {
        self.statistic = ::std::option::Option::Some(input);
        self
    }
    /// <p>The metric data aggregation period, <code>p50</code>, between the specified <code>startDate</code> and <code>endDate</code>. For example, a metric of <code>five_minutes</code> is the median of all the data points gathered within those five minutes. <code>p50</code> is the only supported metric.</p>
    pub fn set_statistic(
        mut self,
        input: ::std::option::Option<crate::types::StatisticType>,
    ) -> Self {
        self.statistic = input;
        self
    }
    /// <p>The aggregation period used for the data query.</p>
    pub fn period(mut self, input: crate::types::PeriodType) -> Self {
        self.period = ::std::option::Option::Some(input);
        self
    }
    /// <p>The aggregation period used for the data query.</p>
    pub fn set_period(mut self, input: ::std::option::Option<crate::types::PeriodType>) -> Self {
        self.period = input;
        self
    }
    /// Consumes the builder and constructs a [`DataQuery`](crate::types::DataQuery).
    pub fn build(self) -> crate::types::DataQuery {
        crate::types::DataQuery {
            id: self.id,
            source: self.source,
            destination: self.destination,
            metric: self.metric,
            statistic: self.statistic,
            period: self.period,
        }
    }
}
