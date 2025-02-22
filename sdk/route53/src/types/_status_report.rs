// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains the status that one Amazon Route 53 health checker reports and the time of the health check.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StatusReport {
    /// <p>A description of the status of the health check endpoint as reported by one of the Amazon Route 53 health checkers.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<::std::string::String>,
    /// <p>The date and time that the health checker performed the health check in <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601 format</a> and Coordinated Universal Time (UTC). For example, the value <code>2017-03-27T17:48:16.751Z</code> represents March 27, 2017 at 17:48:16.751 UTC.</p>
    #[doc(hidden)]
    pub checked_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl StatusReport {
    /// <p>A description of the status of the health check endpoint as reported by one of the Amazon Route 53 health checkers.</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>The date and time that the health checker performed the health check in <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601 format</a> and Coordinated Universal Time (UTC). For example, the value <code>2017-03-27T17:48:16.751Z</code> represents March 27, 2017 at 17:48:16.751 UTC.</p>
    pub fn checked_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.checked_time.as_ref()
    }
}
impl StatusReport {
    /// Creates a new builder-style object to manufacture [`StatusReport`](crate::types::StatusReport).
    pub fn builder() -> crate::types::builders::StatusReportBuilder {
        crate::types::builders::StatusReportBuilder::default()
    }
}

/// A builder for [`StatusReport`](crate::types::StatusReport).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StatusReportBuilder {
    pub(crate) status: ::std::option::Option<::std::string::String>,
    pub(crate) checked_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl StatusReportBuilder {
    /// <p>A description of the status of the health check endpoint as reported by one of the Amazon Route 53 health checkers.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the status of the health check endpoint as reported by one of the Amazon Route 53 health checkers.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>The date and time that the health checker performed the health check in <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601 format</a> and Coordinated Universal Time (UTC). For example, the value <code>2017-03-27T17:48:16.751Z</code> represents March 27, 2017 at 17:48:16.751 UTC.</p>
    pub fn checked_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.checked_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time that the health checker performed the health check in <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601 format</a> and Coordinated Universal Time (UTC). For example, the value <code>2017-03-27T17:48:16.751Z</code> represents March 27, 2017 at 17:48:16.751 UTC.</p>
    pub fn set_checked_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.checked_time = input;
        self
    }
    /// Consumes the builder and constructs a [`StatusReport`](crate::types::StatusReport).
    pub fn build(self) -> crate::types::StatusReport {
        crate::types::StatusReport {
            status: self.status,
            checked_time: self.checked_time,
        }
    }
}
