// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeNodeOutput {
    /// <p>The node's ID.</p>
    #[doc(hidden)]
    pub node_id: ::std::option::Option<::std::string::String>,
    /// <p>The node's name.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The node's category.</p>
    #[doc(hidden)]
    pub category: ::std::option::Option<crate::types::NodeCategory>,
    /// <p>The account ID of the node's owner.</p>
    #[doc(hidden)]
    pub owner_account: ::std::option::Option<::std::string::String>,
    /// <p>The node's package name.</p>
    #[doc(hidden)]
    pub package_name: ::std::option::Option<::std::string::String>,
    /// <p>The node's package ID.</p>
    #[doc(hidden)]
    pub package_id: ::std::option::Option<::std::string::String>,
    /// <p>The node's ARN.</p>
    #[doc(hidden)]
    pub package_arn: ::std::option::Option<::std::string::String>,
    /// <p>The node's package version.</p>
    #[doc(hidden)]
    pub package_version: ::std::option::Option<::std::string::String>,
    /// <p>The node's patch version.</p>
    #[doc(hidden)]
    pub patch_version: ::std::option::Option<::std::string::String>,
    /// <p>The node's interface.</p>
    #[doc(hidden)]
    pub node_interface: ::std::option::Option<crate::types::NodeInterface>,
    /// <p>The node's asset name.</p>
    #[doc(hidden)]
    pub asset_name: ::std::option::Option<::std::string::String>,
    /// <p>The node's description.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>When the node was created.</p>
    #[doc(hidden)]
    pub created_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>When the node was updated.</p>
    #[doc(hidden)]
    pub last_updated_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl DescribeNodeOutput {
    /// <p>The node's ID.</p>
    pub fn node_id(&self) -> ::std::option::Option<&str> {
        self.node_id.as_deref()
    }
    /// <p>The node's name.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The node's category.</p>
    pub fn category(&self) -> ::std::option::Option<&crate::types::NodeCategory> {
        self.category.as_ref()
    }
    /// <p>The account ID of the node's owner.</p>
    pub fn owner_account(&self) -> ::std::option::Option<&str> {
        self.owner_account.as_deref()
    }
    /// <p>The node's package name.</p>
    pub fn package_name(&self) -> ::std::option::Option<&str> {
        self.package_name.as_deref()
    }
    /// <p>The node's package ID.</p>
    pub fn package_id(&self) -> ::std::option::Option<&str> {
        self.package_id.as_deref()
    }
    /// <p>The node's ARN.</p>
    pub fn package_arn(&self) -> ::std::option::Option<&str> {
        self.package_arn.as_deref()
    }
    /// <p>The node's package version.</p>
    pub fn package_version(&self) -> ::std::option::Option<&str> {
        self.package_version.as_deref()
    }
    /// <p>The node's patch version.</p>
    pub fn patch_version(&self) -> ::std::option::Option<&str> {
        self.patch_version.as_deref()
    }
    /// <p>The node's interface.</p>
    pub fn node_interface(&self) -> ::std::option::Option<&crate::types::NodeInterface> {
        self.node_interface.as_ref()
    }
    /// <p>The node's asset name.</p>
    pub fn asset_name(&self) -> ::std::option::Option<&str> {
        self.asset_name.as_deref()
    }
    /// <p>The node's description.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>When the node was created.</p>
    pub fn created_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_time.as_ref()
    }
    /// <p>When the node was updated.</p>
    pub fn last_updated_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated_time.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeNodeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeNodeOutput {
    /// Creates a new builder-style object to manufacture [`DescribeNodeOutput`](crate::operation::describe_node::DescribeNodeOutput).
    pub fn builder() -> crate::operation::describe_node::builders::DescribeNodeOutputBuilder {
        crate::operation::describe_node::builders::DescribeNodeOutputBuilder::default()
    }
}

/// A builder for [`DescribeNodeOutput`](crate::operation::describe_node::DescribeNodeOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeNodeOutputBuilder {
    pub(crate) node_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) category: ::std::option::Option<crate::types::NodeCategory>,
    pub(crate) owner_account: ::std::option::Option<::std::string::String>,
    pub(crate) package_name: ::std::option::Option<::std::string::String>,
    pub(crate) package_id: ::std::option::Option<::std::string::String>,
    pub(crate) package_arn: ::std::option::Option<::std::string::String>,
    pub(crate) package_version: ::std::option::Option<::std::string::String>,
    pub(crate) patch_version: ::std::option::Option<::std::string::String>,
    pub(crate) node_interface: ::std::option::Option<crate::types::NodeInterface>,
    pub(crate) asset_name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) created_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_updated_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl DescribeNodeOutputBuilder {
    /// <p>The node's ID.</p>
    pub fn node_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.node_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The node's ID.</p>
    pub fn set_node_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.node_id = input;
        self
    }
    /// <p>The node's name.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The node's name.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The node's category.</p>
    pub fn category(mut self, input: crate::types::NodeCategory) -> Self {
        self.category = ::std::option::Option::Some(input);
        self
    }
    /// <p>The node's category.</p>
    pub fn set_category(
        mut self,
        input: ::std::option::Option<crate::types::NodeCategory>,
    ) -> Self {
        self.category = input;
        self
    }
    /// <p>The account ID of the node's owner.</p>
    pub fn owner_account(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.owner_account = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The account ID of the node's owner.</p>
    pub fn set_owner_account(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.owner_account = input;
        self
    }
    /// <p>The node's package name.</p>
    pub fn package_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.package_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The node's package name.</p>
    pub fn set_package_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.package_name = input;
        self
    }
    /// <p>The node's package ID.</p>
    pub fn package_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.package_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The node's package ID.</p>
    pub fn set_package_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.package_id = input;
        self
    }
    /// <p>The node's ARN.</p>
    pub fn package_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.package_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The node's ARN.</p>
    pub fn set_package_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.package_arn = input;
        self
    }
    /// <p>The node's package version.</p>
    pub fn package_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.package_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The node's package version.</p>
    pub fn set_package_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.package_version = input;
        self
    }
    /// <p>The node's patch version.</p>
    pub fn patch_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.patch_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The node's patch version.</p>
    pub fn set_patch_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.patch_version = input;
        self
    }
    /// <p>The node's interface.</p>
    pub fn node_interface(mut self, input: crate::types::NodeInterface) -> Self {
        self.node_interface = ::std::option::Option::Some(input);
        self
    }
    /// <p>The node's interface.</p>
    pub fn set_node_interface(
        mut self,
        input: ::std::option::Option<crate::types::NodeInterface>,
    ) -> Self {
        self.node_interface = input;
        self
    }
    /// <p>The node's asset name.</p>
    pub fn asset_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.asset_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The node's asset name.</p>
    pub fn set_asset_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.asset_name = input;
        self
    }
    /// <p>The node's description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The node's description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>When the node was created.</p>
    pub fn created_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>When the node was created.</p>
    pub fn set_created_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_time = input;
        self
    }
    /// <p>When the node was updated.</p>
    pub fn last_updated_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>When the node was updated.</p>
    pub fn set_last_updated_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_updated_time = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeNodeOutput`](crate::operation::describe_node::DescribeNodeOutput).
    pub fn build(self) -> crate::operation::describe_node::DescribeNodeOutput {
        crate::operation::describe_node::DescribeNodeOutput {
            node_id: self.node_id,
            name: self.name,
            category: self.category,
            owner_account: self.owner_account,
            package_name: self.package_name,
            package_id: self.package_id,
            package_arn: self.package_arn,
            package_version: self.package_version,
            patch_version: self.patch_version,
            node_interface: self.node_interface,
            asset_name: self.asset_name,
            description: self.description,
            created_time: self.created_time,
            last_updated_time: self.last_updated_time,
            _request_id: self._request_id,
        }
    }
}
