// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a Session Manager connection to a managed node.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Session {
    /// <p>The ID of the session.</p>
    #[doc(hidden)]
    pub session_id: ::std::option::Option<::std::string::String>,
    /// <p>The managed node that the Session Manager session connected to.</p>
    #[doc(hidden)]
    pub target: ::std::option::Option<::std::string::String>,
    /// <p>The status of the session. For example, "Connected" or "Terminated".</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::SessionStatus>,
    /// <p>The date and time, in ISO-8601 Extended format, when the session began.</p>
    #[doc(hidden)]
    pub start_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date and time, in ISO-8601 Extended format, when the session was terminated.</p>
    #[doc(hidden)]
    pub end_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The name of the Session Manager SSM document used to define the parameters and plugin settings for the session. For example, <code>SSM-SessionManagerRunShell</code>.</p>
    #[doc(hidden)]
    pub document_name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Amazon Web Services user that started the session.</p>
    #[doc(hidden)]
    pub owner: ::std::option::Option<::std::string::String>,
    /// <p>The reason for connecting to the instance.</p>
    #[doc(hidden)]
    pub reason: ::std::option::Option<::std::string::String>,
    /// <p>Reserved for future use.</p>
    #[doc(hidden)]
    pub details: ::std::option::Option<::std::string::String>,
    /// <p>Reserved for future use.</p>
    #[doc(hidden)]
    pub output_url: ::std::option::Option<crate::types::SessionManagerOutputUrl>,
    /// <p>The maximum duration of a session before it terminates.</p>
    #[doc(hidden)]
    pub max_session_duration: ::std::option::Option<::std::string::String>,
}
impl Session {
    /// <p>The ID of the session.</p>
    pub fn session_id(&self) -> ::std::option::Option<&str> {
        self.session_id.as_deref()
    }
    /// <p>The managed node that the Session Manager session connected to.</p>
    pub fn target(&self) -> ::std::option::Option<&str> {
        self.target.as_deref()
    }
    /// <p>The status of the session. For example, "Connected" or "Terminated".</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::SessionStatus> {
        self.status.as_ref()
    }
    /// <p>The date and time, in ISO-8601 Extended format, when the session began.</p>
    pub fn start_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_date.as_ref()
    }
    /// <p>The date and time, in ISO-8601 Extended format, when the session was terminated.</p>
    pub fn end_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.end_date.as_ref()
    }
    /// <p>The name of the Session Manager SSM document used to define the parameters and plugin settings for the session. For example, <code>SSM-SessionManagerRunShell</code>.</p>
    pub fn document_name(&self) -> ::std::option::Option<&str> {
        self.document_name.as_deref()
    }
    /// <p>The ID of the Amazon Web Services user that started the session.</p>
    pub fn owner(&self) -> ::std::option::Option<&str> {
        self.owner.as_deref()
    }
    /// <p>The reason for connecting to the instance.</p>
    pub fn reason(&self) -> ::std::option::Option<&str> {
        self.reason.as_deref()
    }
    /// <p>Reserved for future use.</p>
    pub fn details(&self) -> ::std::option::Option<&str> {
        self.details.as_deref()
    }
    /// <p>Reserved for future use.</p>
    pub fn output_url(&self) -> ::std::option::Option<&crate::types::SessionManagerOutputUrl> {
        self.output_url.as_ref()
    }
    /// <p>The maximum duration of a session before it terminates.</p>
    pub fn max_session_duration(&self) -> ::std::option::Option<&str> {
        self.max_session_duration.as_deref()
    }
}
impl Session {
    /// Creates a new builder-style object to manufacture [`Session`](crate::types::Session).
    pub fn builder() -> crate::types::builders::SessionBuilder {
        crate::types::builders::SessionBuilder::default()
    }
}

/// A builder for [`Session`](crate::types::Session).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SessionBuilder {
    pub(crate) session_id: ::std::option::Option<::std::string::String>,
    pub(crate) target: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::SessionStatus>,
    pub(crate) start_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) end_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) document_name: ::std::option::Option<::std::string::String>,
    pub(crate) owner: ::std::option::Option<::std::string::String>,
    pub(crate) reason: ::std::option::Option<::std::string::String>,
    pub(crate) details: ::std::option::Option<::std::string::String>,
    pub(crate) output_url: ::std::option::Option<crate::types::SessionManagerOutputUrl>,
    pub(crate) max_session_duration: ::std::option::Option<::std::string::String>,
}
impl SessionBuilder {
    /// <p>The ID of the session.</p>
    pub fn session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.session_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the session.</p>
    pub fn set_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.session_id = input;
        self
    }
    /// <p>The managed node that the Session Manager session connected to.</p>
    pub fn target(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.target = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The managed node that the Session Manager session connected to.</p>
    pub fn set_target(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.target = input;
        self
    }
    /// <p>The status of the session. For example, "Connected" or "Terminated".</p>
    pub fn status(mut self, input: crate::types::SessionStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the session. For example, "Connected" or "Terminated".</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::SessionStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The date and time, in ISO-8601 Extended format, when the session began.</p>
    pub fn start_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time, in ISO-8601 Extended format, when the session began.</p>
    pub fn set_start_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.start_date = input;
        self
    }
    /// <p>The date and time, in ISO-8601 Extended format, when the session was terminated.</p>
    pub fn end_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.end_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time, in ISO-8601 Extended format, when the session was terminated.</p>
    pub fn set_end_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.end_date = input;
        self
    }
    /// <p>The name of the Session Manager SSM document used to define the parameters and plugin settings for the session. For example, <code>SSM-SessionManagerRunShell</code>.</p>
    pub fn document_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.document_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Session Manager SSM document used to define the parameters and plugin settings for the session. For example, <code>SSM-SessionManagerRunShell</code>.</p>
    pub fn set_document_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.document_name = input;
        self
    }
    /// <p>The ID of the Amazon Web Services user that started the session.</p>
    pub fn owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services user that started the session.</p>
    pub fn set_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner = input;
        self
    }
    /// <p>The reason for connecting to the instance.</p>
    pub fn reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reason for connecting to the instance.</p>
    pub fn set_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reason = input;
        self
    }
    /// <p>Reserved for future use.</p>
    pub fn details(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.details = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Reserved for future use.</p>
    pub fn set_details(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.details = input;
        self
    }
    /// <p>Reserved for future use.</p>
    pub fn output_url(mut self, input: crate::types::SessionManagerOutputUrl) -> Self {
        self.output_url = ::std::option::Option::Some(input);
        self
    }
    /// <p>Reserved for future use.</p>
    pub fn set_output_url(
        mut self,
        input: ::std::option::Option<crate::types::SessionManagerOutputUrl>,
    ) -> Self {
        self.output_url = input;
        self
    }
    /// <p>The maximum duration of a session before it terminates.</p>
    pub fn max_session_duration(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.max_session_duration = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The maximum duration of a session before it terminates.</p>
    pub fn set_max_session_duration(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.max_session_duration = input;
        self
    }
    /// Consumes the builder and constructs a [`Session`](crate::types::Session).
    pub fn build(self) -> crate::types::Session {
        crate::types::Session {
            session_id: self.session_id,
            target: self.target,
            status: self.status,
            start_date: self.start_date,
            end_date: self.end_date,
            document_name: self.document_name,
            owner: self.owner,
            reason: self.reason,
            details: self.details,
            output_url: self.output_url,
            max_session_duration: self.max_session_duration,
        }
    }
}
