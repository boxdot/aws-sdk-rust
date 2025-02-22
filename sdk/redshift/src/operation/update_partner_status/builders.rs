// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_partner_status::_update_partner_status_output::UpdatePartnerStatusOutputBuilder;

pub use crate::operation::update_partner_status::_update_partner_status_input::UpdatePartnerStatusInputBuilder;

/// Fluent builder constructing a request to `UpdatePartnerStatus`.
///
/// <p>Updates the status of a partner integration.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdatePartnerStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_partner_status::builders::UpdatePartnerStatusInputBuilder,
}
impl UpdatePartnerStatusFluentBuilder {
    /// Creates a new `UpdatePartnerStatus`.
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
            crate::operation::update_partner_status::UpdatePartnerStatus,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_partner_status::UpdatePartnerStatusError,
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
        crate::operation::update_partner_status::UpdatePartnerStatusOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_partner_status::UpdatePartnerStatusError,
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
        crate::operation::update_partner_status::UpdatePartnerStatusOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_partner_status::UpdatePartnerStatusError,
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
            crate::operation::update_partner_status::UpdatePartnerStatus,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_partner_status::UpdatePartnerStatusError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Web Services account ID that owns the cluster.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID that owns the cluster.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The cluster identifier of the cluster whose partner integration status is being updated.</p>
    pub fn cluster_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.cluster_identifier(input.into());
        self
    }
    /// <p>The cluster identifier of the cluster whose partner integration status is being updated.</p>
    pub fn set_cluster_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_cluster_identifier(input);
        self
    }
    /// <p>The name of the database whose partner integration status is being updated.</p>
    pub fn database_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.database_name(input.into());
        self
    }
    /// <p>The name of the database whose partner integration status is being updated.</p>
    pub fn set_database_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_database_name(input);
        self
    }
    /// <p>The name of the partner whose integration status is being updated.</p>
    pub fn partner_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.partner_name(input.into());
        self
    }
    /// <p>The name of the partner whose integration status is being updated.</p>
    pub fn set_partner_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_partner_name(input);
        self
    }
    /// <p>The value of the updated status.</p>
    pub fn status(mut self, input: crate::types::PartnerIntegrationStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>The value of the updated status.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::PartnerIntegrationStatus>,
    ) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>The status message provided by the partner.</p>
    pub fn status_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.status_message(input.into());
        self
    }
    /// <p>The status message provided by the partner.</p>
    pub fn set_status_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_status_message(input);
        self
    }
}
