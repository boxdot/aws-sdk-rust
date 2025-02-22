// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::revoke_security_group_egress::_revoke_security_group_egress_output::RevokeSecurityGroupEgressOutputBuilder;

pub use crate::operation::revoke_security_group_egress::_revoke_security_group_egress_input::RevokeSecurityGroupEgressInputBuilder;

/// Fluent builder constructing a request to `RevokeSecurityGroupEgress`.
///
/// <p>[VPC only] Removes the specified outbound (egress) rules from a security group for EC2-VPC. This action does not apply to security groups for use in EC2-Classic.</p>
/// <p>You can specify rules using either rule IDs or security group rule properties. If you use rule properties, the values that you specify (for example, ports) must match the existing rule's values exactly. Each rule has a protocol, from and to ports, and destination (CIDR range, security group, or prefix list). For the TCP and UDP protocols, you must also specify the destination port or range of ports. For the ICMP protocol, you must also specify the ICMP type and code. If the security group rule has a description, you do not need to specify the description to revoke the rule.</p>
/// <p>[Default VPC] If the values you specify do not match the existing rule's values, no error is returned, and the output describes the security group rules that were not revoked.</p>
/// <p>Amazon Web Services recommends that you describe the security group to verify that the rules were removed.</p>
/// <p>Rule changes are propagated to instances within the security group as quickly as possible. However, a small delay might occur.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RevokeSecurityGroupEgressFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressInputBuilder,
}
impl RevokeSecurityGroupEgressFluentBuilder {
    /// Creates a new `RevokeSecurityGroupEgress`.
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
            crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgress,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError,
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
        crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError,
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
        crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError,
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
            crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgress,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>The ID of the security group.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_id(input.into());
        self
    }
    /// <p>The ID of the security group.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_group_id(input);
        self
    }
    /// Appends an item to `IpPermissions`.
    ///
    /// To override the contents of this collection use [`set_ip_permissions`](Self::set_ip_permissions).
    ///
    /// <p>The sets of IP permissions. You can't specify a destination security group and a CIDR IP address range in the same set of permissions.</p>
    pub fn ip_permissions(mut self, input: crate::types::IpPermission) -> Self {
        self.inner = self.inner.ip_permissions(input);
        self
    }
    /// <p>The sets of IP permissions. You can't specify a destination security group and a CIDR IP address range in the same set of permissions.</p>
    pub fn set_ip_permissions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::IpPermission>>,
    ) -> Self {
        self.inner = self.inner.set_ip_permissions(input);
        self
    }
    /// Appends an item to `SecurityGroupRuleIds`.
    ///
    /// To override the contents of this collection use [`set_security_group_rule_ids`](Self::set_security_group_rule_ids).
    ///
    /// <p>The IDs of the security group rules.</p>
    pub fn security_group_rule_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.security_group_rule_ids(input.into());
        self
    }
    /// <p>The IDs of the security group rules.</p>
    pub fn set_security_group_rule_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_security_group_rule_ids(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the CIDR.</p>
    pub fn cidr_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cidr_ip(input.into());
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the CIDR.</p>
    pub fn set_cidr_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cidr_ip(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn from_port(mut self, input: i32) -> Self {
        self.inner = self.inner.from_port(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn set_from_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_from_port(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the protocol name or number.</p>
    pub fn ip_protocol(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ip_protocol(input.into());
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the protocol name or number.</p>
    pub fn set_ip_protocol(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ip_protocol(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn to_port(mut self, input: i32) -> Self {
        self.inner = self.inner.to_port(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn set_to_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_to_port(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn source_security_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.source_security_group_name(input.into());
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn set_source_security_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_source_security_group_name(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn source_security_group_owner_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.source_security_group_owner_id(input.into());
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn set_source_security_group_owner_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_source_security_group_owner_id(input);
        self
    }
}
