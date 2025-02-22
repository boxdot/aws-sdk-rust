// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBucketReplicationInput {
    /// <p>The bucket name for which to get the replication information.</p>
    #[doc(hidden)]
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    #[doc(hidden)]
    pub expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl GetBucketReplicationInput {
    /// <p>The bucket name for which to get the replication information.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> ::std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl GetBucketReplicationInput {
    /// Creates a new builder-style object to manufacture [`GetBucketReplicationInput`](crate::operation::get_bucket_replication::GetBucketReplicationInput).
    pub fn builder(
    ) -> crate::operation::get_bucket_replication::builders::GetBucketReplicationInputBuilder {
        crate::operation::get_bucket_replication::builders::GetBucketReplicationInputBuilder::default()
    }
}

/// A builder for [`GetBucketReplicationInput`](crate::operation::get_bucket_replication::GetBucketReplicationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetBucketReplicationInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl GetBucketReplicationInputBuilder {
    /// <p>The bucket name for which to get the replication information.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The bucket name for which to get the replication information.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.expected_bucket_owner = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.expected_bucket_owner = input;
        self
    }
    /// Consumes the builder and constructs a [`GetBucketReplicationInput`](crate::operation::get_bucket_replication::GetBucketReplicationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_bucket_replication::GetBucketReplicationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_bucket_replication::GetBucketReplicationInput {
                bucket: self.bucket,
                expected_bucket_owner: self.expected_bucket_owner,
            },
        )
    }
}
