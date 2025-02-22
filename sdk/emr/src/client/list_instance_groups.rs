// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListInstanceGroups`](crate::operation::list_instance_groups::builders::ListInstanceGroupsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_instance_groups::builders::ListInstanceGroupsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_id(impl ::std::convert::Into<String>)`](crate::operation::list_instance_groups::builders::ListInstanceGroupsFluentBuilder::cluster_id) / [`set_cluster_id(Option<String>)`](crate::operation::list_instance_groups::builders::ListInstanceGroupsFluentBuilder::set_cluster_id): <p>The identifier of the cluster for which to list the instance groups.</p>
    ///   - [`marker(impl ::std::convert::Into<String>)`](crate::operation::list_instance_groups::builders::ListInstanceGroupsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_instance_groups::builders::ListInstanceGroupsFluentBuilder::set_marker): <p>The pagination token that indicates the next set of results to retrieve.</p>
    /// - On success, responds with [`ListInstanceGroupsOutput`](crate::operation::list_instance_groups::ListInstanceGroupsOutput) with field(s):
    ///   - [`instance_groups(Option<Vec<InstanceGroup>>)`](crate::operation::list_instance_groups::ListInstanceGroupsOutput::instance_groups): <p>The list of instance groups for the cluster and given filters.</p>
    ///   - [`marker(Option<String>)`](crate::operation::list_instance_groups::ListInstanceGroupsOutput::marker): <p>The pagination token that indicates the next set of results to retrieve.</p>
    /// - On failure, responds with [`SdkError<ListInstanceGroupsError>`](crate::operation::list_instance_groups::ListInstanceGroupsError)
    pub fn list_instance_groups(
        &self,
    ) -> crate::operation::list_instance_groups::builders::ListInstanceGroupsFluentBuilder {
        crate::operation::list_instance_groups::builders::ListInstanceGroupsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
