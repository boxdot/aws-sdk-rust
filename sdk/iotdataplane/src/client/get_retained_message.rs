// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetRetainedMessage`](crate::operation::get_retained_message::builders::GetRetainedMessageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`topic(impl ::std::convert::Into<String>)`](crate::operation::get_retained_message::builders::GetRetainedMessageFluentBuilder::topic) / [`set_topic(Option<String>)`](crate::operation::get_retained_message::builders::GetRetainedMessageFluentBuilder::set_topic): <p>The topic name of the retained message to retrieve.</p>
    /// - On success, responds with [`GetRetainedMessageOutput`](crate::operation::get_retained_message::GetRetainedMessageOutput) with field(s):
    ///   - [`topic(Option<String>)`](crate::operation::get_retained_message::GetRetainedMessageOutput::topic): <p>The topic name to which the retained message was published.</p>
    ///   - [`payload(Option<Blob>)`](crate::operation::get_retained_message::GetRetainedMessageOutput::payload): <p>The Base64-encoded message payload of the retained message body.</p>
    ///   - [`qos(i32)`](crate::operation::get_retained_message::GetRetainedMessageOutput::qos): <p>The quality of service (QoS) level used to publish the retained message.</p>
    ///   - [`last_modified_time(i64)`](crate::operation::get_retained_message::GetRetainedMessageOutput::last_modified_time): <p>The Epoch date and time, in milliseconds, when the retained message was stored by IoT.</p>
    ///   - [`user_properties(Option<Blob>)`](crate::operation::get_retained_message::GetRetainedMessageOutput::user_properties): <p>A base64-encoded JSON string that includes an array of JSON objects, or null if the retained message doesn't include any user properties.</p>  <p>The following example <code>userProperties</code> parameter is a JSON string that represents two user properties. Note that it will be base64-encoded:</p>  <p> <code>[{"deviceName": "alpha"}, {"deviceCnt": "45"}]</code> </p>
    /// - On failure, responds with [`SdkError<GetRetainedMessageError>`](crate::operation::get_retained_message::GetRetainedMessageError)
    pub fn get_retained_message(
        &self,
    ) -> crate::operation::get_retained_message::builders::GetRetainedMessageFluentBuilder {
        crate::operation::get_retained_message::builders::GetRetainedMessageFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
