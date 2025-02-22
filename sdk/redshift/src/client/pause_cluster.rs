// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PauseCluster`](crate::operation::pause_cluster::builders::PauseClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_identifier(impl ::std::convert::Into<String>)`](crate::operation::pause_cluster::builders::PauseClusterFluentBuilder::cluster_identifier) / [`set_cluster_identifier(Option<String>)`](crate::operation::pause_cluster::builders::PauseClusterFluentBuilder::set_cluster_identifier): <p>The identifier of the cluster to be paused.</p>
    /// - On success, responds with [`PauseClusterOutput`](crate::operation::pause_cluster::PauseClusterOutput) with field(s):
    ///   - [`cluster(Option<Cluster>)`](crate::operation::pause_cluster::PauseClusterOutput::cluster): <p>Describes a cluster.</p>
    /// - On failure, responds with [`SdkError<PauseClusterError>`](crate::operation::pause_cluster::PauseClusterError)
    pub fn pause_cluster(
        &self,
    ) -> crate::operation::pause_cluster::builders::PauseClusterFluentBuilder {
        crate::operation::pause_cluster::builders::PauseClusterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
