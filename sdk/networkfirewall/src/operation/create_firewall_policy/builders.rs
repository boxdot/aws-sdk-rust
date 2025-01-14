// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_firewall_policy::_create_firewall_policy_output::CreateFirewallPolicyOutputBuilder;

pub use crate::operation::create_firewall_policy::_create_firewall_policy_input::CreateFirewallPolicyInputBuilder;

/// Fluent builder constructing a request to `CreateFirewallPolicy`.
///
/// <p>Creates the firewall policy for the firewall according to the specifications. </p>
/// <p>An Network Firewall firewall policy defines the behavior of a firewall, in a collection of stateless and stateful rule groups and other settings. You can use one firewall policy for multiple firewalls. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateFirewallPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_firewall_policy::builders::CreateFirewallPolicyInputBuilder,
}
impl CreateFirewallPolicyFluentBuilder {
    /// Creates a new `CreateFirewallPolicy`.
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
            crate::operation::create_firewall_policy::CreateFirewallPolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_firewall_policy::CreateFirewallPolicyError,
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
        crate::operation::create_firewall_policy::CreateFirewallPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_firewall_policy::CreateFirewallPolicyError,
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
        crate::operation::create_firewall_policy::CreateFirewallPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_firewall_policy::CreateFirewallPolicyError,
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
            crate::operation::create_firewall_policy::CreateFirewallPolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_firewall_policy::CreateFirewallPolicyError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The descriptive name of the firewall policy. You can't change the name of a firewall policy after you create it.</p>
    pub fn firewall_policy_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.firewall_policy_name(input.into());
        self
    }
    /// <p>The descriptive name of the firewall policy. You can't change the name of a firewall policy after you create it.</p>
    pub fn set_firewall_policy_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_firewall_policy_name(input);
        self
    }
    /// <p>The rule groups and policy actions to use in the firewall policy.</p>
    pub fn firewall_policy(mut self, input: crate::types::FirewallPolicy) -> Self {
        self.inner = self.inner.firewall_policy(input);
        self
    }
    /// <p>The rule groups and policy actions to use in the firewall policy.</p>
    pub fn set_firewall_policy(
        mut self,
        input: ::std::option::Option<crate::types::FirewallPolicy>,
    ) -> Self {
        self.inner = self.inner.set_firewall_policy(input);
        self
    }
    /// <p>A description of the firewall policy.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the firewall policy.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The key:value pairs to associate with the resource.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The key:value pairs to associate with the resource.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Indicates whether you want Network Firewall to just check the validity of the request, rather than run the request. </p>
    /// <p>If set to <code>TRUE</code>, Network Firewall checks whether the request can run successfully, but doesn't actually make the requested changes. The call returns the value that the request would return if you ran it with dry run set to <code>FALSE</code>, but doesn't make additions or changes to your resources. This option allows you to make sure that you have the required permissions to run the request and that your request parameters are valid. </p>
    /// <p>If set to <code>FALSE</code>, Network Firewall makes the requested changes to your resources. </p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Indicates whether you want Network Firewall to just check the validity of the request, rather than run the request. </p>
    /// <p>If set to <code>TRUE</code>, Network Firewall checks whether the request can run successfully, but doesn't actually make the requested changes. The call returns the value that the request would return if you ran it with dry run set to <code>FALSE</code>, but doesn't make additions or changes to your resources. This option allows you to make sure that you have the required permissions to run the request and that your request parameters are valid. </p>
    /// <p>If set to <code>FALSE</code>, Network Firewall makes the requested changes to your resources. </p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>A complex type that contains settings for encryption of your firewall policy resources.</p>
    pub fn encryption_configuration(
        mut self,
        input: crate::types::EncryptionConfiguration,
    ) -> Self {
        self.inner = self.inner.encryption_configuration(input);
        self
    }
    /// <p>A complex type that contains settings for encryption of your firewall policy resources.</p>
    pub fn set_encryption_configuration(
        mut self,
        input: ::std::option::Option<crate::types::EncryptionConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_encryption_configuration(input);
        self
    }
}
