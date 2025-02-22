// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SendVoiceMessage`](crate::operation::send_voice_message::builders::SendVoiceMessageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`caller_id(impl ::std::convert::Into<String>)`](crate::operation::send_voice_message::builders::SendVoiceMessageFluentBuilder::caller_id) / [`set_caller_id(Option<String>)`](crate::operation::send_voice_message::builders::SendVoiceMessageFluentBuilder::set_caller_id): The phone number that appears on recipients' devices when they receive the message.
    ///   - [`configuration_set_name(impl ::std::convert::Into<String>)`](crate::operation::send_voice_message::builders::SendVoiceMessageFluentBuilder::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::operation::send_voice_message::builders::SendVoiceMessageFluentBuilder::set_configuration_set_name): The name of the configuration set that you want to use to send the message.
    ///   - [`content(VoiceMessageContent)`](crate::operation::send_voice_message::builders::SendVoiceMessageFluentBuilder::content) / [`set_content(Option<VoiceMessageContent>)`](crate::operation::send_voice_message::builders::SendVoiceMessageFluentBuilder::set_content): An object that contains a voice message and information about the recipient that you want to send it to.
    ///   - [`destination_phone_number(impl ::std::convert::Into<String>)`](crate::operation::send_voice_message::builders::SendVoiceMessageFluentBuilder::destination_phone_number) / [`set_destination_phone_number(Option<String>)`](crate::operation::send_voice_message::builders::SendVoiceMessageFluentBuilder::set_destination_phone_number): The phone number that you want to send the voice message to.
    ///   - [`origination_phone_number(impl ::std::convert::Into<String>)`](crate::operation::send_voice_message::builders::SendVoiceMessageFluentBuilder::origination_phone_number) / [`set_origination_phone_number(Option<String>)`](crate::operation::send_voice_message::builders::SendVoiceMessageFluentBuilder::set_origination_phone_number): The phone number that Amazon Pinpoint should use to send the voice message. This isn't necessarily the phone number that appears on recipients' devices when they receive the message, because you can specify a CallerId parameter in the request.
    /// - On success, responds with [`SendVoiceMessageOutput`](crate::operation::send_voice_message::SendVoiceMessageOutput) with field(s):
    ///   - [`message_id(Option<String>)`](crate::operation::send_voice_message::SendVoiceMessageOutput::message_id): A unique identifier for the voice message.
    /// - On failure, responds with [`SdkError<SendVoiceMessageError>`](crate::operation::send_voice_message::SendVoiceMessageError)
    pub fn send_voice_message(
        &self,
    ) -> crate::operation::send_voice_message::builders::SendVoiceMessageFluentBuilder {
        crate::operation::send_voice_message::builders::SendVoiceMessageFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
