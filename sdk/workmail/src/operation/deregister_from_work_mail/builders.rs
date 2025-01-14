// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deregister_from_work_mail::_deregister_from_work_mail_output::DeregisterFromWorkMailOutputBuilder;

pub use crate::operation::deregister_from_work_mail::_deregister_from_work_mail_input::DeregisterFromWorkMailInputBuilder;

/// Fluent builder constructing a request to `DeregisterFromWorkMail`.
///
/// <p>Mark a user, group, or resource as no longer used in WorkMail. This action disassociates the mailbox and schedules it for clean-up. WorkMail keeps mailboxes for 30 days before they are permanently removed. The functionality in the console is <i>Disable</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeregisterFromWorkMailFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::deregister_from_work_mail::builders::DeregisterFromWorkMailInputBuilder,
}
impl DeregisterFromWorkMailFluentBuilder {
    /// Creates a new `DeregisterFromWorkMail`.
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
            crate::operation::deregister_from_work_mail::DeregisterFromWorkMail,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::deregister_from_work_mail::DeregisterFromWorkMailError,
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
        crate::operation::deregister_from_work_mail::DeregisterFromWorkMailOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::deregister_from_work_mail::DeregisterFromWorkMailError,
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
        crate::operation::deregister_from_work_mail::DeregisterFromWorkMailOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::deregister_from_work_mail::DeregisterFromWorkMailError,
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
            crate::operation::deregister_from_work_mail::DeregisterFromWorkMail,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::deregister_from_work_mail::DeregisterFromWorkMailError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The identifier for the organization under which the WorkMail entity exists.</p>
    pub fn organization_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>The identifier for the organization under which the WorkMail entity exists.</p>
    pub fn set_organization_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>The identifier for the member (user or group) to be updated.</p>
    pub fn entity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.entity_id(input.into());
        self
    }
    /// <p>The identifier for the member (user or group) to be updated.</p>
    pub fn set_entity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_entity_id(input);
        self
    }
}
