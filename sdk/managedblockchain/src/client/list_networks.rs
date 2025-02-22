// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListNetworks`](crate::operation::list_networks::builders::ListNetworksFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_networks::builders::ListNetworksFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::list_networks::builders::ListNetworksFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::list_networks::builders::ListNetworksFluentBuilder::set_name): <p>The name of the network.</p>
    ///   - [`framework(Framework)`](crate::operation::list_networks::builders::ListNetworksFluentBuilder::framework) / [`set_framework(Option<Framework>)`](crate::operation::list_networks::builders::ListNetworksFluentBuilder::set_framework): <p>An optional framework specifier. If provided, only networks of this framework type are listed.</p>
    ///   - [`status(NetworkStatus)`](crate::operation::list_networks::builders::ListNetworksFluentBuilder::status) / [`set_status(Option<NetworkStatus>)`](crate::operation::list_networks::builders::ListNetworksFluentBuilder::set_status): <p>An optional status specifier. If provided, only networks currently in this status are listed.</p>  <p>Applies only to Hyperledger Fabric.</p>
    ///   - [`max_results(i32)`](crate::operation::list_networks::builders::ListNetworksFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_networks::builders::ListNetworksFluentBuilder::set_max_results): <p>The maximum number of networks to list.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_networks::builders::ListNetworksFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_networks::builders::ListNetworksFluentBuilder::set_next_token): <p>The pagination token that indicates the next set of results to retrieve.</p>
    /// - On success, responds with [`ListNetworksOutput`](crate::operation::list_networks::ListNetworksOutput) with field(s):
    ///   - [`networks(Option<Vec<NetworkSummary>>)`](crate::operation::list_networks::ListNetworksOutput::networks): <p>An array of <code>NetworkSummary</code> objects that contain configuration properties for each network.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_networks::ListNetworksOutput::next_token): <p>The pagination token that indicates the next set of results to retrieve.</p>
    /// - On failure, responds with [`SdkError<ListNetworksError>`](crate::operation::list_networks::ListNetworksError)
    pub fn list_networks(
        &self,
    ) -> crate::operation::list_networks::builders::ListNetworksFluentBuilder {
        crate::operation::list_networks::builders::ListNetworksFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
