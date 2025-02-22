// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_scheduled_audit::_update_scheduled_audit_output::UpdateScheduledAuditOutputBuilder;

pub use crate::operation::update_scheduled_audit::_update_scheduled_audit_input::UpdateScheduledAuditInputBuilder;

/// Fluent builder constructing a request to `UpdateScheduledAudit`.
///
/// <p>Updates a scheduled audit, including which checks are performed and how often the audit takes place.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">UpdateScheduledAudit</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateScheduledAuditFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_scheduled_audit::builders::UpdateScheduledAuditInputBuilder,
}
impl UpdateScheduledAuditFluentBuilder {
    /// Creates a new `UpdateScheduledAudit`.
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
            crate::operation::update_scheduled_audit::UpdateScheduledAudit,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_scheduled_audit::UpdateScheduledAuditError,
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
        crate::operation::update_scheduled_audit::UpdateScheduledAuditOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_scheduled_audit::UpdateScheduledAuditError,
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
        crate::operation::update_scheduled_audit::UpdateScheduledAuditOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_scheduled_audit::UpdateScheduledAuditError,
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
            crate::operation::update_scheduled_audit::UpdateScheduledAudit,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_scheduled_audit::UpdateScheduledAuditError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>How often the scheduled audit takes place, either <code>DAILY</code>, <code>WEEKLY</code>, <code>BIWEEKLY</code>, or <code>MONTHLY</code>. The start time of each audit is determined by the system.</p>
    pub fn frequency(mut self, input: crate::types::AuditFrequency) -> Self {
        self.inner = self.inner.frequency(input);
        self
    }
    /// <p>How often the scheduled audit takes place, either <code>DAILY</code>, <code>WEEKLY</code>, <code>BIWEEKLY</code>, or <code>MONTHLY</code>. The start time of each audit is determined by the system.</p>
    pub fn set_frequency(
        mut self,
        input: ::std::option::Option<crate::types::AuditFrequency>,
    ) -> Self {
        self.inner = self.inner.set_frequency(input);
        self
    }
    /// <p>The day of the month on which the scheduled audit takes place. This can be <code>1</code> through <code>31</code> or <code>LAST</code>. This field is required if the <code>frequency</code> parameter is set to <code>MONTHLY</code>. If days 29-31 are specified, and the month does not have that many days, the audit takes place on the "LAST" day of the month.</p>
    pub fn day_of_month(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.day_of_month(input.into());
        self
    }
    /// <p>The day of the month on which the scheduled audit takes place. This can be <code>1</code> through <code>31</code> or <code>LAST</code>. This field is required if the <code>frequency</code> parameter is set to <code>MONTHLY</code>. If days 29-31 are specified, and the month does not have that many days, the audit takes place on the "LAST" day of the month.</p>
    pub fn set_day_of_month(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_day_of_month(input);
        self
    }
    /// <p>The day of the week on which the scheduled audit takes place. This can be one of <code>SUN</code>, <code>MON</code>, <code>TUE</code>, <code>WED</code>, <code>THU</code>, <code>FRI</code>, or <code>SAT</code>. This field is required if the "frequency" parameter is set to <code>WEEKLY</code> or <code>BIWEEKLY</code>.</p>
    pub fn day_of_week(mut self, input: crate::types::DayOfWeek) -> Self {
        self.inner = self.inner.day_of_week(input);
        self
    }
    /// <p>The day of the week on which the scheduled audit takes place. This can be one of <code>SUN</code>, <code>MON</code>, <code>TUE</code>, <code>WED</code>, <code>THU</code>, <code>FRI</code>, or <code>SAT</code>. This field is required if the "frequency" parameter is set to <code>WEEKLY</code> or <code>BIWEEKLY</code>.</p>
    pub fn set_day_of_week(
        mut self,
        input: ::std::option::Option<crate::types::DayOfWeek>,
    ) -> Self {
        self.inner = self.inner.set_day_of_week(input);
        self
    }
    /// Appends an item to `targetCheckNames`.
    ///
    /// To override the contents of this collection use [`set_target_check_names`](Self::set_target_check_names).
    ///
    /// <p>Which checks are performed during the scheduled audit. Checks must be enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> to select which checks are enabled.)</p>
    pub fn target_check_names(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.target_check_names(input.into());
        self
    }
    /// <p>Which checks are performed during the scheduled audit. Checks must be enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> to select which checks are enabled.)</p>
    pub fn set_target_check_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_target_check_names(input);
        self
    }
    /// <p>The name of the scheduled audit. (Max. 128 chars)</p>
    pub fn scheduled_audit_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.scheduled_audit_name(input.into());
        self
    }
    /// <p>The name of the scheduled audit. (Max. 128 chars)</p>
    pub fn set_scheduled_audit_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_scheduled_audit_name(input);
        self
    }
}
