// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_monitoring_members::_start_monitoring_members_output::StartMonitoringMembersOutputBuilder;

pub use crate::operation::start_monitoring_members::_start_monitoring_members_input::StartMonitoringMembersInputBuilder;

/// Fluent builder constructing a request to `StartMonitoringMembers`.
///
/// <p>Turns on GuardDuty monitoring of the specified member accounts. Use this operation to restart monitoring of accounts that you stopped monitoring with the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_StopMonitoringMembers.html">StopMonitoringMembers</a> operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartMonitoringMembersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_monitoring_members::builders::StartMonitoringMembersInputBuilder,
}
impl StartMonitoringMembersFluentBuilder {
    /// Creates a new `StartMonitoringMembers`.
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
            crate::operation::start_monitoring_members::StartMonitoringMembers,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_monitoring_members::StartMonitoringMembersError,
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
        crate::operation::start_monitoring_members::StartMonitoringMembersOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_monitoring_members::StartMonitoringMembersError,
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
        crate::operation::start_monitoring_members::StartMonitoringMembersOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_monitoring_members::StartMonitoringMembersError,
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
            crate::operation::start_monitoring_members::StartMonitoringMembers,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_monitoring_members::StartMonitoringMembersError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique ID of the detector of the GuardDuty administrator account associated with the member accounts to monitor.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The unique ID of the detector of the GuardDuty administrator account associated with the member accounts to monitor.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// Appends an item to `AccountIds`.
    ///
    /// To override the contents of this collection use [`set_account_ids`](Self::set_account_ids).
    ///
    /// <p>A list of account IDs of the GuardDuty member accounts to start monitoring.</p>
    pub fn account_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_ids(input.into());
        self
    }
    /// <p>A list of account IDs of the GuardDuty member accounts to start monitoring.</p>
    pub fn set_account_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_account_ids(input);
        self
    }
}
