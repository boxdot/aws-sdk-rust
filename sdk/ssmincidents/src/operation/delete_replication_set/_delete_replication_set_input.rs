// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteReplicationSetInput {
    /// <p>The Amazon Resource Name (ARN) of the replication set you're deleting.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
}
impl DeleteReplicationSetInput {
    /// <p>The Amazon Resource Name (ARN) of the replication set you're deleting.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
}
impl DeleteReplicationSetInput {
    /// Creates a new builder-style object to manufacture [`DeleteReplicationSetInput`](crate::operation::delete_replication_set::DeleteReplicationSetInput).
    pub fn builder(
    ) -> crate::operation::delete_replication_set::builders::DeleteReplicationSetInputBuilder {
        crate::operation::delete_replication_set::builders::DeleteReplicationSetInputBuilder::default()
    }
}

/// A builder for [`DeleteReplicationSetInput`](crate::operation::delete_replication_set::DeleteReplicationSetInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteReplicationSetInputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
}
impl DeleteReplicationSetInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the replication set you're deleting.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the replication set you're deleting.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteReplicationSetInput`](crate::operation::delete_replication_set::DeleteReplicationSetInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_replication_set::DeleteReplicationSetInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_replication_set::DeleteReplicationSetInput { arn: self.arn },
        )
    }
}
