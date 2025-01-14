// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutVoiceConnectorLoggingConfigurationInput {
    /// <p>The Voice Connector ID.</p>
    #[doc(hidden)]
    pub voice_connector_id: ::std::option::Option<::std::string::String>,
    /// <p>The logging configuration being updated.</p>
    #[doc(hidden)]
    pub logging_configuration: ::std::option::Option<crate::types::LoggingConfiguration>,
}
impl PutVoiceConnectorLoggingConfigurationInput {
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(&self) -> ::std::option::Option<&str> {
        self.voice_connector_id.as_deref()
    }
    /// <p>The logging configuration being updated.</p>
    pub fn logging_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::LoggingConfiguration> {
        self.logging_configuration.as_ref()
    }
}
impl PutVoiceConnectorLoggingConfigurationInput {
    /// Creates a new builder-style object to manufacture [`PutVoiceConnectorLoggingConfigurationInput`](crate::operation::put_voice_connector_logging_configuration::PutVoiceConnectorLoggingConfigurationInput).
    pub fn builder() -> crate::operation::put_voice_connector_logging_configuration::builders::PutVoiceConnectorLoggingConfigurationInputBuilder{
        crate::operation::put_voice_connector_logging_configuration::builders::PutVoiceConnectorLoggingConfigurationInputBuilder::default()
    }
}

/// A builder for [`PutVoiceConnectorLoggingConfigurationInput`](crate::operation::put_voice_connector_logging_configuration::PutVoiceConnectorLoggingConfigurationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutVoiceConnectorLoggingConfigurationInputBuilder {
    pub(crate) voice_connector_id: ::std::option::Option<::std::string::String>,
    pub(crate) logging_configuration: ::std::option::Option<crate::types::LoggingConfiguration>,
}
impl PutVoiceConnectorLoggingConfigurationInputBuilder {
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.voice_connector_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Voice Connector ID.</p>
    pub fn set_voice_connector_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.voice_connector_id = input;
        self
    }
    /// <p>The logging configuration being updated.</p>
    pub fn logging_configuration(mut self, input: crate::types::LoggingConfiguration) -> Self {
        self.logging_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The logging configuration being updated.</p>
    pub fn set_logging_configuration(
        mut self,
        input: ::std::option::Option<crate::types::LoggingConfiguration>,
    ) -> Self {
        self.logging_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`PutVoiceConnectorLoggingConfigurationInput`](crate::operation::put_voice_connector_logging_configuration::PutVoiceConnectorLoggingConfigurationInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::put_voice_connector_logging_configuration::PutVoiceConnectorLoggingConfigurationInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::put_voice_connector_logging_configuration::PutVoiceConnectorLoggingConfigurationInput {
                voice_connector_id: self.voice_connector_id
                ,
                logging_configuration: self.logging_configuration
                ,
            }
        )
    }
}
