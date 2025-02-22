// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_maintenance_window_execution::_cancel_maintenance_window_execution_output::CancelMaintenanceWindowExecutionOutputBuilder;

pub use crate::operation::cancel_maintenance_window_execution::_cancel_maintenance_window_execution_input::CancelMaintenanceWindowExecutionInputBuilder;

/// Fluent builder constructing a request to `CancelMaintenanceWindowExecution`.
///
/// <p>Stops a maintenance window execution that is already in progress and cancels any tasks in the window that haven't already starting running. Tasks already in progress will continue to completion.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CancelMaintenanceWindowExecutionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::cancel_maintenance_window_execution::builders::CancelMaintenanceWindowExecutionInputBuilder,
}
impl CancelMaintenanceWindowExecutionFluentBuilder {
    /// Creates a new `CancelMaintenanceWindowExecution`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::cancel_maintenance_window_execution::CancelMaintenanceWindowExecution, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::cancel_maintenance_window_execution::CancelMaintenanceWindowExecutionError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::cancel_maintenance_window_execution::CancelMaintenanceWindowExecutionOutput, ::aws_smithy_http::result::SdkError<crate::operation::cancel_maintenance_window_execution::CancelMaintenanceWindowExecutionError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::cancel_maintenance_window_execution::CancelMaintenanceWindowExecutionOutput, ::aws_smithy_http::result::SdkError<crate::operation::cancel_maintenance_window_execution::CancelMaintenanceWindowExecutionError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::cancel_maintenance_window_execution::CancelMaintenanceWindowExecution, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::cancel_maintenance_window_execution::CancelMaintenanceWindowExecutionError>
    >{
        self.customize_middleware().await
    }
    /// <p>The ID of the maintenance window execution to stop.</p>
    pub fn window_execution_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.window_execution_id(input.into());
        self
    }
    /// <p>The ID of the maintenance window execution to stop.</p>
    pub fn set_window_execution_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_window_execution_id(input);
        self
    }
}
