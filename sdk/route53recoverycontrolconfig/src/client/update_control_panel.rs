// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateControlPanel`](crate::operation::update_control_panel::builders::UpdateControlPanelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`control_panel_arn(impl ::std::convert::Into<String>)`](crate::operation::update_control_panel::builders::UpdateControlPanelFluentBuilder::control_panel_arn) / [`set_control_panel_arn(Option<String>)`](crate::operation::update_control_panel::builders::UpdateControlPanelFluentBuilder::set_control_panel_arn): <p>The Amazon Resource Name (ARN) of the control panel.</p>
    ///   - [`control_panel_name(impl ::std::convert::Into<String>)`](crate::operation::update_control_panel::builders::UpdateControlPanelFluentBuilder::control_panel_name) / [`set_control_panel_name(Option<String>)`](crate::operation::update_control_panel::builders::UpdateControlPanelFluentBuilder::set_control_panel_name): <p>The name of the control panel.</p>
    /// - On success, responds with [`UpdateControlPanelOutput`](crate::operation::update_control_panel::UpdateControlPanelOutput) with field(s):
    ///   - [`control_panel(Option<ControlPanel>)`](crate::operation::update_control_panel::UpdateControlPanelOutput::control_panel): <p>The control panel to update.</p>
    /// - On failure, responds with [`SdkError<UpdateControlPanelError>`](crate::operation::update_control_panel::UpdateControlPanelError)
    pub fn update_control_panel(
        &self,
    ) -> crate::operation::update_control_panel::builders::UpdateControlPanelFluentBuilder {
        crate::operation::update_control_panel::builders::UpdateControlPanelFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
