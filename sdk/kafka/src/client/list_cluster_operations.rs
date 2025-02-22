// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListClusterOperations`](crate::operation::list_cluster_operations::builders::ListClusterOperationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_cluster_operations::builders::ListClusterOperationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_arn(impl ::std::convert::Into<String>)`](crate::operation::list_cluster_operations::builders::ListClusterOperationsFluentBuilder::cluster_arn) / [`set_cluster_arn(Option<String>)`](crate::operation::list_cluster_operations::builders::ListClusterOperationsFluentBuilder::set_cluster_arn): <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p>
    ///   - [`max_results(i32)`](crate::operation::list_cluster_operations::builders::ListClusterOperationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_cluster_operations::builders::ListClusterOperationsFluentBuilder::set_max_results): <p>The maximum number of results to return in the response. If there are more results, the response includes a NextToken parameter.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_cluster_operations::builders::ListClusterOperationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_cluster_operations::builders::ListClusterOperationsFluentBuilder::set_next_token): <p>The paginated results marker. When the result of the operation is truncated, the call returns NextToken in the response. To get the next batch, provide this token in your next request.</p>
    /// - On success, responds with [`ListClusterOperationsOutput`](crate::operation::list_cluster_operations::ListClusterOperationsOutput) with field(s):
    ///   - [`cluster_operation_info_list(Option<Vec<ClusterOperationInfo>>)`](crate::operation::list_cluster_operations::ListClusterOperationsOutput::cluster_operation_info_list): <p>An array of cluster operation information objects.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_cluster_operations::ListClusterOperationsOutput::next_token): <p>If the response of ListClusterOperations is truncated, it returns a NextToken in the response. This Nexttoken should be sent in the subsequent request to ListClusterOperations.</p>
    /// - On failure, responds with [`SdkError<ListClusterOperationsError>`](crate::operation::list_cluster_operations::ListClusterOperationsError)
    pub fn list_cluster_operations(
        &self,
    ) -> crate::operation::list_cluster_operations::builders::ListClusterOperationsFluentBuilder
    {
        crate::operation::list_cluster_operations::builders::ListClusterOperationsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
