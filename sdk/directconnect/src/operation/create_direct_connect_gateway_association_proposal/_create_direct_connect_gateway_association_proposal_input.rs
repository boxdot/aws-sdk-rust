// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateDirectConnectGatewayAssociationProposalInput {
    /// <p>The ID of the Direct Connect gateway.</p>
    #[doc(hidden)]
    pub direct_connect_gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Amazon Web Services account that owns the Direct Connect gateway.</p>
    #[doc(hidden)]
    pub direct_connect_gateway_owner_account: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the virtual private gateway or transit gateway.</p>
    #[doc(hidden)]
    pub gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon VPC prefixes to advertise to the Direct Connect gateway.</p>
    #[doc(hidden)]
    pub add_allowed_prefixes_to_direct_connect_gateway:
        ::std::option::Option<::std::vec::Vec<crate::types::RouteFilterPrefix>>,
    /// <p>The Amazon VPC prefixes to no longer advertise to the Direct Connect gateway.</p>
    #[doc(hidden)]
    pub remove_allowed_prefixes_to_direct_connect_gateway:
        ::std::option::Option<::std::vec::Vec<crate::types::RouteFilterPrefix>>,
}
impl CreateDirectConnectGatewayAssociationProposalInput {
    /// <p>The ID of the Direct Connect gateway.</p>
    pub fn direct_connect_gateway_id(&self) -> ::std::option::Option<&str> {
        self.direct_connect_gateway_id.as_deref()
    }
    /// <p>The ID of the Amazon Web Services account that owns the Direct Connect gateway.</p>
    pub fn direct_connect_gateway_owner_account(&self) -> ::std::option::Option<&str> {
        self.direct_connect_gateway_owner_account.as_deref()
    }
    /// <p>The ID of the virtual private gateway or transit gateway.</p>
    pub fn gateway_id(&self) -> ::std::option::Option<&str> {
        self.gateway_id.as_deref()
    }
    /// <p>The Amazon VPC prefixes to advertise to the Direct Connect gateway.</p>
    pub fn add_allowed_prefixes_to_direct_connect_gateway(
        &self,
    ) -> ::std::option::Option<&[crate::types::RouteFilterPrefix]> {
        self.add_allowed_prefixes_to_direct_connect_gateway
            .as_deref()
    }
    /// <p>The Amazon VPC prefixes to no longer advertise to the Direct Connect gateway.</p>
    pub fn remove_allowed_prefixes_to_direct_connect_gateway(
        &self,
    ) -> ::std::option::Option<&[crate::types::RouteFilterPrefix]> {
        self.remove_allowed_prefixes_to_direct_connect_gateway
            .as_deref()
    }
}
impl CreateDirectConnectGatewayAssociationProposalInput {
    /// Creates a new builder-style object to manufacture [`CreateDirectConnectGatewayAssociationProposalInput`](crate::operation::create_direct_connect_gateway_association_proposal::CreateDirectConnectGatewayAssociationProposalInput).
    pub fn builder() -> crate::operation::create_direct_connect_gateway_association_proposal::builders::CreateDirectConnectGatewayAssociationProposalInputBuilder{
        crate::operation::create_direct_connect_gateway_association_proposal::builders::CreateDirectConnectGatewayAssociationProposalInputBuilder::default()
    }
}

/// A builder for [`CreateDirectConnectGatewayAssociationProposalInput`](crate::operation::create_direct_connect_gateway_association_proposal::CreateDirectConnectGatewayAssociationProposalInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateDirectConnectGatewayAssociationProposalInputBuilder {
    pub(crate) direct_connect_gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) direct_connect_gateway_owner_account: ::std::option::Option<::std::string::String>,
    pub(crate) gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) add_allowed_prefixes_to_direct_connect_gateway:
        ::std::option::Option<::std::vec::Vec<crate::types::RouteFilterPrefix>>,
    pub(crate) remove_allowed_prefixes_to_direct_connect_gateway:
        ::std::option::Option<::std::vec::Vec<crate::types::RouteFilterPrefix>>,
}
impl CreateDirectConnectGatewayAssociationProposalInputBuilder {
    /// <p>The ID of the Direct Connect gateway.</p>
    pub fn direct_connect_gateway_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.direct_connect_gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Direct Connect gateway.</p>
    pub fn set_direct_connect_gateway_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.direct_connect_gateway_id = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the Direct Connect gateway.</p>
    pub fn direct_connect_gateway_owner_account(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.direct_connect_gateway_owner_account = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the Direct Connect gateway.</p>
    pub fn set_direct_connect_gateway_owner_account(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.direct_connect_gateway_owner_account = input;
        self
    }
    /// <p>The ID of the virtual private gateway or transit gateway.</p>
    pub fn gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the virtual private gateway or transit gateway.</p>
    pub fn set_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.gateway_id = input;
        self
    }
    /// Appends an item to `add_allowed_prefixes_to_direct_connect_gateway`.
    ///
    /// To override the contents of this collection use [`set_add_allowed_prefixes_to_direct_connect_gateway`](Self::set_add_allowed_prefixes_to_direct_connect_gateway).
    ///
    /// <p>The Amazon VPC prefixes to advertise to the Direct Connect gateway.</p>
    pub fn add_allowed_prefixes_to_direct_connect_gateway(
        mut self,
        input: crate::types::RouteFilterPrefix,
    ) -> Self {
        let mut v = self
            .add_allowed_prefixes_to_direct_connect_gateway
            .unwrap_or_default();
        v.push(input);
        self.add_allowed_prefixes_to_direct_connect_gateway = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Amazon VPC prefixes to advertise to the Direct Connect gateway.</p>
    pub fn set_add_allowed_prefixes_to_direct_connect_gateway(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RouteFilterPrefix>>,
    ) -> Self {
        self.add_allowed_prefixes_to_direct_connect_gateway = input;
        self
    }
    /// Appends an item to `remove_allowed_prefixes_to_direct_connect_gateway`.
    ///
    /// To override the contents of this collection use [`set_remove_allowed_prefixes_to_direct_connect_gateway`](Self::set_remove_allowed_prefixes_to_direct_connect_gateway).
    ///
    /// <p>The Amazon VPC prefixes to no longer advertise to the Direct Connect gateway.</p>
    pub fn remove_allowed_prefixes_to_direct_connect_gateway(
        mut self,
        input: crate::types::RouteFilterPrefix,
    ) -> Self {
        let mut v = self
            .remove_allowed_prefixes_to_direct_connect_gateway
            .unwrap_or_default();
        v.push(input);
        self.remove_allowed_prefixes_to_direct_connect_gateway = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Amazon VPC prefixes to no longer advertise to the Direct Connect gateway.</p>
    pub fn set_remove_allowed_prefixes_to_direct_connect_gateway(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RouteFilterPrefix>>,
    ) -> Self {
        self.remove_allowed_prefixes_to_direct_connect_gateway = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateDirectConnectGatewayAssociationProposalInput`](crate::operation::create_direct_connect_gateway_association_proposal::CreateDirectConnectGatewayAssociationProposalInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::create_direct_connect_gateway_association_proposal::CreateDirectConnectGatewayAssociationProposalInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::create_direct_connect_gateway_association_proposal::CreateDirectConnectGatewayAssociationProposalInput {
                direct_connect_gateway_id: self.direct_connect_gateway_id
                ,
                direct_connect_gateway_owner_account: self.direct_connect_gateway_owner_account
                ,
                gateway_id: self.gateway_id
                ,
                add_allowed_prefixes_to_direct_connect_gateway: self.add_allowed_prefixes_to_direct_connect_gateway
                ,
                remove_allowed_prefixes_to_direct_connect_gateway: self.remove_allowed_prefixes_to_direct_connect_gateway
                ,
            }
        )
    }
}
