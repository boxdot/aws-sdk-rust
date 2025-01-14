// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The log delivery configurations being modified </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PendingLogDeliveryConfiguration {
    /// <p>Refers to <a href="https://redis.io/commands/slowlog">slow-log</a> or engine-log..</p>
    #[doc(hidden)]
    pub log_type: ::std::option::Option<crate::types::LogType>,
    /// <p>Returns the destination type, either CloudWatch Logs or Kinesis Data Firehose.</p>
    #[doc(hidden)]
    pub destination_type: ::std::option::Option<crate::types::DestinationType>,
    /// <p>Configuration details of either a CloudWatch Logs destination or Kinesis Data Firehose destination.</p>
    #[doc(hidden)]
    pub destination_details: ::std::option::Option<crate::types::DestinationDetails>,
    /// <p>Returns the log format, either JSON or TEXT</p>
    #[doc(hidden)]
    pub log_format: ::std::option::Option<crate::types::LogFormat>,
}
impl PendingLogDeliveryConfiguration {
    /// <p>Refers to <a href="https://redis.io/commands/slowlog">slow-log</a> or engine-log..</p>
    pub fn log_type(&self) -> ::std::option::Option<&crate::types::LogType> {
        self.log_type.as_ref()
    }
    /// <p>Returns the destination type, either CloudWatch Logs or Kinesis Data Firehose.</p>
    pub fn destination_type(&self) -> ::std::option::Option<&crate::types::DestinationType> {
        self.destination_type.as_ref()
    }
    /// <p>Configuration details of either a CloudWatch Logs destination or Kinesis Data Firehose destination.</p>
    pub fn destination_details(&self) -> ::std::option::Option<&crate::types::DestinationDetails> {
        self.destination_details.as_ref()
    }
    /// <p>Returns the log format, either JSON or TEXT</p>
    pub fn log_format(&self) -> ::std::option::Option<&crate::types::LogFormat> {
        self.log_format.as_ref()
    }
}
impl PendingLogDeliveryConfiguration {
    /// Creates a new builder-style object to manufacture [`PendingLogDeliveryConfiguration`](crate::types::PendingLogDeliveryConfiguration).
    pub fn builder() -> crate::types::builders::PendingLogDeliveryConfigurationBuilder {
        crate::types::builders::PendingLogDeliveryConfigurationBuilder::default()
    }
}

/// A builder for [`PendingLogDeliveryConfiguration`](crate::types::PendingLogDeliveryConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PendingLogDeliveryConfigurationBuilder {
    pub(crate) log_type: ::std::option::Option<crate::types::LogType>,
    pub(crate) destination_type: ::std::option::Option<crate::types::DestinationType>,
    pub(crate) destination_details: ::std::option::Option<crate::types::DestinationDetails>,
    pub(crate) log_format: ::std::option::Option<crate::types::LogFormat>,
}
impl PendingLogDeliveryConfigurationBuilder {
    /// <p>Refers to <a href="https://redis.io/commands/slowlog">slow-log</a> or engine-log..</p>
    pub fn log_type(mut self, input: crate::types::LogType) -> Self {
        self.log_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Refers to <a href="https://redis.io/commands/slowlog">slow-log</a> or engine-log..</p>
    pub fn set_log_type(mut self, input: ::std::option::Option<crate::types::LogType>) -> Self {
        self.log_type = input;
        self
    }
    /// <p>Returns the destination type, either CloudWatch Logs or Kinesis Data Firehose.</p>
    pub fn destination_type(mut self, input: crate::types::DestinationType) -> Self {
        self.destination_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns the destination type, either CloudWatch Logs or Kinesis Data Firehose.</p>
    pub fn set_destination_type(
        mut self,
        input: ::std::option::Option<crate::types::DestinationType>,
    ) -> Self {
        self.destination_type = input;
        self
    }
    /// <p>Configuration details of either a CloudWatch Logs destination or Kinesis Data Firehose destination.</p>
    pub fn destination_details(mut self, input: crate::types::DestinationDetails) -> Self {
        self.destination_details = ::std::option::Option::Some(input);
        self
    }
    /// <p>Configuration details of either a CloudWatch Logs destination or Kinesis Data Firehose destination.</p>
    pub fn set_destination_details(
        mut self,
        input: ::std::option::Option<crate::types::DestinationDetails>,
    ) -> Self {
        self.destination_details = input;
        self
    }
    /// <p>Returns the log format, either JSON or TEXT</p>
    pub fn log_format(mut self, input: crate::types::LogFormat) -> Self {
        self.log_format = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns the log format, either JSON or TEXT</p>
    pub fn set_log_format(mut self, input: ::std::option::Option<crate::types::LogFormat>) -> Self {
        self.log_format = input;
        self
    }
    /// Consumes the builder and constructs a [`PendingLogDeliveryConfiguration`](crate::types::PendingLogDeliveryConfiguration).
    pub fn build(self) -> crate::types::PendingLogDeliveryConfiguration {
        crate::types::PendingLogDeliveryConfiguration {
            log_type: self.log_type,
            destination_type: self.destination_type,
            destination_details: self.destination_details,
            log_format: self.log_format,
        }
    }
}
