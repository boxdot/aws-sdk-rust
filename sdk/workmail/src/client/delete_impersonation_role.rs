// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteImpersonationRole`](crate::operation::delete_impersonation_role::builders::DeleteImpersonationRoleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`organization_id(impl ::std::convert::Into<String>)`](crate::operation::delete_impersonation_role::builders::DeleteImpersonationRoleFluentBuilder::organization_id) / [`set_organization_id(Option<String>)`](crate::operation::delete_impersonation_role::builders::DeleteImpersonationRoleFluentBuilder::set_organization_id): <p>The WorkMail organization from which to delete the impersonation role.</p>
    ///   - [`impersonation_role_id(impl ::std::convert::Into<String>)`](crate::operation::delete_impersonation_role::builders::DeleteImpersonationRoleFluentBuilder::impersonation_role_id) / [`set_impersonation_role_id(Option<String>)`](crate::operation::delete_impersonation_role::builders::DeleteImpersonationRoleFluentBuilder::set_impersonation_role_id): <p>The ID of the impersonation role to delete.</p>
    /// - On success, responds with [`DeleteImpersonationRoleOutput`](crate::operation::delete_impersonation_role::DeleteImpersonationRoleOutput)
    /// - On failure, responds with [`SdkError<DeleteImpersonationRoleError>`](crate::operation::delete_impersonation_role::DeleteImpersonationRoleError)
    pub fn delete_impersonation_role(
        &self,
    ) -> crate::operation::delete_impersonation_role::builders::DeleteImpersonationRoleFluentBuilder
    {
        crate::operation::delete_impersonation_role::builders::DeleteImpersonationRoleFluentBuilder::new(self.handle.clone())
    }
}
