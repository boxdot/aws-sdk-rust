// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_hapg::_modify_hapg_output::ModifyHapgOutputBuilder;

pub use crate::operation::modify_hapg::_modify_hapg_input::ModifyHapgInputBuilder;

/// Fluent builder constructing a request to `ModifyHapg`.
///
/// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="https://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="https://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p>
/// <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p>
/// <p>Modifies an existing high-availability partition group.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyHapgFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_hapg::builders::ModifyHapgInputBuilder,
}
impl ModifyHapgFluentBuilder {
    /// Creates a new `ModifyHapg`.
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
            crate::operation::modify_hapg::ModifyHapg,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_hapg::ModifyHapgError>,
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
        crate::operation::modify_hapg::ModifyHapgOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_hapg::ModifyHapgError>,
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
        crate::operation::modify_hapg::ModifyHapgOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_hapg::ModifyHapgError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::modify_hapg::ModifyHapg,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_hapg::ModifyHapgError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The ARN of the high-availability partition group to modify.</p>
    pub fn hapg_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.hapg_arn(input.into());
        self
    }
    /// <p>The ARN of the high-availability partition group to modify.</p>
    pub fn set_hapg_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_hapg_arn(input);
        self
    }
    /// <p>The new label for the high-availability partition group.</p>
    pub fn label(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.label(input.into());
        self
    }
    /// <p>The new label for the high-availability partition group.</p>
    pub fn set_label(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_label(input);
        self
    }
    /// Appends an item to `PartitionSerialList`.
    ///
    /// To override the contents of this collection use [`set_partition_serial_list`](Self::set_partition_serial_list).
    ///
    /// <p>The list of partition serial numbers to make members of the high-availability partition group.</p>
    pub fn partition_serial_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.partition_serial_list(input.into());
        self
    }
    /// <p>The list of partition serial numbers to make members of the high-availability partition group.</p>
    pub fn set_partition_serial_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_partition_serial_list(input);
        self
    }
}
