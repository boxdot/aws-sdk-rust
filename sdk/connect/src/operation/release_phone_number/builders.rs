// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::release_phone_number::_release_phone_number_output::ReleasePhoneNumberOutputBuilder;

pub use crate::operation::release_phone_number::_release_phone_number_input::ReleasePhoneNumberInputBuilder;

/// Fluent builder constructing a request to `ReleasePhoneNumber`.
///
/// <p>Releases a phone number previously claimed to an Amazon Connect instance or traffic distribution group. You can call this API only in the Amazon Web Services Region where the number was claimed.</p> <important>
/// <p>To release phone numbers from a traffic distribution group, use the <code>ReleasePhoneNumber</code> API, not the Amazon Connect console.</p>
/// <p>After releasing a phone number, the phone number enters into a cooldown period of 30 days. It cannot be searched for or claimed again until the period has ended. If you accidentally release a phone number, contact Amazon Web Services Support.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ReleasePhoneNumberFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::release_phone_number::builders::ReleasePhoneNumberInputBuilder,
}
impl ReleasePhoneNumberFluentBuilder {
    /// Creates a new `ReleasePhoneNumber`.
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
            crate::operation::release_phone_number::ReleasePhoneNumber,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::release_phone_number::ReleasePhoneNumberError,
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
        crate::operation::release_phone_number::ReleasePhoneNumberOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::release_phone_number::ReleasePhoneNumberError,
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
        crate::operation::release_phone_number::ReleasePhoneNumberOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::release_phone_number::ReleasePhoneNumberError,
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
            crate::operation::release_phone_number::ReleasePhoneNumber,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::release_phone_number::ReleasePhoneNumberError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A unique identifier for the phone number.</p>
    pub fn phone_number_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.phone_number_id(input.into());
        self
    }
    /// <p>A unique identifier for the phone number.</p>
    pub fn set_phone_number_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_phone_number_id(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
