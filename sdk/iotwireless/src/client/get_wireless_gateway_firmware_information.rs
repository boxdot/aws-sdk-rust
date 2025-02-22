// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetWirelessGatewayFirmwareInformation`](crate::operation::get_wireless_gateway_firmware_information::builders::GetWirelessGatewayFirmwareInformationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::get_wireless_gateway_firmware_information::builders::GetWirelessGatewayFirmwareInformationFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_wireless_gateway_firmware_information::builders::GetWirelessGatewayFirmwareInformationFluentBuilder::set_id): <p>The ID of the resource to get.</p>
    /// - On success, responds with [`GetWirelessGatewayFirmwareInformationOutput`](crate::operation::get_wireless_gateway_firmware_information::GetWirelessGatewayFirmwareInformationOutput) with field(s):
    ///   - [`lo_ra_wan(Option<LoRaWanGatewayCurrentVersion>)`](crate::operation::get_wireless_gateway_firmware_information::GetWirelessGatewayFirmwareInformationOutput::lo_ra_wan): <p>Information about the wireless gateway's firmware.</p>
    /// - On failure, responds with [`SdkError<GetWirelessGatewayFirmwareInformationError>`](crate::operation::get_wireless_gateway_firmware_information::GetWirelessGatewayFirmwareInformationError)
    pub fn get_wireless_gateway_firmware_information(&self) -> crate::operation::get_wireless_gateway_firmware_information::builders::GetWirelessGatewayFirmwareInformationFluentBuilder{
        crate::operation::get_wireless_gateway_firmware_information::builders::GetWirelessGatewayFirmwareInformationFluentBuilder::new(self.handle.clone())
    }
}
