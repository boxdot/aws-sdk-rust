// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteIpamResourceDiscovery`](crate::operation::delete_ipam_resource_discovery::builders::DeleteIpamResourceDiscoveryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::delete_ipam_resource_discovery::builders::DeleteIpamResourceDiscoveryFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_ipam_resource_discovery::builders::DeleteIpamResourceDiscoveryFluentBuilder::set_dry_run): <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`ipam_resource_discovery_id(impl ::std::convert::Into<String>)`](crate::operation::delete_ipam_resource_discovery::builders::DeleteIpamResourceDiscoveryFluentBuilder::ipam_resource_discovery_id) / [`set_ipam_resource_discovery_id(Option<String>)`](crate::operation::delete_ipam_resource_discovery::builders::DeleteIpamResourceDiscoveryFluentBuilder::set_ipam_resource_discovery_id): <p>The IPAM resource discovery ID.</p>
    /// - On success, responds with [`DeleteIpamResourceDiscoveryOutput`](crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryOutput) with field(s):
    ///   - [`ipam_resource_discovery(Option<IpamResourceDiscovery>)`](crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryOutput::ipam_resource_discovery): <p>The IPAM resource discovery.</p>
    /// - On failure, responds with [`SdkError<DeleteIpamResourceDiscoveryError>`](crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryError)
    pub fn delete_ipam_resource_discovery(&self) -> crate::operation::delete_ipam_resource_discovery::builders::DeleteIpamResourceDiscoveryFluentBuilder{
        crate::operation::delete_ipam_resource_discovery::builders::DeleteIpamResourceDiscoveryFluentBuilder::new(self.handle.clone())
    }
}
