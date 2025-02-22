// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_pipeline_change_progress::_get_pipeline_change_progress_output::GetPipelineChangeProgressOutputBuilder;

pub use crate::operation::get_pipeline_change_progress::_get_pipeline_change_progress_input::GetPipelineChangeProgressInputBuilder;

/// Fluent builder constructing a request to `GetPipelineChangeProgress`.
///
/// <p>Returns progress information for the current change happening on an OpenSearch Ingestion pipeline. Currently, this operation only returns information when a pipeline is being created.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/creating-pipeline.html#get-pipeline-progress">Tracking the status of pipeline creation</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetPipelineChangeProgressFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::get_pipeline_change_progress::builders::GetPipelineChangeProgressInputBuilder,
}
impl GetPipelineChangeProgressFluentBuilder {
    /// Creates a new `GetPipelineChangeProgress`.
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
            crate::operation::get_pipeline_change_progress::GetPipelineChangeProgress,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_pipeline_change_progress::GetPipelineChangeProgressError,
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
        crate::operation::get_pipeline_change_progress::GetPipelineChangeProgressOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_pipeline_change_progress::GetPipelineChangeProgressError,
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
        crate::operation::get_pipeline_change_progress::GetPipelineChangeProgressOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_pipeline_change_progress::GetPipelineChangeProgressError,
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
            crate::operation::get_pipeline_change_progress::GetPipelineChangeProgress,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_pipeline_change_progress::GetPipelineChangeProgressError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the pipeline.</p>
    pub fn pipeline_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.pipeline_name(input.into());
        self
    }
    /// <p>The name of the pipeline.</p>
    pub fn set_pipeline_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_pipeline_name(input);
        self
    }
}
