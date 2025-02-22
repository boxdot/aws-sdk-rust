// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAssociatedRole`](crate::operation::get_associated_role::builders::GetAssociatedRoleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`group_id(impl ::std::convert::Into<String>)`](crate::operation::get_associated_role::builders::GetAssociatedRoleFluentBuilder::group_id) / [`set_group_id(Option<String>)`](crate::operation::get_associated_role::builders::GetAssociatedRoleFluentBuilder::set_group_id): The ID of the Greengrass group.
    /// - On success, responds with [`GetAssociatedRoleOutput`](crate::operation::get_associated_role::GetAssociatedRoleOutput) with field(s):
    ///   - [`associated_at(Option<String>)`](crate::operation::get_associated_role::GetAssociatedRoleOutput::associated_at): The time when the role was associated with the group.
    ///   - [`role_arn(Option<String>)`](crate::operation::get_associated_role::GetAssociatedRoleOutput::role_arn): The ARN of the role that is associated with the group.
    /// - On failure, responds with [`SdkError<GetAssociatedRoleError>`](crate::operation::get_associated_role::GetAssociatedRoleError)
    pub fn get_associated_role(
        &self,
    ) -> crate::operation::get_associated_role::builders::GetAssociatedRoleFluentBuilder {
        crate::operation::get_associated_role::builders::GetAssociatedRoleFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
