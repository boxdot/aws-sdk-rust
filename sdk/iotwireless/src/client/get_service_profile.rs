// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetServiceProfile`](crate::operation::get_service_profile::builders::GetServiceProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::get_service_profile::builders::GetServiceProfileFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_service_profile::builders::GetServiceProfileFluentBuilder::set_id): <p>The ID of the resource to get.</p>
    /// - On success, responds with [`GetServiceProfileOutput`](crate::operation::get_service_profile::GetServiceProfileOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::get_service_profile::GetServiceProfileOutput::arn): <p>The Amazon Resource Name of the resource.</p>
    ///   - [`name(Option<String>)`](crate::operation::get_service_profile::GetServiceProfileOutput::name): <p>The name of the resource.</p>
    ///   - [`id(Option<String>)`](crate::operation::get_service_profile::GetServiceProfileOutput::id): <p>The ID of the service profile.</p>
    ///   - [`lo_ra_wan(Option<LoRaWanGetServiceProfileInfo>)`](crate::operation::get_service_profile::GetServiceProfileOutput::lo_ra_wan): <p>Information about the service profile.</p>
    /// - On failure, responds with [`SdkError<GetServiceProfileError>`](crate::operation::get_service_profile::GetServiceProfileError)
    pub fn get_service_profile(
        &self,
    ) -> crate::operation::get_service_profile::builders::GetServiceProfileFluentBuilder {
        crate::operation::get_service_profile::builders::GetServiceProfileFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
