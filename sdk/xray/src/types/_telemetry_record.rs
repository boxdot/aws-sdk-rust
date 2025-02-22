// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TelemetryRecord {
    /// <p></p>
    #[doc(hidden)]
    pub timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p></p>
    #[doc(hidden)]
    pub segments_received_count: ::std::option::Option<i32>,
    /// <p></p>
    #[doc(hidden)]
    pub segments_sent_count: ::std::option::Option<i32>,
    /// <p></p>
    #[doc(hidden)]
    pub segments_spillover_count: ::std::option::Option<i32>,
    /// <p></p>
    #[doc(hidden)]
    pub segments_rejected_count: ::std::option::Option<i32>,
    /// <p></p>
    #[doc(hidden)]
    pub backend_connection_errors: ::std::option::Option<crate::types::BackendConnectionErrors>,
}
impl TelemetryRecord {
    /// <p></p>
    pub fn timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }
    /// <p></p>
    pub fn segments_received_count(&self) -> ::std::option::Option<i32> {
        self.segments_received_count
    }
    /// <p></p>
    pub fn segments_sent_count(&self) -> ::std::option::Option<i32> {
        self.segments_sent_count
    }
    /// <p></p>
    pub fn segments_spillover_count(&self) -> ::std::option::Option<i32> {
        self.segments_spillover_count
    }
    /// <p></p>
    pub fn segments_rejected_count(&self) -> ::std::option::Option<i32> {
        self.segments_rejected_count
    }
    /// <p></p>
    pub fn backend_connection_errors(
        &self,
    ) -> ::std::option::Option<&crate::types::BackendConnectionErrors> {
        self.backend_connection_errors.as_ref()
    }
}
impl TelemetryRecord {
    /// Creates a new builder-style object to manufacture [`TelemetryRecord`](crate::types::TelemetryRecord).
    pub fn builder() -> crate::types::builders::TelemetryRecordBuilder {
        crate::types::builders::TelemetryRecordBuilder::default()
    }
}

/// A builder for [`TelemetryRecord`](crate::types::TelemetryRecord).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TelemetryRecordBuilder {
    pub(crate) timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) segments_received_count: ::std::option::Option<i32>,
    pub(crate) segments_sent_count: ::std::option::Option<i32>,
    pub(crate) segments_spillover_count: ::std::option::Option<i32>,
    pub(crate) segments_rejected_count: ::std::option::Option<i32>,
    pub(crate) backend_connection_errors:
        ::std::option::Option<crate::types::BackendConnectionErrors>,
}
impl TelemetryRecordBuilder {
    /// <p></p>
    pub fn timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p></p>
    pub fn set_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.timestamp = input;
        self
    }
    /// <p></p>
    pub fn segments_received_count(mut self, input: i32) -> Self {
        self.segments_received_count = ::std::option::Option::Some(input);
        self
    }
    /// <p></p>
    pub fn set_segments_received_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.segments_received_count = input;
        self
    }
    /// <p></p>
    pub fn segments_sent_count(mut self, input: i32) -> Self {
        self.segments_sent_count = ::std::option::Option::Some(input);
        self
    }
    /// <p></p>
    pub fn set_segments_sent_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.segments_sent_count = input;
        self
    }
    /// <p></p>
    pub fn segments_spillover_count(mut self, input: i32) -> Self {
        self.segments_spillover_count = ::std::option::Option::Some(input);
        self
    }
    /// <p></p>
    pub fn set_segments_spillover_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.segments_spillover_count = input;
        self
    }
    /// <p></p>
    pub fn segments_rejected_count(mut self, input: i32) -> Self {
        self.segments_rejected_count = ::std::option::Option::Some(input);
        self
    }
    /// <p></p>
    pub fn set_segments_rejected_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.segments_rejected_count = input;
        self
    }
    /// <p></p>
    pub fn backend_connection_errors(
        mut self,
        input: crate::types::BackendConnectionErrors,
    ) -> Self {
        self.backend_connection_errors = ::std::option::Option::Some(input);
        self
    }
    /// <p></p>
    pub fn set_backend_connection_errors(
        mut self,
        input: ::std::option::Option<crate::types::BackendConnectionErrors>,
    ) -> Self {
        self.backend_connection_errors = input;
        self
    }
    /// Consumes the builder and constructs a [`TelemetryRecord`](crate::types::TelemetryRecord).
    pub fn build(self) -> crate::types::TelemetryRecord {
        crate::types::TelemetryRecord {
            timestamp: self.timestamp,
            segments_received_count: self.segments_received_count,
            segments_sent_count: self.segments_sent_count,
            segments_spillover_count: self.segments_spillover_count,
            segments_rejected_count: self.segments_rejected_count,
            backend_connection_errors: self.backend_connection_errors,
        }
    }
}
