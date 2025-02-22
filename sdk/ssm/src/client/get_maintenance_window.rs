// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetMaintenanceWindow`](crate::operation::get_maintenance_window::builders::GetMaintenanceWindowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`window_id(impl ::std::convert::Into<String>)`](crate::operation::get_maintenance_window::builders::GetMaintenanceWindowFluentBuilder::window_id) / [`set_window_id(Option<String>)`](crate::operation::get_maintenance_window::builders::GetMaintenanceWindowFluentBuilder::set_window_id): <p>The ID of the maintenance window for which you want to retrieve information.</p>
    /// - On success, responds with [`GetMaintenanceWindowOutput`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput) with field(s):
    ///   - [`window_id(Option<String>)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::window_id): <p>The ID of the created maintenance window.</p>
    ///   - [`name(Option<String>)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::name): <p>The name of the maintenance window.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::description): <p>The description of the maintenance window.</p>
    ///   - [`start_date(Option<String>)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::start_date): <p>The date and time, in ISO-8601 Extended format, for when the maintenance window is scheduled to become active. The maintenance window won't run before this specified time.</p>
    ///   - [`end_date(Option<String>)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::end_date): <p>The date and time, in ISO-8601 Extended format, for when the maintenance window is scheduled to become inactive. The maintenance window won't run after this specified time.</p>
    ///   - [`schedule(Option<String>)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::schedule): <p>The schedule of the maintenance window in the form of a cron or rate expression.</p>
    ///   - [`schedule_timezone(Option<String>)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::schedule_timezone): <p>The time zone that the scheduled maintenance window executions are based on, in Internet Assigned Numbers Authority (IANA) format. For example: "America/Los_Angeles", "UTC", or "Asia/Seoul". For more information, see the <a href="https://www.iana.org/time-zones">Time Zone Database</a> on the IANA website.</p>
    ///   - [`schedule_offset(Option<i32>)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::schedule_offset): <p>The number of days to wait to run a maintenance window after the scheduled cron expression date and time.</p>
    ///   - [`next_execution_time(Option<String>)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::next_execution_time): <p>The next time the maintenance window will actually run, taking into account any specified times for the maintenance window to become active or inactive.</p>
    ///   - [`duration(i32)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::duration): <p>The duration of the maintenance window in hours.</p>
    ///   - [`cutoff(i32)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::cutoff): <p>The number of hours before the end of the maintenance window that Amazon Web Services Systems Manager stops scheduling new tasks for execution.</p>
    ///   - [`allow_unassociated_targets(bool)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::allow_unassociated_targets): <p>Whether targets must be registered with the maintenance window before tasks can be defined for those targets.</p>
    ///   - [`enabled(bool)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::enabled): <p>Indicates whether the maintenance window is enabled.</p>
    ///   - [`created_date(Option<DateTime>)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::created_date): <p>The date the maintenance window was created.</p>
    ///   - [`modified_date(Option<DateTime>)`](crate::operation::get_maintenance_window::GetMaintenanceWindowOutput::modified_date): <p>The date the maintenance window was last modified.</p>
    /// - On failure, responds with [`SdkError<GetMaintenanceWindowError>`](crate::operation::get_maintenance_window::GetMaintenanceWindowError)
    pub fn get_maintenance_window(
        &self,
    ) -> crate::operation::get_maintenance_window::builders::GetMaintenanceWindowFluentBuilder {
        crate::operation::get_maintenance_window::builders::GetMaintenanceWindowFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
