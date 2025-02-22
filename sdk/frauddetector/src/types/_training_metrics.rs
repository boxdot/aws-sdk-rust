// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The training metric details.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TrainingMetrics {
    /// <p>The area under the curve. This summarizes true positive rate (TPR) and false positive rate (FPR) across all possible model score thresholds. A model with no predictive power has an AUC of 0.5, whereas a perfect model has a score of 1.0.</p>
    #[doc(hidden)]
    pub auc: ::std::option::Option<f32>,
    /// <p>The data points details.</p>
    #[doc(hidden)]
    pub metric_data_points: ::std::option::Option<::std::vec::Vec<crate::types::MetricDataPoint>>,
}
impl TrainingMetrics {
    /// <p>The area under the curve. This summarizes true positive rate (TPR) and false positive rate (FPR) across all possible model score thresholds. A model with no predictive power has an AUC of 0.5, whereas a perfect model has a score of 1.0.</p>
    pub fn auc(&self) -> ::std::option::Option<f32> {
        self.auc
    }
    /// <p>The data points details.</p>
    pub fn metric_data_points(&self) -> ::std::option::Option<&[crate::types::MetricDataPoint]> {
        self.metric_data_points.as_deref()
    }
}
impl TrainingMetrics {
    /// Creates a new builder-style object to manufacture [`TrainingMetrics`](crate::types::TrainingMetrics).
    pub fn builder() -> crate::types::builders::TrainingMetricsBuilder {
        crate::types::builders::TrainingMetricsBuilder::default()
    }
}

/// A builder for [`TrainingMetrics`](crate::types::TrainingMetrics).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TrainingMetricsBuilder {
    pub(crate) auc: ::std::option::Option<f32>,
    pub(crate) metric_data_points:
        ::std::option::Option<::std::vec::Vec<crate::types::MetricDataPoint>>,
}
impl TrainingMetricsBuilder {
    /// <p>The area under the curve. This summarizes true positive rate (TPR) and false positive rate (FPR) across all possible model score thresholds. A model with no predictive power has an AUC of 0.5, whereas a perfect model has a score of 1.0.</p>
    pub fn auc(mut self, input: f32) -> Self {
        self.auc = ::std::option::Option::Some(input);
        self
    }
    /// <p>The area under the curve. This summarizes true positive rate (TPR) and false positive rate (FPR) across all possible model score thresholds. A model with no predictive power has an AUC of 0.5, whereas a perfect model has a score of 1.0.</p>
    pub fn set_auc(mut self, input: ::std::option::Option<f32>) -> Self {
        self.auc = input;
        self
    }
    /// Appends an item to `metric_data_points`.
    ///
    /// To override the contents of this collection use [`set_metric_data_points`](Self::set_metric_data_points).
    ///
    /// <p>The data points details.</p>
    pub fn metric_data_points(mut self, input: crate::types::MetricDataPoint) -> Self {
        let mut v = self.metric_data_points.unwrap_or_default();
        v.push(input);
        self.metric_data_points = ::std::option::Option::Some(v);
        self
    }
    /// <p>The data points details.</p>
    pub fn set_metric_data_points(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MetricDataPoint>>,
    ) -> Self {
        self.metric_data_points = input;
        self
    }
    /// Consumes the builder and constructs a [`TrainingMetrics`](crate::types::TrainingMetrics).
    pub fn build(self) -> crate::types::TrainingMetrics {
        crate::types::TrainingMetrics {
            auc: self.auc,
            metric_data_points: self.metric_data_points,
        }
    }
}
