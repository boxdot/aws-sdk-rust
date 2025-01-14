// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_kms_key::_disassociate_kms_key_output::DisassociateKmsKeyOutputBuilder;

pub use crate::operation::disassociate_kms_key::_disassociate_kms_key_input::DisassociateKmsKeyInputBuilder;

/// Fluent builder constructing a request to `DisassociateKmsKey`.
///
/// <p>Disassociates the associated KMS key from the specified log group.</p>
/// <p>After the KMS key is disassociated from the log group, CloudWatch Logs stops encrypting newly ingested data for the log group. All previously ingested data remains encrypted, and CloudWatch Logs requires permissions for the KMS key whenever the encrypted data is requested.</p>
/// <p>Note that it can take up to 5 minutes for this operation to take effect.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateKmsKeyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_kms_key::builders::DisassociateKmsKeyInputBuilder,
}
impl DisassociateKmsKeyFluentBuilder {
    /// Creates a new `DisassociateKmsKey`.
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
            crate::operation::disassociate_kms_key::DisassociateKmsKey,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_kms_key::DisassociateKmsKeyError,
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
        crate::operation::disassociate_kms_key::DisassociateKmsKeyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_kms_key::DisassociateKmsKeyError,
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
        crate::operation::disassociate_kms_key::DisassociateKmsKeyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_kms_key::DisassociateKmsKeyError,
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
            crate::operation::disassociate_kms_key::DisassociateKmsKey,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_kms_key::DisassociateKmsKeyError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the log group.</p>
    pub fn log_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.log_group_name(input.into());
        self
    }
    /// <p>The name of the log group.</p>
    pub fn set_log_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_log_group_name(input);
        self
    }
}
