// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetVoiceConnectorTermination`](crate::operation::get_voice_connector_termination::builders::GetVoiceConnectorTerminationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl ::std::convert::Into<String>)`](crate::operation::get_voice_connector_termination::builders::GetVoiceConnectorTerminationFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::get_voice_connector_termination::builders::GetVoiceConnectorTerminationFluentBuilder::set_voice_connector_id): <p>The Amazon Chime Voice Connector ID.</p>
    /// - On success, responds with [`GetVoiceConnectorTerminationOutput`](crate::operation::get_voice_connector_termination::GetVoiceConnectorTerminationOutput) with field(s):
    ///   - [`termination(Option<Termination>)`](crate::operation::get_voice_connector_termination::GetVoiceConnectorTerminationOutput::termination): <p>The termination setting details.</p>
    /// - On failure, responds with [`SdkError<GetVoiceConnectorTerminationError>`](crate::operation::get_voice_connector_termination::GetVoiceConnectorTerminationError)
    pub fn get_voice_connector_termination(&self) -> crate::operation::get_voice_connector_termination::builders::GetVoiceConnectorTerminationFluentBuilder{
        crate::operation::get_voice_connector_termination::builders::GetVoiceConnectorTerminationFluentBuilder::new(self.handle.clone())
    }
}
