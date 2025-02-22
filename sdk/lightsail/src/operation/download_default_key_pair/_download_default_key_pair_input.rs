// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DownloadDefaultKeyPairInput {}
impl DownloadDefaultKeyPairInput {
    /// Creates a new builder-style object to manufacture [`DownloadDefaultKeyPairInput`](crate::operation::download_default_key_pair::DownloadDefaultKeyPairInput).
    pub fn builder(
    ) -> crate::operation::download_default_key_pair::builders::DownloadDefaultKeyPairInputBuilder
    {
        crate::operation::download_default_key_pair::builders::DownloadDefaultKeyPairInputBuilder::default()
    }
}

/// A builder for [`DownloadDefaultKeyPairInput`](crate::operation::download_default_key_pair::DownloadDefaultKeyPairInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DownloadDefaultKeyPairInputBuilder {}
impl DownloadDefaultKeyPairInputBuilder {
    /// Consumes the builder and constructs a [`DownloadDefaultKeyPairInput`](crate::operation::download_default_key_pair::DownloadDefaultKeyPairInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::download_default_key_pair::DownloadDefaultKeyPairInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::download_default_key_pair::DownloadDefaultKeyPairInput {},
        )
    }
}
