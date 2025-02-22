// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_organization::_create_organization_output::CreateOrganizationOutputBuilder;

pub use crate::operation::create_organization::_create_organization_input::CreateOrganizationInputBuilder;

/// Fluent builder constructing a request to `CreateOrganization`.
///
/// <p>Creates a new WorkMail organization. Optionally, you can choose to associate an existing AWS Directory Service directory with your organization. If an AWS Directory Service directory ID is specified, the organization alias must match the directory alias. If you choose not to associate an existing directory with your organization, then we create a new WorkMail directory for you. For more information, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/add_new_organization.html">Adding an organization</a> in the <i>WorkMail Administrator Guide</i>.</p>
/// <p>You can associate multiple email domains with an organization, then choose your default email domain from the WorkMail console. You can also associate a domain that is managed in an Amazon Route 53 public hosted zone. For more information, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/add_domain.html">Adding a domain</a> and <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/default_domain.html">Choosing the default domain</a> in the <i>WorkMail Administrator Guide</i>.</p>
/// <p>Optionally, you can use a customer managed key from AWS Key Management Service (AWS KMS) to encrypt email for your organization. If you don't associate an AWS KMS key, WorkMail creates a default, AWS managed key for you.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateOrganizationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_organization::builders::CreateOrganizationInputBuilder,
}
impl CreateOrganizationFluentBuilder {
    /// Creates a new `CreateOrganization`.
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
            crate::operation::create_organization::CreateOrganization,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_organization::CreateOrganizationError,
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
        crate::operation::create_organization::CreateOrganizationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_organization::CreateOrganizationError,
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
        crate::operation::create_organization::CreateOrganizationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_organization::CreateOrganizationError,
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
            crate::operation::create_organization::CreateOrganization,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_organization::CreateOrganizationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The AWS Directory Service directory ID.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The AWS Directory Service directory ID.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>The organization alias.</p>
    pub fn alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.alias(input.into());
        self
    }
    /// <p>The organization alias.</p>
    pub fn set_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_alias(input);
        self
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
    /// Appends an item to `Domains`.
    ///
    /// To override the contents of this collection use [`set_domains`](Self::set_domains).
    ///
    /// <p>The email domains to associate with the organization.</p>
    pub fn domains(mut self, input: crate::types::Domain) -> Self {
        self.inner = self.inner.domains(input);
        self
    }
    /// <p>The email domains to associate with the organization.</p>
    pub fn set_domains(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Domain>>,
    ) -> Self {
        self.inner = self.inner.set_domains(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a customer managed key from AWS KMS.</p>
    pub fn kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a customer managed key from AWS KMS.</p>
    pub fn set_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_arn(input);
        self
    }
    /// <p>When <code>true</code>, allows organization interoperability between WorkMail and Microsoft Exchange. If <code>true</code>, you must include a AD Connector directory ID in the request.</p>
    pub fn enable_interoperability(mut self, input: bool) -> Self {
        self.inner = self.inner.enable_interoperability(input);
        self
    }
    /// <p>When <code>true</code>, allows organization interoperability between WorkMail and Microsoft Exchange. If <code>true</code>, you must include a AD Connector directory ID in the request.</p>
    pub fn set_enable_interoperability(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enable_interoperability(input);
        self
    }
}
