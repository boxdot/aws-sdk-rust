// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::check_in_license::_check_in_license_output::CheckInLicenseOutputBuilder;

pub use crate::operation::check_in_license::_check_in_license_input::CheckInLicenseInputBuilder;

/// Fluent builder constructing a request to `CheckInLicense`.
///
/// <p>Checks in the specified license. Check in a license when it is no longer in use.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CheckInLicenseFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::check_in_license::builders::CheckInLicenseInputBuilder,
}
impl CheckInLicenseFluentBuilder {
    /// Creates a new `CheckInLicense`.
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
            crate::operation::check_in_license::CheckInLicense,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::check_in_license::CheckInLicenseError,
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
        crate::operation::check_in_license::CheckInLicenseOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::check_in_license::CheckInLicenseError,
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
        crate::operation::check_in_license::CheckInLicenseOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::check_in_license::CheckInLicenseError,
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
            crate::operation::check_in_license::CheckInLicense,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::check_in_license::CheckInLicenseError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>License consumption token.</p>
    pub fn license_consumption_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.license_consumption_token(input.into());
        self
    }
    /// <p>License consumption token.</p>
    pub fn set_license_consumption_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_license_consumption_token(input);
        self
    }
    /// <p>License beneficiary.</p>
    pub fn beneficiary(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.beneficiary(input.into());
        self
    }
    /// <p>License beneficiary.</p>
    pub fn set_beneficiary(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_beneficiary(input);
        self
    }
}
