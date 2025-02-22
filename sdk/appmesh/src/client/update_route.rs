// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateRoute`](crate::operation::update_route::builders::UpdateRouteFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`route_name(impl ::std::convert::Into<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::route_name) / [`set_route_name(Option<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::set_route_name): <p>The name of the route to update.</p>
    ///   - [`mesh_name(impl ::std::convert::Into<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::mesh_name) / [`set_mesh_name(Option<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::set_mesh_name): <p>The name of the service mesh that the route resides in.</p>
    ///   - [`virtual_router_name(impl ::std::convert::Into<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::virtual_router_name) / [`set_virtual_router_name(Option<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::set_virtual_router_name): <p>The name of the virtual router that the route is associated with.</p>
    ///   - [`spec(RouteSpec)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::spec) / [`set_spec(Option<RouteSpec>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::set_spec): <p>The new route specification to apply. This overwrites the existing data.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    ///   - [`mesh_owner(impl ::std::convert::Into<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::mesh_owner) / [`set_mesh_owner(Option<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::set_mesh_owner): <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    /// - On success, responds with [`UpdateRouteOutput`](crate::operation::update_route::UpdateRouteOutput) with field(s):
    ///   - [`route(Option<RouteData>)`](crate::operation::update_route::UpdateRouteOutput::route): <p>A full description of the route that was updated.</p>
    /// - On failure, responds with [`SdkError<UpdateRouteError>`](crate::operation::update_route::UpdateRouteError)
    pub fn update_route(
        &self,
    ) -> crate::operation::update_route::builders::UpdateRouteFluentBuilder {
        crate::operation::update_route::builders::UpdateRouteFluentBuilder::new(self.handle.clone())
    }
}
