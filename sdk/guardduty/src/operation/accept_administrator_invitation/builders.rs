// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::accept_administrator_invitation::_accept_administrator_invitation_output::AcceptAdministratorInvitationOutputBuilder;

pub use crate::operation::accept_administrator_invitation::_accept_administrator_invitation_input::AcceptAdministratorInvitationInputBuilder;

/// Fluent builder constructing a request to `AcceptAdministratorInvitation`.
///
/// <p>Accepts the invitation to be a member account and get monitored by a GuardDuty administrator account that sent the invitation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AcceptAdministratorInvitationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::accept_administrator_invitation::builders::AcceptAdministratorInvitationInputBuilder,
}
impl AcceptAdministratorInvitationFluentBuilder {
    /// Creates a new `AcceptAdministratorInvitation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::accept_administrator_invitation::AcceptAdministratorInvitation,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::accept_administrator_invitation::AcceptAdministratorInvitation,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique ID of the detector of the GuardDuty member account.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The unique ID of the detector of the GuardDuty member account.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// <p>The account ID of the GuardDuty administrator account whose invitation you're accepting.</p>
    pub fn administrator_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.administrator_id(input.into());
        self
    }
    /// <p>The account ID of the GuardDuty administrator account whose invitation you're accepting.</p>
    pub fn set_administrator_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_administrator_id(input);
        self
    }
    /// <p>The value that is used to validate the administrator account to the member account.</p>
    pub fn invitation_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.invitation_id(input.into());
        self
    }
    /// <p>The value that is used to validate the administrator account to the member account.</p>
    pub fn set_invitation_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_invitation_id(input);
        self
    }
}
