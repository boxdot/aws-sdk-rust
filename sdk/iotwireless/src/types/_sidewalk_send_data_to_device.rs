// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a Sidewalk router.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SidewalkSendDataToDevice {
    /// <p>The sequence number.</p>
    #[doc(hidden)]
    pub seq: ::std::option::Option<i32>,
    /// <p>Sidewalk device message type. Default value is <code>CUSTOM_COMMAND_ID_NOTIFY</code>.</p>
    #[doc(hidden)]
    pub message_type: ::std::option::Option<crate::types::MessageType>,
    /// <p>The duration of time in seconds to retry sending the ACK.</p>
    #[doc(hidden)]
    pub ack_mode_retry_duration_secs: ::std::option::Option<i32>,
}
impl SidewalkSendDataToDevice {
    /// <p>The sequence number.</p>
    pub fn seq(&self) -> ::std::option::Option<i32> {
        self.seq
    }
    /// <p>Sidewalk device message type. Default value is <code>CUSTOM_COMMAND_ID_NOTIFY</code>.</p>
    pub fn message_type(&self) -> ::std::option::Option<&crate::types::MessageType> {
        self.message_type.as_ref()
    }
    /// <p>The duration of time in seconds to retry sending the ACK.</p>
    pub fn ack_mode_retry_duration_secs(&self) -> ::std::option::Option<i32> {
        self.ack_mode_retry_duration_secs
    }
}
impl SidewalkSendDataToDevice {
    /// Creates a new builder-style object to manufacture [`SidewalkSendDataToDevice`](crate::types::SidewalkSendDataToDevice).
    pub fn builder() -> crate::types::builders::SidewalkSendDataToDeviceBuilder {
        crate::types::builders::SidewalkSendDataToDeviceBuilder::default()
    }
}

/// A builder for [`SidewalkSendDataToDevice`](crate::types::SidewalkSendDataToDevice).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SidewalkSendDataToDeviceBuilder {
    pub(crate) seq: ::std::option::Option<i32>,
    pub(crate) message_type: ::std::option::Option<crate::types::MessageType>,
    pub(crate) ack_mode_retry_duration_secs: ::std::option::Option<i32>,
}
impl SidewalkSendDataToDeviceBuilder {
    /// <p>The sequence number.</p>
    pub fn seq(mut self, input: i32) -> Self {
        self.seq = ::std::option::Option::Some(input);
        self
    }
    /// <p>The sequence number.</p>
    pub fn set_seq(mut self, input: ::std::option::Option<i32>) -> Self {
        self.seq = input;
        self
    }
    /// <p>Sidewalk device message type. Default value is <code>CUSTOM_COMMAND_ID_NOTIFY</code>.</p>
    pub fn message_type(mut self, input: crate::types::MessageType) -> Self {
        self.message_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Sidewalk device message type. Default value is <code>CUSTOM_COMMAND_ID_NOTIFY</code>.</p>
    pub fn set_message_type(
        mut self,
        input: ::std::option::Option<crate::types::MessageType>,
    ) -> Self {
        self.message_type = input;
        self
    }
    /// <p>The duration of time in seconds to retry sending the ACK.</p>
    pub fn ack_mode_retry_duration_secs(mut self, input: i32) -> Self {
        self.ack_mode_retry_duration_secs = ::std::option::Option::Some(input);
        self
    }
    /// <p>The duration of time in seconds to retry sending the ACK.</p>
    pub fn set_ack_mode_retry_duration_secs(mut self, input: ::std::option::Option<i32>) -> Self {
        self.ack_mode_retry_duration_secs = input;
        self
    }
    /// Consumes the builder and constructs a [`SidewalkSendDataToDevice`](crate::types::SidewalkSendDataToDevice).
    pub fn build(self) -> crate::types::SidewalkSendDataToDevice {
        crate::types::SidewalkSendDataToDevice {
            seq: self.seq,
            message_type: self.message_type,
            ack_mode_retry_duration_secs: self.ack_mode_retry_duration_secs,
        }
    }
}
