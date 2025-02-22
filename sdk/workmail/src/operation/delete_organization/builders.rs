// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_organization::_delete_organization_output::DeleteOrganizationOutputBuilder;

pub use crate::operation::delete_organization::_delete_organization_input::DeleteOrganizationInputBuilder;

/// Fluent builder constructing a request to `DeleteOrganization`.
///
/// <p>Deletes an WorkMail organization and all underlying AWS resources managed by WorkMail as part of the organization. You can choose whether to delete the associated directory. For more information, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/remove_organization.html">Removing an organization</a> in the <i>WorkMail Administrator Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteOrganizationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_organization::builders::DeleteOrganizationInputBuilder,
}
impl DeleteOrganizationFluentBuilder {
    /// Creates a new `DeleteOrganization`.
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
            crate::operation::delete_organization::DeleteOrganization,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_organization::DeleteOrganizationError,
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
        crate::operation::delete_organization::DeleteOrganizationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_organization::DeleteOrganizationError,
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
        crate::operation::delete_organization::DeleteOrganizationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_organization::DeleteOrganizationError,
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
            crate::operation::delete_organization::DeleteOrganization,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_organization::DeleteOrganizationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The idempotency token associated with the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The idempotency token associated with the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The organization ID.</p>
    pub fn organization_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>The organization ID.</p>
    pub fn set_organization_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>If true, deletes the AWS Directory Service directory associated with the organization.</p>
    pub fn delete_directory(mut self, input: bool) -> Self {
        self.inner = self.inner.delete_directory(input);
        self
    }
    /// <p>If true, deletes the AWS Directory Service directory associated with the organization.</p>
    pub fn set_delete_directory(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_delete_directory(input);
        self
    }
}
