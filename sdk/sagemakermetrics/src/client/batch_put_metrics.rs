// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchPutMetrics`](crate::operation::batch_put_metrics::builders::BatchPutMetricsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`trial_component_name(impl ::std::convert::Into<String>)`](crate::operation::batch_put_metrics::builders::BatchPutMetricsFluentBuilder::trial_component_name) / [`set_trial_component_name(Option<String>)`](crate::operation::batch_put_metrics::builders::BatchPutMetricsFluentBuilder::set_trial_component_name): <p>The name of the Trial Component to associate with the metrics.</p>
    ///   - [`metric_data(Vec<RawMetricData>)`](crate::operation::batch_put_metrics::builders::BatchPutMetricsFluentBuilder::metric_data) / [`set_metric_data(Option<Vec<RawMetricData>>)`](crate::operation::batch_put_metrics::builders::BatchPutMetricsFluentBuilder::set_metric_data): <p>A list of raw metric values to put.</p>
    /// - On success, responds with [`BatchPutMetricsOutput`](crate::operation::batch_put_metrics::BatchPutMetricsOutput) with field(s):
    ///   - [`errors(Option<Vec<BatchPutMetricsError>>)`](crate::operation::batch_put_metrics::BatchPutMetricsOutput::errors): <p>Lists any errors that occur when inserting metric data.</p>
    /// - On failure, responds with [`SdkError<BatchPutMetricsError>`](crate::operation::batch_put_metrics::BatchPutMetricsError)
    pub fn batch_put_metrics(
        &self,
    ) -> crate::operation::batch_put_metrics::builders::BatchPutMetricsFluentBuilder {
        crate::operation::batch_put_metrics::builders::BatchPutMetricsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
