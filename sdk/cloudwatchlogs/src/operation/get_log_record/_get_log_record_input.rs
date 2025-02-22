// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetLogRecordInput {
    /// <p>The pointer corresponding to the log event record you want to retrieve. You get this from the response of a <code>GetQueryResults</code> operation. In that response, the value of the <code>@ptr</code> field for a log event is the value to use as <code>logRecordPointer</code> to retrieve that complete log event record.</p>
    #[doc(hidden)]
    pub log_record_pointer: ::std::option::Option<::std::string::String>,
    /// <p>Specify <code>true</code> to display the log event fields with all sensitive data unmasked and visible. The default is <code>false</code>.</p>
    /// <p>To use this operation with this parameter, you must be signed into an account with the <code>logs:Unmask</code> permission.</p>
    #[doc(hidden)]
    pub unmask: bool,
}
impl GetLogRecordInput {
    /// <p>The pointer corresponding to the log event record you want to retrieve. You get this from the response of a <code>GetQueryResults</code> operation. In that response, the value of the <code>@ptr</code> field for a log event is the value to use as <code>logRecordPointer</code> to retrieve that complete log event record.</p>
    pub fn log_record_pointer(&self) -> ::std::option::Option<&str> {
        self.log_record_pointer.as_deref()
    }
    /// <p>Specify <code>true</code> to display the log event fields with all sensitive data unmasked and visible. The default is <code>false</code>.</p>
    /// <p>To use this operation with this parameter, you must be signed into an account with the <code>logs:Unmask</code> permission.</p>
    pub fn unmask(&self) -> bool {
        self.unmask
    }
}
impl GetLogRecordInput {
    /// Creates a new builder-style object to manufacture [`GetLogRecordInput`](crate::operation::get_log_record::GetLogRecordInput).
    pub fn builder() -> crate::operation::get_log_record::builders::GetLogRecordInputBuilder {
        crate::operation::get_log_record::builders::GetLogRecordInputBuilder::default()
    }
}

/// A builder for [`GetLogRecordInput`](crate::operation::get_log_record::GetLogRecordInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetLogRecordInputBuilder {
    pub(crate) log_record_pointer: ::std::option::Option<::std::string::String>,
    pub(crate) unmask: ::std::option::Option<bool>,
}
impl GetLogRecordInputBuilder {
    /// <p>The pointer corresponding to the log event record you want to retrieve. You get this from the response of a <code>GetQueryResults</code> operation. In that response, the value of the <code>@ptr</code> field for a log event is the value to use as <code>logRecordPointer</code> to retrieve that complete log event record.</p>
    pub fn log_record_pointer(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.log_record_pointer = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pointer corresponding to the log event record you want to retrieve. You get this from the response of a <code>GetQueryResults</code> operation. In that response, the value of the <code>@ptr</code> field for a log event is the value to use as <code>logRecordPointer</code> to retrieve that complete log event record.</p>
    pub fn set_log_record_pointer(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.log_record_pointer = input;
        self
    }
    /// <p>Specify <code>true</code> to display the log event fields with all sensitive data unmasked and visible. The default is <code>false</code>.</p>
    /// <p>To use this operation with this parameter, you must be signed into an account with the <code>logs:Unmask</code> permission.</p>
    pub fn unmask(mut self, input: bool) -> Self {
        self.unmask = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specify <code>true</code> to display the log event fields with all sensitive data unmasked and visible. The default is <code>false</code>.</p>
    /// <p>To use this operation with this parameter, you must be signed into an account with the <code>logs:Unmask</code> permission.</p>
    pub fn set_unmask(mut self, input: ::std::option::Option<bool>) -> Self {
        self.unmask = input;
        self
    }
    /// Consumes the builder and constructs a [`GetLogRecordInput`](crate::operation::get_log_record::GetLogRecordInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_log_record::GetLogRecordInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_log_record::GetLogRecordInput {
            log_record_pointer: self.log_record_pointer,
            unmask: self.unmask.unwrap_or_default(),
        })
    }
}
