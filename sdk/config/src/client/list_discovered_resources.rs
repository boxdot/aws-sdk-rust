// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDiscoveredResources`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_type(ResourceType)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::resource_type) / [`set_resource_type(Option<ResourceType>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::set_resource_type): <p>The type of resources that you want Config to list in the response.</p>
    ///   - [`resource_ids(Vec<String>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::resource_ids) / [`set_resource_ids(Option<Vec<String>>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::set_resource_ids): <p>The IDs of only those resources that you want Config to list in the response. If you do not specify this parameter, Config lists all resources of the specified type that it has discovered. You can list a minimum of 1 resourceID and a maximum of 20 resourceIds.</p>
    ///   - [`resource_name(impl ::std::convert::Into<String>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::resource_name) / [`set_resource_name(Option<String>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::set_resource_name): <p>The custom name of only those resources that you want Config to list in the response. If you do not specify this parameter, Config lists all resources of the specified type that it has discovered.</p>
    ///   - [`limit(i32)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::set_limit): <p>The maximum number of resource identifiers returned on each page. The default is 100. You cannot specify a number greater than 100. If you specify 0, Config uses the default.</p>
    ///   - [`include_deleted_resources(bool)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::include_deleted_resources) / [`set_include_deleted_resources(Option<bool>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::set_include_deleted_resources): <p>Specifies whether Config includes deleted resources in the results. By default, deleted resources are not included.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::set_next_token): <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    /// - On success, responds with [`ListDiscoveredResourcesOutput`](crate::operation::list_discovered_resources::ListDiscoveredResourcesOutput) with field(s):
    ///   - [`resource_identifiers(Option<Vec<ResourceIdentifier>>)`](crate::operation::list_discovered_resources::ListDiscoveredResourcesOutput::resource_identifiers): <p>The details that identify a resource that is discovered by Config, including the resource type, ID, and (if available) the custom resource name.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_discovered_resources::ListDiscoveredResourcesOutput::next_token): <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    /// - On failure, responds with [`SdkError<ListDiscoveredResourcesError>`](crate::operation::list_discovered_resources::ListDiscoveredResourcesError)
    pub fn list_discovered_resources(
        &self,
    ) -> crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder
    {
        crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::new(self.handle.clone())
    }
}
