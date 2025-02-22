// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetLaunchTemplateData`](crate::operation::get_launch_template_data::builders::GetLaunchTemplateDataFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::get_launch_template_data::builders::GetLaunchTemplateDataFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_launch_template_data::builders::GetLaunchTemplateDataFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::get_launch_template_data::builders::GetLaunchTemplateDataFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::get_launch_template_data::builders::GetLaunchTemplateDataFluentBuilder::set_instance_id): <p>The ID of the instance.</p>
    /// - On success, responds with [`GetLaunchTemplateDataOutput`](crate::operation::get_launch_template_data::GetLaunchTemplateDataOutput) with field(s):
    ///   - [`launch_template_data(Option<ResponseLaunchTemplateData>)`](crate::operation::get_launch_template_data::GetLaunchTemplateDataOutput::launch_template_data): <p>The instance data.</p>
    /// - On failure, responds with [`SdkError<GetLaunchTemplateDataError>`](crate::operation::get_launch_template_data::GetLaunchTemplateDataError)
    pub fn get_launch_template_data(
        &self,
    ) -> crate::operation::get_launch_template_data::builders::GetLaunchTemplateDataFluentBuilder
    {
        crate::operation::get_launch_template_data::builders::GetLaunchTemplateDataFluentBuilder::new(self.handle.clone())
    }
}
