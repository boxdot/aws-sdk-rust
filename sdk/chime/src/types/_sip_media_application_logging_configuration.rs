// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Logging configuration of the SIP media application.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SipMediaApplicationLoggingConfiguration {
    /// <p>Enables application message logs for the SIP media application.</p>
    #[doc(hidden)]
    pub enable_sip_media_application_message_logs: ::std::option::Option<bool>,
}
impl SipMediaApplicationLoggingConfiguration {
    /// <p>Enables application message logs for the SIP media application.</p>
    pub fn enable_sip_media_application_message_logs(&self) -> ::std::option::Option<bool> {
        self.enable_sip_media_application_message_logs
    }
}
impl SipMediaApplicationLoggingConfiguration {
    /// Creates a new builder-style object to manufacture [`SipMediaApplicationLoggingConfiguration`](crate::types::SipMediaApplicationLoggingConfiguration).
    pub fn builder() -> crate::types::builders::SipMediaApplicationLoggingConfigurationBuilder {
        crate::types::builders::SipMediaApplicationLoggingConfigurationBuilder::default()
    }
}

/// A builder for [`SipMediaApplicationLoggingConfiguration`](crate::types::SipMediaApplicationLoggingConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SipMediaApplicationLoggingConfigurationBuilder {
    pub(crate) enable_sip_media_application_message_logs: ::std::option::Option<bool>,
}
impl SipMediaApplicationLoggingConfigurationBuilder {
    /// <p>Enables application message logs for the SIP media application.</p>
    pub fn enable_sip_media_application_message_logs(mut self, input: bool) -> Self {
        self.enable_sip_media_application_message_logs = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enables application message logs for the SIP media application.</p>
    pub fn set_enable_sip_media_application_message_logs(
        mut self,
        input: ::std::option::Option<bool>,
    ) -> Self {
        self.enable_sip_media_application_message_logs = input;
        self
    }
    /// Consumes the builder and constructs a [`SipMediaApplicationLoggingConfiguration`](crate::types::SipMediaApplicationLoggingConfiguration).
    pub fn build(self) -> crate::types::SipMediaApplicationLoggingConfiguration {
        crate::types::SipMediaApplicationLoggingConfiguration {
            enable_sip_media_application_message_logs: self
                .enable_sip_media_application_message_logs,
        }
    }
}
