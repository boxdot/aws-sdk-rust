// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetVPCEConfiguration`](crate::operation::get_vpce_configuration::builders::GetVPCEConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl ::std::convert::Into<String>)`](crate::operation::get_vpce_configuration::builders::GetVPCEConfigurationFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::get_vpce_configuration::builders::GetVPCEConfigurationFluentBuilder::set_arn): <p>The Amazon Resource Name (ARN) of the VPC endpoint configuration you want to describe.</p>
    /// - On success, responds with [`GetVpceConfigurationOutput`](crate::operation::get_vpce_configuration::GetVpceConfigurationOutput) with field(s):
    ///   - [`vpce_configuration(Option<VpceConfiguration>)`](crate::operation::get_vpce_configuration::GetVpceConfigurationOutput::vpce_configuration): <p>An object that contains information about your VPC endpoint configuration.</p>
    /// - On failure, responds with [`SdkError<GetVPCEConfigurationError>`](crate::operation::get_vpce_configuration::GetVPCEConfigurationError)
    pub fn get_vpce_configuration(
        &self,
    ) -> crate::operation::get_vpce_configuration::builders::GetVPCEConfigurationFluentBuilder {
        crate::operation::get_vpce_configuration::builders::GetVPCEConfigurationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
