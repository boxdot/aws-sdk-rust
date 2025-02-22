// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Amazon Security Lake collects logs and events from supported Amazon Web Services and custom sources. For the list of supported Amazon Web Services, see the <a href="https://docs.aws.amazon.com/security-lake/latest/userguide/internal-sources.html">Amazon Security Lake User Guide</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AccountSources {
    /// <p>The ID of the Security Lake account for which logs are collected.</p>
    #[doc(hidden)]
    pub account: ::std::option::Option<::std::string::String>,
    /// <p>The supported Amazon Web Services from which logs and events are collected. Amazon Security Lake supports log and event collection for natively supported Amazon Web Services. </p>
    #[doc(hidden)]
    pub source_type: ::std::option::Option<::std::string::String>,
    /// <p>The log status for the Security Lake account.</p>
    #[doc(hidden)]
    pub logs_status: ::std::option::Option<::std::vec::Vec<crate::types::LogsStatus>>,
    /// <p>Initializes a new instance of the Event class.</p>
    #[doc(hidden)]
    pub event_class: ::std::option::Option<crate::types::OcsfEventClass>,
}
impl AccountSources {
    /// <p>The ID of the Security Lake account for which logs are collected.</p>
    pub fn account(&self) -> ::std::option::Option<&str> {
        self.account.as_deref()
    }
    /// <p>The supported Amazon Web Services from which logs and events are collected. Amazon Security Lake supports log and event collection for natively supported Amazon Web Services. </p>
    pub fn source_type(&self) -> ::std::option::Option<&str> {
        self.source_type.as_deref()
    }
    /// <p>The log status for the Security Lake account.</p>
    pub fn logs_status(&self) -> ::std::option::Option<&[crate::types::LogsStatus]> {
        self.logs_status.as_deref()
    }
    /// <p>Initializes a new instance of the Event class.</p>
    pub fn event_class(&self) -> ::std::option::Option<&crate::types::OcsfEventClass> {
        self.event_class.as_ref()
    }
}
impl AccountSources {
    /// Creates a new builder-style object to manufacture [`AccountSources`](crate::types::AccountSources).
    pub fn builder() -> crate::types::builders::AccountSourcesBuilder {
        crate::types::builders::AccountSourcesBuilder::default()
    }
}

/// A builder for [`AccountSources`](crate::types::AccountSources).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AccountSourcesBuilder {
    pub(crate) account: ::std::option::Option<::std::string::String>,
    pub(crate) source_type: ::std::option::Option<::std::string::String>,
    pub(crate) logs_status: ::std::option::Option<::std::vec::Vec<crate::types::LogsStatus>>,
    pub(crate) event_class: ::std::option::Option<crate::types::OcsfEventClass>,
}
impl AccountSourcesBuilder {
    /// <p>The ID of the Security Lake account for which logs are collected.</p>
    pub fn account(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Security Lake account for which logs are collected.</p>
    pub fn set_account(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account = input;
        self
    }
    /// <p>The supported Amazon Web Services from which logs and events are collected. Amazon Security Lake supports log and event collection for natively supported Amazon Web Services. </p>
    pub fn source_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The supported Amazon Web Services from which logs and events are collected. Amazon Security Lake supports log and event collection for natively supported Amazon Web Services. </p>
    pub fn set_source_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_type = input;
        self
    }
    /// Appends an item to `logs_status`.
    ///
    /// To override the contents of this collection use [`set_logs_status`](Self::set_logs_status).
    ///
    /// <p>The log status for the Security Lake account.</p>
    pub fn logs_status(mut self, input: crate::types::LogsStatus) -> Self {
        let mut v = self.logs_status.unwrap_or_default();
        v.push(input);
        self.logs_status = ::std::option::Option::Some(v);
        self
    }
    /// <p>The log status for the Security Lake account.</p>
    pub fn set_logs_status(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::LogsStatus>>,
    ) -> Self {
        self.logs_status = input;
        self
    }
    /// <p>Initializes a new instance of the Event class.</p>
    pub fn event_class(mut self, input: crate::types::OcsfEventClass) -> Self {
        self.event_class = ::std::option::Option::Some(input);
        self
    }
    /// <p>Initializes a new instance of the Event class.</p>
    pub fn set_event_class(
        mut self,
        input: ::std::option::Option<crate::types::OcsfEventClass>,
    ) -> Self {
        self.event_class = input;
        self
    }
    /// Consumes the builder and constructs a [`AccountSources`](crate::types::AccountSources).
    pub fn build(self) -> crate::types::AccountSources {
        crate::types::AccountSources {
            account: self.account,
            source_type: self.source_type,
            logs_status: self.logs_status,
            event_class: self.event_class,
        }
    }
}
