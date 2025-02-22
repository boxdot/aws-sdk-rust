// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateDestination`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::set_id): Destination ARN.
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::set_name): Human friendly name of the resource.
    ///   - [`state(DestinationState)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::state) / [`set_state(Option<DestinationState>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::set_state): State of the destination.
    ///   - [`additional_fixed_properties(impl ::std::convert::Into<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::additional_fixed_properties) / [`set_additional_fixed_properties(Option<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::set_additional_fixed_properties): JSON document containing additional fixed properties regarding the destination
    /// - On success, responds with [`UpdateDestinationOutput`](crate::operation::update_destination::UpdateDestinationOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::update_destination::UpdateDestinationOutput::arn): Destination ARN.
    ///   - [`id(Option<String>)`](crate::operation::update_destination::UpdateDestinationOutput::id): Filters access by the destination's identifier
    ///   - [`name(Option<String>)`](crate::operation::update_destination::UpdateDestinationOutput::name): Human friendly name of the resource.
    ///   - [`updated_at(Option<DateTime>)`](crate::operation::update_destination::UpdateDestinationOutput::updated_at): Timestamp at which the resource was last updated.
    ///   - [`state(Option<DestinationState>)`](crate::operation::update_destination::UpdateDestinationOutput::state): State of the destination.
    ///   - [`additional_fixed_properties(Option<String>)`](crate::operation::update_destination::UpdateDestinationOutput::additional_fixed_properties): JSON document containing additional fixed properties regarding the destination
    /// - On failure, responds with [`SdkError<UpdateDestinationError>`](crate::operation::update_destination::UpdateDestinationError)
    pub fn update_destination(
        &self,
    ) -> crate::operation::update_destination::builders::UpdateDestinationFluentBuilder {
        crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
