// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListHosts`](crate::operation::list_hosts::builders::ListHostsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_hosts::builders::ListHostsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_hosts::builders::ListHostsFluentBuilder::max_results) / [`set_max_results(i32)`](crate::operation::list_hosts::builders::ListHostsFluentBuilder::set_max_results): <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_hosts::builders::ListHostsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_hosts::builders::ListHostsFluentBuilder::set_next_token): <p>The token that was returned from the previous <code>ListHosts</code> call, which can be used to return the next set of hosts in the list.</p>
    /// - On success, responds with [`ListHostsOutput`](crate::operation::list_hosts::ListHostsOutput) with field(s):
    ///   - [`hosts(Option<Vec<Host>>)`](crate::operation::list_hosts::ListHostsOutput::hosts): <p>A list of hosts and the details for each host, such as status, endpoint, and provider type.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_hosts::ListHostsOutput::next_token): <p>A token that can be used in the next <code>ListHosts</code> call. To view all items in the list, continue to call this operation with each subsequent token until no more <code>nextToken</code> values are returned.</p>
    /// - On failure, responds with [`SdkError<ListHostsError>`](crate::operation::list_hosts::ListHostsError)
    pub fn list_hosts(&self) -> crate::operation::list_hosts::builders::ListHostsFluentBuilder {
        crate::operation::list_hosts::builders::ListHostsFluentBuilder::new(self.handle.clone())
    }
}
