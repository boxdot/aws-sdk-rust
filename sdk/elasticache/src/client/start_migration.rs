// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartMigration`](crate::operation::start_migration::builders::StartMigrationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`replication_group_id(impl ::std::convert::Into<String>)`](crate::operation::start_migration::builders::StartMigrationFluentBuilder::replication_group_id) / [`set_replication_group_id(Option<String>)`](crate::operation::start_migration::builders::StartMigrationFluentBuilder::set_replication_group_id): <p>The ID of the replication group to which data should be migrated.</p>
    ///   - [`customer_node_endpoint_list(Vec<CustomerNodeEndpoint>)`](crate::operation::start_migration::builders::StartMigrationFluentBuilder::customer_node_endpoint_list) / [`set_customer_node_endpoint_list(Option<Vec<CustomerNodeEndpoint>>)`](crate::operation::start_migration::builders::StartMigrationFluentBuilder::set_customer_node_endpoint_list): <p>List of endpoints from which data should be migrated. For Redis (cluster mode disabled), list should have only one element.</p>
    /// - On success, responds with [`StartMigrationOutput`](crate::operation::start_migration::StartMigrationOutput) with field(s):
    ///   - [`replication_group(Option<ReplicationGroup>)`](crate::operation::start_migration::StartMigrationOutput::replication_group): <p>Contains all of the attributes of a specific Redis replication group.</p>
    /// - On failure, responds with [`SdkError<StartMigrationError>`](crate::operation::start_migration::StartMigrationError)
    pub fn start_migration(
        &self,
    ) -> crate::operation::start_migration::builders::StartMigrationFluentBuilder {
        crate::operation::start_migration::builders::StartMigrationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
