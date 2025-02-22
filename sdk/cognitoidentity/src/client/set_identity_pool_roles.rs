// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SetIdentityPoolRoles`](crate::operation::set_identity_pool_roles::builders::SetIdentityPoolRolesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identity_pool_id(impl ::std::convert::Into<String>)`](crate::operation::set_identity_pool_roles::builders::SetIdentityPoolRolesFluentBuilder::identity_pool_id) / [`set_identity_pool_id(Option<String>)`](crate::operation::set_identity_pool_roles::builders::SetIdentityPoolRolesFluentBuilder::set_identity_pool_id): <p>An identity pool ID in the format REGION:GUID.</p>
    ///   - [`roles(HashMap<String, String>)`](crate::operation::set_identity_pool_roles::builders::SetIdentityPoolRolesFluentBuilder::roles) / [`set_roles(Option<HashMap<String, String>>)`](crate::operation::set_identity_pool_roles::builders::SetIdentityPoolRolesFluentBuilder::set_roles): <p>The map of roles associated with this pool. For a given role, the key will be either "authenticated" or "unauthenticated" and the value will be the Role ARN.</p>
    ///   - [`role_mappings(HashMap<String, RoleMapping>)`](crate::operation::set_identity_pool_roles::builders::SetIdentityPoolRolesFluentBuilder::role_mappings) / [`set_role_mappings(Option<HashMap<String, RoleMapping>>)`](crate::operation::set_identity_pool_roles::builders::SetIdentityPoolRolesFluentBuilder::set_role_mappings): <p>How users for a specific identity provider are to mapped to roles. This is a string to <code>RoleMapping</code> object map. The string identifies the identity provider, for example, "graph.facebook.com" or "cognito-idp.us-east-1.amazonaws.com/us-east-1_abcdefghi:app_client_id".</p>  <p>Up to 25 rules can be specified per identity provider.</p>
    /// - On success, responds with [`SetIdentityPoolRolesOutput`](crate::operation::set_identity_pool_roles::SetIdentityPoolRolesOutput)
    /// - On failure, responds with [`SdkError<SetIdentityPoolRolesError>`](crate::operation::set_identity_pool_roles::SetIdentityPoolRolesError)
    pub fn set_identity_pool_roles(
        &self,
    ) -> crate::operation::set_identity_pool_roles::builders::SetIdentityPoolRolesFluentBuilder
    {
        crate::operation::set_identity_pool_roles::builders::SetIdentityPoolRolesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
