// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_hosts::_modify_hosts_output::ModifyHostsOutputBuilder;

pub use crate::operation::modify_hosts::_modify_hosts_input::ModifyHostsInputBuilder;

/// Fluent builder constructing a request to `ModifyHosts`.
///
/// <p>Modify the auto-placement setting of a Dedicated Host. When auto-placement is enabled, any instances that you launch with a tenancy of <code>host</code> but without a specific host ID are placed onto any available Dedicated Host in your account that has auto-placement enabled. When auto-placement is disabled, you need to provide a host ID to have the instance launch onto a specific host. If no host ID is provided, the instance is launched onto a suitable host with auto-placement enabled.</p>
/// <p>You can also use this API action to modify a Dedicated Host to support either multiple instance types in an instance family, or to support a specific instance type only.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyHostsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_hosts::builders::ModifyHostsInputBuilder,
}
impl ModifyHostsFluentBuilder {
    /// Creates a new `ModifyHosts`.
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
            crate::operation::modify_hosts::ModifyHosts,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_hosts::ModifyHostsError>,
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
        crate::operation::modify_hosts::ModifyHostsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_hosts::ModifyHostsError>,
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
        crate::operation::modify_hosts::ModifyHostsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_hosts::ModifyHostsError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::modify_hosts::ModifyHosts,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_hosts::ModifyHostsError>,
    > {
        self.customize_middleware().await
    }
    /// <p>Specify whether to enable or disable auto-placement.</p>
    pub fn auto_placement(mut self, input: crate::types::AutoPlacement) -> Self {
        self.inner = self.inner.auto_placement(input);
        self
    }
    /// <p>Specify whether to enable or disable auto-placement.</p>
    pub fn set_auto_placement(
        mut self,
        input: ::std::option::Option<crate::types::AutoPlacement>,
    ) -> Self {
        self.inner = self.inner.set_auto_placement(input);
        self
    }
    /// Appends an item to `HostIds`.
    ///
    /// To override the contents of this collection use [`set_host_ids`](Self::set_host_ids).
    ///
    /// <p>The IDs of the Dedicated Hosts to modify.</p>
    pub fn host_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.host_ids(input.into());
        self
    }
    /// <p>The IDs of the Dedicated Hosts to modify.</p>
    pub fn set_host_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_host_ids(input);
        self
    }
    /// <p>Indicates whether to enable or disable host recovery for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn host_recovery(mut self, input: crate::types::HostRecovery) -> Self {
        self.inner = self.inner.host_recovery(input);
        self
    }
    /// <p>Indicates whether to enable or disable host recovery for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_host_recovery(
        mut self,
        input: ::std::option::Option<crate::types::HostRecovery>,
    ) -> Self {
        self.inner = self.inner.set_host_recovery(input);
        self
    }
    /// <p>Specifies the instance type to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support only a specific instance type.</p>
    /// <p>If you want to modify a Dedicated Host to support multiple instance types in its current instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    pub fn instance_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.instance_type(input.into());
        self
    }
    /// <p>Specifies the instance type to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support only a specific instance type.</p>
    /// <p>If you want to modify a Dedicated Host to support multiple instance types in its current instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    pub fn set_instance_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_instance_type(input);
        self
    }
    /// <p>Specifies the instance family to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support multiple instance types within its current instance family.</p>
    /// <p>If you want to modify a Dedicated Host to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
    pub fn instance_family(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.instance_family(input.into());
        self
    }
    /// <p>Specifies the instance family to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support multiple instance types within its current instance family.</p>
    /// <p>If you want to modify a Dedicated Host to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
    pub fn set_instance_family(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_instance_family(input);
        self
    }
    /// <p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html"> Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn host_maintenance(mut self, input: crate::types::HostMaintenance) -> Self {
        self.inner = self.inner.host_maintenance(input);
        self
    }
    /// <p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html"> Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_host_maintenance(
        mut self,
        input: ::std::option::Option<crate::types::HostMaintenance>,
    ) -> Self {
        self.inner = self.inner.set_host_maintenance(input);
        self
    }
}
