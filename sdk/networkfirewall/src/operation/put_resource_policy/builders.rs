// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_resource_policy::_put_resource_policy_output::PutResourcePolicyOutputBuilder;

pub use crate::operation::put_resource_policy::_put_resource_policy_input::PutResourcePolicyInputBuilder;

/// Fluent builder constructing a request to `PutResourcePolicy`.
///
/// <p>Creates or updates an IAM policy for your rule group or firewall policy. Use this to share rule groups and firewall policies between accounts. This operation works in conjunction with the Amazon Web Services Resource Access Manager (RAM) service to manage resource sharing for Network Firewall. </p>
/// <p>Use this operation to create or update a resource policy for your rule group or firewall policy. In the policy, you specify the accounts that you want to share the resource with and the operations that you want the accounts to be able to perform. </p>
/// <p>When you add an account in the resource policy, you then run the following Resource Access Manager (RAM) operations to access and accept the shared rule group or firewall policy. </p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_GetResourceShareInvitations.html">GetResourceShareInvitations</a> - Returns the Amazon Resource Names (ARNs) of the resource share invitations. </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/ram/latest/APIReference/API_AcceptResourceShareInvitation.html">AcceptResourceShareInvitation</a> - Accepts the share invitation for a specified resource share. </p> </li>
/// </ul>
/// <p>For additional information about resource sharing using RAM, see <a href="https://docs.aws.amazon.com/ram/latest/userguide/what-is.html">Resource Access Manager User Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutResourcePolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_resource_policy::builders::PutResourcePolicyInputBuilder,
}
impl PutResourcePolicyFluentBuilder {
    /// Creates a new `PutResourcePolicy`.
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
            crate::operation::put_resource_policy::PutResourcePolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_resource_policy::PutResourcePolicyError,
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
        crate::operation::put_resource_policy::PutResourcePolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_resource_policy::PutResourcePolicyError,
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
        crate::operation::put_resource_policy::PutResourcePolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_resource_policy::PutResourcePolicyError,
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
            crate::operation::put_resource_policy::PutResourcePolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_resource_policy::PutResourcePolicyError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (ARN) of the account that you want to share rule groups and firewall policies with.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the account that you want to share rule groups and firewall policies with.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>The IAM policy statement that lists the accounts that you want to share your rule group or firewall policy with and the operations that you want the accounts to be able to perform. </p>
    /// <p>For a rule group resource, you can specify the following operations in the Actions section of the statement:</p>
    /// <ul>
    /// <li> <p>network-firewall:CreateFirewallPolicy</p> </li>
    /// <li> <p>network-firewall:UpdateFirewallPolicy</p> </li>
    /// <li> <p>network-firewall:ListRuleGroups</p> </li>
    /// </ul>
    /// <p>For a firewall policy resource, you can specify the following operations in the Actions section of the statement:</p>
    /// <ul>
    /// <li> <p>network-firewall:AssociateFirewallPolicy</p> </li>
    /// <li> <p>network-firewall:ListFirewallPolicies</p> </li>
    /// </ul>
    /// <p>In the Resource section of the statement, you specify the ARNs for the rule groups and firewall policies that you want to share with the account that you specified in <code>Arn</code>.</p>
    pub fn policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy(input.into());
        self
    }
    /// <p>The IAM policy statement that lists the accounts that you want to share your rule group or firewall policy with and the operations that you want the accounts to be able to perform. </p>
    /// <p>For a rule group resource, you can specify the following operations in the Actions section of the statement:</p>
    /// <ul>
    /// <li> <p>network-firewall:CreateFirewallPolicy</p> </li>
    /// <li> <p>network-firewall:UpdateFirewallPolicy</p> </li>
    /// <li> <p>network-firewall:ListRuleGroups</p> </li>
    /// </ul>
    /// <p>For a firewall policy resource, you can specify the following operations in the Actions section of the statement:</p>
    /// <ul>
    /// <li> <p>network-firewall:AssociateFirewallPolicy</p> </li>
    /// <li> <p>network-firewall:ListFirewallPolicies</p> </li>
    /// </ul>
    /// <p>In the Resource section of the statement, you specify the ARNs for the rule groups and firewall policies that you want to share with the account that you specified in <code>Arn</code>.</p>
    pub fn set_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy(input);
        self
    }
}
