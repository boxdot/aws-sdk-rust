// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAppInstanceUser`](crate::operation::delete_app_instance_user::builders::DeleteAppInstanceUserFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_instance_user_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_app_instance_user::builders::DeleteAppInstanceUserFluentBuilder::app_instance_user_arn) / [`set_app_instance_user_arn(Option<String>)`](crate::operation::delete_app_instance_user::builders::DeleteAppInstanceUserFluentBuilder::set_app_instance_user_arn): <p>The ARN of the user request being deleted.</p>
    /// - On success, responds with [`DeleteAppInstanceUserOutput`](crate::operation::delete_app_instance_user::DeleteAppInstanceUserOutput)
    /// - On failure, responds with [`SdkError<DeleteAppInstanceUserError>`](crate::operation::delete_app_instance_user::DeleteAppInstanceUserError)
    pub fn delete_app_instance_user(
        &self,
    ) -> crate::operation::delete_app_instance_user::builders::DeleteAppInstanceUserFluentBuilder
    {
        crate::operation::delete_app_instance_user::builders::DeleteAppInstanceUserFluentBuilder::new(self.handle.clone())
    }
}
