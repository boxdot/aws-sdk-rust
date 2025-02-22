// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateMonitoringSchedule`](crate::operation::update_monitoring_schedule::builders::UpdateMonitoringScheduleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`monitoring_schedule_name(impl ::std::convert::Into<String>)`](crate::operation::update_monitoring_schedule::builders::UpdateMonitoringScheduleFluentBuilder::monitoring_schedule_name) / [`set_monitoring_schedule_name(Option<String>)`](crate::operation::update_monitoring_schedule::builders::UpdateMonitoringScheduleFluentBuilder::set_monitoring_schedule_name): <p>The name of the monitoring schedule. The name must be unique within an Amazon Web Services Region within an Amazon Web Services account.</p>
    ///   - [`monitoring_schedule_config(MonitoringScheduleConfig)`](crate::operation::update_monitoring_schedule::builders::UpdateMonitoringScheduleFluentBuilder::monitoring_schedule_config) / [`set_monitoring_schedule_config(Option<MonitoringScheduleConfig>)`](crate::operation::update_monitoring_schedule::builders::UpdateMonitoringScheduleFluentBuilder::set_monitoring_schedule_config): <p>The configuration object that specifies the monitoring schedule and defines the monitoring job.</p>
    /// - On success, responds with [`UpdateMonitoringScheduleOutput`](crate::operation::update_monitoring_schedule::UpdateMonitoringScheduleOutput) with field(s):
    ///   - [`monitoring_schedule_arn(Option<String>)`](crate::operation::update_monitoring_schedule::UpdateMonitoringScheduleOutput::monitoring_schedule_arn): <p>The Amazon Resource Name (ARN) of the monitoring schedule.</p>
    /// - On failure, responds with [`SdkError<UpdateMonitoringScheduleError>`](crate::operation::update_monitoring_schedule::UpdateMonitoringScheduleError)
    pub fn update_monitoring_schedule(
        &self,
    ) -> crate::operation::update_monitoring_schedule::builders::UpdateMonitoringScheduleFluentBuilder
    {
        crate::operation::update_monitoring_schedule::builders::UpdateMonitoringScheduleFluentBuilder::new(self.handle.clone())
    }
}
