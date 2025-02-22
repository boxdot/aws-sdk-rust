// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the auto scaling settings of a replica that will be modified.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReplicaAutoScalingUpdate {
    /// <p>The Region where the replica exists.</p>
    #[doc(hidden)]
    pub region_name: ::std::option::Option<::std::string::String>,
    /// <p>Represents the auto scaling settings of global secondary indexes that will be modified.</p>
    #[doc(hidden)]
    pub replica_global_secondary_index_updates: ::std::option::Option<
        ::std::vec::Vec<crate::types::ReplicaGlobalSecondaryIndexAutoScalingUpdate>,
    >,
    /// <p>Represents the auto scaling settings to be modified for a global table or global secondary index.</p>
    #[doc(hidden)]
    pub replica_provisioned_read_capacity_auto_scaling_update:
        ::std::option::Option<crate::types::AutoScalingSettingsUpdate>,
}
impl ReplicaAutoScalingUpdate {
    /// <p>The Region where the replica exists.</p>
    pub fn region_name(&self) -> ::std::option::Option<&str> {
        self.region_name.as_deref()
    }
    /// <p>Represents the auto scaling settings of global secondary indexes that will be modified.</p>
    pub fn replica_global_secondary_index_updates(
        &self,
    ) -> ::std::option::Option<&[crate::types::ReplicaGlobalSecondaryIndexAutoScalingUpdate]> {
        self.replica_global_secondary_index_updates.as_deref()
    }
    /// <p>Represents the auto scaling settings to be modified for a global table or global secondary index.</p>
    pub fn replica_provisioned_read_capacity_auto_scaling_update(
        &self,
    ) -> ::std::option::Option<&crate::types::AutoScalingSettingsUpdate> {
        self.replica_provisioned_read_capacity_auto_scaling_update
            .as_ref()
    }
}
impl ReplicaAutoScalingUpdate {
    /// Creates a new builder-style object to manufacture [`ReplicaAutoScalingUpdate`](crate::types::ReplicaAutoScalingUpdate).
    pub fn builder() -> crate::types::builders::ReplicaAutoScalingUpdateBuilder {
        crate::types::builders::ReplicaAutoScalingUpdateBuilder::default()
    }
}

/// A builder for [`ReplicaAutoScalingUpdate`](crate::types::ReplicaAutoScalingUpdate).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReplicaAutoScalingUpdateBuilder {
    pub(crate) region_name: ::std::option::Option<::std::string::String>,
    pub(crate) replica_global_secondary_index_updates: ::std::option::Option<
        ::std::vec::Vec<crate::types::ReplicaGlobalSecondaryIndexAutoScalingUpdate>,
    >,
    pub(crate) replica_provisioned_read_capacity_auto_scaling_update:
        ::std::option::Option<crate::types::AutoScalingSettingsUpdate>,
}
impl ReplicaAutoScalingUpdateBuilder {
    /// <p>The Region where the replica exists.</p>
    pub fn region_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.region_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Region where the replica exists.</p>
    pub fn set_region_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.region_name = input;
        self
    }
    /// Appends an item to `replica_global_secondary_index_updates`.
    ///
    /// To override the contents of this collection use [`set_replica_global_secondary_index_updates`](Self::set_replica_global_secondary_index_updates).
    ///
    /// <p>Represents the auto scaling settings of global secondary indexes that will be modified.</p>
    pub fn replica_global_secondary_index_updates(
        mut self,
        input: crate::types::ReplicaGlobalSecondaryIndexAutoScalingUpdate,
    ) -> Self {
        let mut v = self
            .replica_global_secondary_index_updates
            .unwrap_or_default();
        v.push(input);
        self.replica_global_secondary_index_updates = ::std::option::Option::Some(v);
        self
    }
    /// <p>Represents the auto scaling settings of global secondary indexes that will be modified.</p>
    pub fn set_replica_global_secondary_index_updates(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::ReplicaGlobalSecondaryIndexAutoScalingUpdate>,
        >,
    ) -> Self {
        self.replica_global_secondary_index_updates = input;
        self
    }
    /// <p>Represents the auto scaling settings to be modified for a global table or global secondary index.</p>
    pub fn replica_provisioned_read_capacity_auto_scaling_update(
        mut self,
        input: crate::types::AutoScalingSettingsUpdate,
    ) -> Self {
        self.replica_provisioned_read_capacity_auto_scaling_update =
            ::std::option::Option::Some(input);
        self
    }
    /// <p>Represents the auto scaling settings to be modified for a global table or global secondary index.</p>
    pub fn set_replica_provisioned_read_capacity_auto_scaling_update(
        mut self,
        input: ::std::option::Option<crate::types::AutoScalingSettingsUpdate>,
    ) -> Self {
        self.replica_provisioned_read_capacity_auto_scaling_update = input;
        self
    }
    /// Consumes the builder and constructs a [`ReplicaAutoScalingUpdate`](crate::types::ReplicaAutoScalingUpdate).
    pub fn build(self) -> crate::types::ReplicaAutoScalingUpdate {
        crate::types::ReplicaAutoScalingUpdate {
            region_name: self.region_name,
            replica_global_secondary_index_updates: self.replica_global_secondary_index_updates,
            replica_provisioned_read_capacity_auto_scaling_update: self
                .replica_provisioned_read_capacity_auto_scaling_update,
        }
    }
}
