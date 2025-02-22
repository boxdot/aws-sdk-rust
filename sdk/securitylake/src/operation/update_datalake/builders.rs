// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_datalake::_update_datalake_output::UpdateDatalakeOutputBuilder;

pub use crate::operation::update_datalake::_update_datalake_input::UpdateDatalakeInputBuilder;

/// Fluent builder constructing a request to `UpdateDatalake`.
///
/// <p>Specifies where to store your security data and for how long. You can add a rollup Region to consolidate data from multiple Amazon Web Services Regions. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDatalakeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_datalake::builders::UpdateDatalakeInputBuilder,
}
impl UpdateDatalakeFluentBuilder {
    /// Creates a new `UpdateDatalake`.
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
            crate::operation::update_datalake::UpdateDatalake,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_datalake::UpdateDatalakeError>,
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
        crate::operation::update_datalake::UpdateDatalakeOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_datalake::UpdateDatalakeError>,
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
        crate::operation::update_datalake::UpdateDatalakeOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_datalake::UpdateDatalakeError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_datalake::UpdateDatalake,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_datalake::UpdateDatalakeError>,
    > {
        self.customize_middleware().await
    }
    /// Adds a key-value pair to `configurations`.
    ///
    /// To override the contents of this collection use [`set_configurations`](Self::set_configurations).
    ///
    /// <p>Specify the Region or Regions that will contribute data to the rollup region.</p>
    pub fn configurations(
        mut self,
        k: crate::types::Region,
        v: crate::types::LakeConfigurationRequest,
    ) -> Self {
        self.inner = self.inner.configurations(k, v);
        self
    }
    /// <p>Specify the Region or Regions that will contribute data to the rollup region.</p>
    pub fn set_configurations(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<
                crate::types::Region,
                crate::types::LakeConfigurationRequest,
            >,
        >,
    ) -> Self {
        self.inner = self.inner.set_configurations(input);
        self
    }
}
