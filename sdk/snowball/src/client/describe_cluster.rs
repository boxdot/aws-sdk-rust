// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeCluster`](crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_id(impl ::std::convert::Into<String>)`](crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder::cluster_id) / [`set_cluster_id(Option<String>)`](crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder::set_cluster_id): <p>The automatically generated ID for a cluster.</p>
    /// - On success, responds with [`DescribeClusterOutput`](crate::operation::describe_cluster::DescribeClusterOutput) with field(s):
    ///   - [`cluster_metadata(Option<ClusterMetadata>)`](crate::operation::describe_cluster::DescribeClusterOutput::cluster_metadata): <p>Information about a specific cluster, including shipping information, cluster status, and other important metadata.</p>
    /// - On failure, responds with [`SdkError<DescribeClusterError>`](crate::operation::describe_cluster::DescribeClusterError)
    pub fn describe_cluster(
        &self,
    ) -> crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder {
        crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
