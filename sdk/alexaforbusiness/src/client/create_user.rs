// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateUser`](crate::operation::create_user::builders::CreateUserFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_id(impl ::std::convert::Into<String>)`](crate::operation::create_user::builders::CreateUserFluentBuilder::user_id) / [`set_user_id(Option<String>)`](crate::operation::create_user::builders::CreateUserFluentBuilder::set_user_id): <p>The ARN for the user.</p>
    ///   - [`first_name(impl ::std::convert::Into<String>)`](crate::operation::create_user::builders::CreateUserFluentBuilder::first_name) / [`set_first_name(Option<String>)`](crate::operation::create_user::builders::CreateUserFluentBuilder::set_first_name): <p>The first name for the user.</p>
    ///   - [`last_name(impl ::std::convert::Into<String>)`](crate::operation::create_user::builders::CreateUserFluentBuilder::last_name) / [`set_last_name(Option<String>)`](crate::operation::create_user::builders::CreateUserFluentBuilder::set_last_name): <p>The last name for the user.</p>
    ///   - [`email(impl ::std::convert::Into<String>)`](crate::operation::create_user::builders::CreateUserFluentBuilder::email) / [`set_email(Option<String>)`](crate::operation::create_user::builders::CreateUserFluentBuilder::set_email): <p>The email address for the user.</p>
    ///   - [`client_request_token(impl ::std::convert::Into<String>)`](crate::operation::create_user::builders::CreateUserFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::create_user::builders::CreateUserFluentBuilder::set_client_request_token): <p>A unique, user-specified identifier for this request that ensures idempotency. </p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_user::builders::CreateUserFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_user::builders::CreateUserFluentBuilder::set_tags): <p>The tags for the user.</p>
    /// - On success, responds with [`CreateUserOutput`](crate::operation::create_user::CreateUserOutput) with field(s):
    ///   - [`user_arn(Option<String>)`](crate::operation::create_user::CreateUserOutput::user_arn): <p>The ARN of the newly created user in the response.</p>
    /// - On failure, responds with [`SdkError<CreateUserError>`](crate::operation::create_user::CreateUserError)
    pub fn create_user(&self) -> crate::operation::create_user::builders::CreateUserFluentBuilder {
        crate::operation::create_user::builders::CreateUserFluentBuilder::new(self.handle.clone())
    }
}
