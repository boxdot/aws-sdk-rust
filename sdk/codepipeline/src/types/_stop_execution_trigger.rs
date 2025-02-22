// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The interaction that stopped a pipeline execution.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StopExecutionTrigger {
    /// <p>The user-specified reason the pipeline was stopped.</p>
    #[doc(hidden)]
    pub reason: ::std::option::Option<::std::string::String>,
}
impl StopExecutionTrigger {
    /// <p>The user-specified reason the pipeline was stopped.</p>
    pub fn reason(&self) -> ::std::option::Option<&str> {
        self.reason.as_deref()
    }
}
impl StopExecutionTrigger {
    /// Creates a new builder-style object to manufacture [`StopExecutionTrigger`](crate::types::StopExecutionTrigger).
    pub fn builder() -> crate::types::builders::StopExecutionTriggerBuilder {
        crate::types::builders::StopExecutionTriggerBuilder::default()
    }
}

/// A builder for [`StopExecutionTrigger`](crate::types::StopExecutionTrigger).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StopExecutionTriggerBuilder {
    pub(crate) reason: ::std::option::Option<::std::string::String>,
}
impl StopExecutionTriggerBuilder {
    /// <p>The user-specified reason the pipeline was stopped.</p>
    pub fn reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user-specified reason the pipeline was stopped.</p>
    pub fn set_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reason = input;
        self
    }
    /// Consumes the builder and constructs a [`StopExecutionTrigger`](crate::types::StopExecutionTrigger).
    pub fn build(self) -> crate::types::StopExecutionTrigger {
        crate::types::StopExecutionTrigger {
            reason: self.reason,
        }
    }
}
