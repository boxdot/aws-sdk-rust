// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RebootNodeInput {
    /// <p>The name of the DAX cluster containing the node to be rebooted.</p>
    #[doc(hidden)]
    pub cluster_name: ::std::option::Option<::std::string::String>,
    /// <p>The system-assigned ID of the node to be rebooted.</p>
    #[doc(hidden)]
    pub node_id: ::std::option::Option<::std::string::String>,
}
impl RebootNodeInput {
    /// <p>The name of the DAX cluster containing the node to be rebooted.</p>
    pub fn cluster_name(&self) -> ::std::option::Option<&str> {
        self.cluster_name.as_deref()
    }
    /// <p>The system-assigned ID of the node to be rebooted.</p>
    pub fn node_id(&self) -> ::std::option::Option<&str> {
        self.node_id.as_deref()
    }
}
impl RebootNodeInput {
    /// Creates a new builder-style object to manufacture [`RebootNodeInput`](crate::operation::reboot_node::RebootNodeInput).
    pub fn builder() -> crate::operation::reboot_node::builders::RebootNodeInputBuilder {
        crate::operation::reboot_node::builders::RebootNodeInputBuilder::default()
    }
}

/// A builder for [`RebootNodeInput`](crate::operation::reboot_node::RebootNodeInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RebootNodeInputBuilder {
    pub(crate) cluster_name: ::std::option::Option<::std::string::String>,
    pub(crate) node_id: ::std::option::Option<::std::string::String>,
}
impl RebootNodeInputBuilder {
    /// <p>The name of the DAX cluster containing the node to be rebooted.</p>
    pub fn cluster_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cluster_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the DAX cluster containing the node to be rebooted.</p>
    pub fn set_cluster_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cluster_name = input;
        self
    }
    /// <p>The system-assigned ID of the node to be rebooted.</p>
    pub fn node_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.node_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The system-assigned ID of the node to be rebooted.</p>
    pub fn set_node_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.node_id = input;
        self
    }
    /// Consumes the builder and constructs a [`RebootNodeInput`](crate::operation::reboot_node::RebootNodeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::reboot_node::RebootNodeInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::reboot_node::RebootNodeInput {
            cluster_name: self.cluster_name,
            node_id: self.node_id,
        })
    }
}
