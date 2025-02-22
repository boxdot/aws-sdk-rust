// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_modify_cluster_snapshots::_batch_modify_cluster_snapshots_output::BatchModifyClusterSnapshotsOutputBuilder;

pub use crate::operation::batch_modify_cluster_snapshots::_batch_modify_cluster_snapshots_input::BatchModifyClusterSnapshotsInputBuilder;

/// Fluent builder constructing a request to `BatchModifyClusterSnapshots`.
///
/// <p>Modifies the settings for a set of cluster snapshots.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchModifyClusterSnapshotsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::batch_modify_cluster_snapshots::builders::BatchModifyClusterSnapshotsInputBuilder,
}
impl BatchModifyClusterSnapshotsFluentBuilder {
    /// Creates a new `BatchModifyClusterSnapshots`.
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
            crate::operation::batch_modify_cluster_snapshots::BatchModifyClusterSnapshots,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_modify_cluster_snapshots::BatchModifyClusterSnapshotsError,
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
        crate::operation::batch_modify_cluster_snapshots::BatchModifyClusterSnapshotsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_modify_cluster_snapshots::BatchModifyClusterSnapshotsError,
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
        crate::operation::batch_modify_cluster_snapshots::BatchModifyClusterSnapshotsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_modify_cluster_snapshots::BatchModifyClusterSnapshotsError,
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
            crate::operation::batch_modify_cluster_snapshots::BatchModifyClusterSnapshots,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_modify_cluster_snapshots::BatchModifyClusterSnapshotsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `SnapshotIdentifierList`.
    ///
    /// To override the contents of this collection use [`set_snapshot_identifier_list`](Self::set_snapshot_identifier_list).
    ///
    /// <p>A list of snapshot identifiers you want to modify.</p>
    pub fn snapshot_identifier_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.snapshot_identifier_list(input.into());
        self
    }
    /// <p>A list of snapshot identifiers you want to modify.</p>
    pub fn set_snapshot_identifier_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_snapshot_identifier_list(input);
        self
    }
    /// <p>The number of days that a manual snapshot is retained. If you specify the value -1, the manual snapshot is retained indefinitely.</p>
    /// <p>The number must be either -1 or an integer between 1 and 3,653.</p>
    /// <p>If you decrease the manual snapshot retention period from its current value, existing manual snapshots that fall outside of the new retention period will return an error. If you want to suppress the errors and delete the snapshots, use the force option. </p>
    pub fn manual_snapshot_retention_period(mut self, input: i32) -> Self {
        self.inner = self.inner.manual_snapshot_retention_period(input);
        self
    }
    /// <p>The number of days that a manual snapshot is retained. If you specify the value -1, the manual snapshot is retained indefinitely.</p>
    /// <p>The number must be either -1 or an integer between 1 and 3,653.</p>
    /// <p>If you decrease the manual snapshot retention period from its current value, existing manual snapshots that fall outside of the new retention period will return an error. If you want to suppress the errors and delete the snapshots, use the force option. </p>
    pub fn set_manual_snapshot_retention_period(
        mut self,
        input: ::std::option::Option<i32>,
    ) -> Self {
        self.inner = self.inner.set_manual_snapshot_retention_period(input);
        self
    }
    /// <p>A boolean value indicating whether to override an exception if the retention period has passed. </p>
    pub fn force(mut self, input: bool) -> Self {
        self.inner = self.inner.force(input);
        self
    }
    /// <p>A boolean value indicating whether to override an exception if the retention period has passed. </p>
    pub fn set_force(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force(input);
        self
    }
}
