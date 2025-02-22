// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_sms_sandbox_phone_number::_create_sms_sandbox_phone_number_output::CreateSmsSandboxPhoneNumberOutputBuilder;

pub use crate::operation::create_sms_sandbox_phone_number::_create_sms_sandbox_phone_number_input::CreateSmsSandboxPhoneNumberInputBuilder;

/// Fluent builder constructing a request to `CreateSMSSandboxPhoneNumber`.
///
/// <p>Adds a destination phone number to an Amazon Web Services account in the SMS sandbox and sends a one-time password (OTP) to that phone number.</p>
/// <p>When you start using Amazon SNS to send SMS messages, your Amazon Web Services account is in the <i>SMS sandbox</i>. The SMS sandbox provides a safe environment for you to try Amazon SNS features without risking your reputation as an SMS sender. While your Amazon Web Services account is in the SMS sandbox, you can use all of the features of Amazon SNS. However, you can send SMS messages only to verified destination phone numbers. For more information, including how to move out of the sandbox to send messages without restrictions, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html">SMS sandbox</a> in the <i>Amazon SNS Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSMSSandboxPhoneNumberFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_sms_sandbox_phone_number::builders::CreateSmsSandboxPhoneNumberInputBuilder,
}
impl CreateSMSSandboxPhoneNumberFluentBuilder {
    /// Creates a new `CreateSMSSandboxPhoneNumber`.
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
            crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumber,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
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
        crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
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
        crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
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
            crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumber,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_sms_sandbox_phone_number::CreateSMSSandboxPhoneNumberError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The destination phone number to verify. On verification, Amazon SNS adds this phone number to the list of verified phone numbers that you can send SMS messages to.</p>
    pub fn phone_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.phone_number(input.into());
        self
    }
    /// <p>The destination phone number to verify. On verification, Amazon SNS adds this phone number to the list of verified phone numbers that you can send SMS messages to.</p>
    pub fn set_phone_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_phone_number(input);
        self
    }
    /// <p>The language to use for sending the OTP. The default value is <code>en-US</code>.</p>
    pub fn language_code(mut self, input: crate::types::LanguageCodeString) -> Self {
        self.inner = self.inner.language_code(input);
        self
    }
    /// <p>The language to use for sending the OTP. The default value is <code>en-US</code>.</p>
    pub fn set_language_code(
        mut self,
        input: ::std::option::Option<crate::types::LanguageCodeString>,
    ) -> Self {
        self.inner = self.inner.set_language_code(input);
        self
    }
}
