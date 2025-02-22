// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteUserHierarchyGroup`](crate::operation::delete_user_hierarchy_group::builders::DeleteUserHierarchyGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`hierarchy_group_id(impl ::std::convert::Into<String>)`](crate::operation::delete_user_hierarchy_group::builders::DeleteUserHierarchyGroupFluentBuilder::hierarchy_group_id) / [`set_hierarchy_group_id(Option<String>)`](crate::operation::delete_user_hierarchy_group::builders::DeleteUserHierarchyGroupFluentBuilder::set_hierarchy_group_id): <p>The identifier of the hierarchy group.</p>
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::delete_user_hierarchy_group::builders::DeleteUserHierarchyGroupFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::delete_user_hierarchy_group::builders::DeleteUserHierarchyGroupFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    /// - On success, responds with [`DeleteUserHierarchyGroupOutput`](crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroupOutput)
    /// - On failure, responds with [`SdkError<DeleteUserHierarchyGroupError>`](crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroupError)
    pub fn delete_user_hierarchy_group(&self) -> crate::operation::delete_user_hierarchy_group::builders::DeleteUserHierarchyGroupFluentBuilder{
        crate::operation::delete_user_hierarchy_group::builders::DeleteUserHierarchyGroupFluentBuilder::new(self.handle.clone())
    }
}
