// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetServiceSetting`](crate::operation::get_service_setting::builders::GetServiceSettingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`setting_id(impl ::std::convert::Into<String>)`](crate::operation::get_service_setting::builders::GetServiceSettingFluentBuilder::setting_id) / [`set_setting_id(Option<String>)`](crate::operation::get_service_setting::builders::GetServiceSettingFluentBuilder::set_setting_id): <p>The ID of the service setting to get. The setting ID can be one of the following.</p>  <ul>   <li> <p> <code>/ssm/managed-instance/default-ec2-instance-management-role</code> </p> </li>   <li> <p> <code>/ssm/automation/customer-script-log-destination</code> </p> </li>   <li> <p> <code>/ssm/automation/customer-script-log-group-name</code> </p> </li>   <li> <p> <code>/ssm/documents/console/public-sharing-permission</code> </p> </li>   <li> <p> <code>/ssm/managed-instance/activation-tier</code> </p> </li>   <li> <p> <code>/ssm/opsinsights/opscenter</code> </p> </li>   <li> <p> <code>/ssm/parameter-store/default-parameter-tier</code> </p> </li>   <li> <p> <code>/ssm/parameter-store/high-throughput-enabled</code> </p> </li>  </ul>
    /// - On success, responds with [`GetServiceSettingOutput`](crate::operation::get_service_setting::GetServiceSettingOutput) with field(s):
    ///   - [`service_setting(Option<ServiceSetting>)`](crate::operation::get_service_setting::GetServiceSettingOutput::service_setting): <p>The query result of the current service setting.</p>
    /// - On failure, responds with [`SdkError<GetServiceSettingError>`](crate::operation::get_service_setting::GetServiceSettingError)
    pub fn get_service_setting(
        &self,
    ) -> crate::operation::get_service_setting::builders::GetServiceSettingFluentBuilder {
        crate::operation::get_service_setting::builders::GetServiceSettingFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
