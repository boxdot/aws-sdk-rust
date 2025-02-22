// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateFromMasterAccountInput {}
impl DisassociateFromMasterAccountInput {
    /// Creates a new builder-style object to manufacture [`DisassociateFromMasterAccountInput`](crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountInput).
    pub fn builder() -> crate::operation::disassociate_from_master_account::builders::DisassociateFromMasterAccountInputBuilder{
        crate::operation::disassociate_from_master_account::builders::DisassociateFromMasterAccountInputBuilder::default()
    }
}

/// A builder for [`DisassociateFromMasterAccountInput`](crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DisassociateFromMasterAccountInputBuilder {}
impl DisassociateFromMasterAccountInputBuilder {
    /// Consumes the builder and constructs a [`DisassociateFromMasterAccountInput`](crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountInput {
            }
        )
    }
}
