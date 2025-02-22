// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListIpRoutes`](crate::operation::list_ip_routes::builders::ListIpRoutesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_ip_routes::builders::ListIpRoutesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl ::std::convert::Into<String>)`](crate::operation::list_ip_routes::builders::ListIpRoutesFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::list_ip_routes::builders::ListIpRoutesFluentBuilder::set_directory_id): <p>Identifier (ID) of the directory for which you want to retrieve the IP addresses.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_ip_routes::builders::ListIpRoutesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_ip_routes::builders::ListIpRoutesFluentBuilder::set_next_token): <p>The <i>ListIpRoutes.NextToken</i> value from a previous call to <code>ListIpRoutes</code>. Pass null if this is the first call.</p>
    ///   - [`limit(i32)`](crate::operation::list_ip_routes::builders::ListIpRoutesFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_ip_routes::builders::ListIpRoutesFluentBuilder::set_limit): <p>Maximum number of items to return. If this value is zero, the maximum number of items is specified by the limitations of the operation.</p>
    /// - On success, responds with [`ListIpRoutesOutput`](crate::operation::list_ip_routes::ListIpRoutesOutput) with field(s):
    ///   - [`ip_routes_info(Option<Vec<IpRouteInfo>>)`](crate::operation::list_ip_routes::ListIpRoutesOutput::ip_routes_info): <p>A list of <code>IpRoute</code>s.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_ip_routes::ListIpRoutesOutput::next_token): <p>If not null, more results are available. Pass this value for the <i>NextToken</i> parameter in a subsequent call to <code>ListIpRoutes</code> to retrieve the next set of items.</p>
    /// - On failure, responds with [`SdkError<ListIpRoutesError>`](crate::operation::list_ip_routes::ListIpRoutesError)
    pub fn list_ip_routes(
        &self,
    ) -> crate::operation::list_ip_routes::builders::ListIpRoutesFluentBuilder {
        crate::operation::list_ip_routes::builders::ListIpRoutesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
