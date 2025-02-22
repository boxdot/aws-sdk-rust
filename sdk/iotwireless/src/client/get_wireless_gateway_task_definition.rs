// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetWirelessGatewayTaskDefinition`](crate::operation::get_wireless_gateway_task_definition::builders::GetWirelessGatewayTaskDefinitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::get_wireless_gateway_task_definition::builders::GetWirelessGatewayTaskDefinitionFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_wireless_gateway_task_definition::builders::GetWirelessGatewayTaskDefinitionFluentBuilder::set_id): <p>The ID of the resource to get.</p>
    /// - On success, responds with [`GetWirelessGatewayTaskDefinitionOutput`](crate::operation::get_wireless_gateway_task_definition::GetWirelessGatewayTaskDefinitionOutput) with field(s):
    ///   - [`auto_create_tasks(bool)`](crate::operation::get_wireless_gateway_task_definition::GetWirelessGatewayTaskDefinitionOutput::auto_create_tasks): <p>Whether to automatically create tasks using this task definition for all gateways with the specified current version. If <code>false</code>, the task must me created by calling <code>CreateWirelessGatewayTask</code>.</p>
    ///   - [`name(Option<String>)`](crate::operation::get_wireless_gateway_task_definition::GetWirelessGatewayTaskDefinitionOutput::name): <p>The name of the resource.</p>
    ///   - [`update(Option<UpdateWirelessGatewayTaskCreate>)`](crate::operation::get_wireless_gateway_task_definition::GetWirelessGatewayTaskDefinitionOutput::update): <p>Information about the gateways to update.</p>
    ///   - [`arn(Option<String>)`](crate::operation::get_wireless_gateway_task_definition::GetWirelessGatewayTaskDefinitionOutput::arn): <p>The Amazon Resource Name of the resource.</p>
    /// - On failure, responds with [`SdkError<GetWirelessGatewayTaskDefinitionError>`](crate::operation::get_wireless_gateway_task_definition::GetWirelessGatewayTaskDefinitionError)
    pub fn get_wireless_gateway_task_definition(&self) -> crate::operation::get_wireless_gateway_task_definition::builders::GetWirelessGatewayTaskDefinitionFluentBuilder{
        crate::operation::get_wireless_gateway_task_definition::builders::GetWirelessGatewayTaskDefinitionFluentBuilder::new(self.handle.clone())
    }
}
