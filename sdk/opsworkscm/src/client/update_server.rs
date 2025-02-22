// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateServer`](crate::operation::update_server::builders::UpdateServerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`disable_automated_backup(bool)`](crate::operation::update_server::builders::UpdateServerFluentBuilder::disable_automated_backup) / [`set_disable_automated_backup(Option<bool>)`](crate::operation::update_server::builders::UpdateServerFluentBuilder::set_disable_automated_backup): <p>Setting DisableAutomatedBackup to <code>true</code> disables automated or scheduled backups. Automated backups are enabled by default. </p>
    ///   - [`backup_retention_count(i32)`](crate::operation::update_server::builders::UpdateServerFluentBuilder::backup_retention_count) / [`set_backup_retention_count(Option<i32>)`](crate::operation::update_server::builders::UpdateServerFluentBuilder::set_backup_retention_count): <p>Sets the number of automated backups that you want to keep. </p>
    ///   - [`server_name(impl ::std::convert::Into<String>)`](crate::operation::update_server::builders::UpdateServerFluentBuilder::server_name) / [`set_server_name(Option<String>)`](crate::operation::update_server::builders::UpdateServerFluentBuilder::set_server_name): <p>The name of the server to update. </p>
    ///   - [`preferred_maintenance_window(impl ::std::convert::Into<String>)`](crate::operation::update_server::builders::UpdateServerFluentBuilder::preferred_maintenance_window) / [`set_preferred_maintenance_window(Option<String>)`](crate::operation::update_server::builders::UpdateServerFluentBuilder::set_preferred_maintenance_window): <p> <code>DDD:HH:MM</code> (weekly start time) or <code>HH:MM</code> (daily start time). </p>  <p> Time windows always use coordinated universal time (UTC). Valid strings for day of week (<code>DDD</code>) are: <code>Mon</code>, <code>Tue</code>, <code>Wed</code>, <code>Thr</code>, <code>Fri</code>, <code>Sat</code>, or <code>Sun</code>.</p>
    ///   - [`preferred_backup_window(impl ::std::convert::Into<String>)`](crate::operation::update_server::builders::UpdateServerFluentBuilder::preferred_backup_window) / [`set_preferred_backup_window(Option<String>)`](crate::operation::update_server::builders::UpdateServerFluentBuilder::set_preferred_backup_window): <p> <code>DDD:HH:MM</code> (weekly start time) or <code>HH:MM</code> (daily start time). </p>  <p> Time windows always use coordinated universal time (UTC). Valid strings for day of week (<code>DDD</code>) are: <code>Mon</code>, <code>Tue</code>, <code>Wed</code>, <code>Thr</code>, <code>Fri</code>, <code>Sat</code>, or <code>Sun</code>.</p>
    /// - On success, responds with [`UpdateServerOutput`](crate::operation::update_server::UpdateServerOutput) with field(s):
    ///   - [`server(Option<Server>)`](crate::operation::update_server::UpdateServerOutput::server): <p>Contains the response to a <code>UpdateServer</code> request. </p>
    /// - On failure, responds with [`SdkError<UpdateServerError>`](crate::operation::update_server::UpdateServerError)
    pub fn update_server(
        &self,
    ) -> crate::operation::update_server::builders::UpdateServerFluentBuilder {
        crate::operation::update_server::builders::UpdateServerFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
