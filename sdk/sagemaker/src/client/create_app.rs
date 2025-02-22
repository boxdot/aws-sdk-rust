// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateApp`](crate::operation::create_app::builders::CreateAppFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_id(impl ::std::convert::Into<String>)`](crate::operation::create_app::builders::CreateAppFluentBuilder::domain_id) / [`set_domain_id(Option<String>)`](crate::operation::create_app::builders::CreateAppFluentBuilder::set_domain_id): <p>The domain ID.</p>
    ///   - [`user_profile_name(impl ::std::convert::Into<String>)`](crate::operation::create_app::builders::CreateAppFluentBuilder::user_profile_name) / [`set_user_profile_name(Option<String>)`](crate::operation::create_app::builders::CreateAppFluentBuilder::set_user_profile_name): <p>The user profile name. If this value is not set, then <code>SpaceName</code> must be set.</p>
    ///   - [`app_type(AppType)`](crate::operation::create_app::builders::CreateAppFluentBuilder::app_type) / [`set_app_type(Option<AppType>)`](crate::operation::create_app::builders::CreateAppFluentBuilder::set_app_type): <p>The type of app.</p>
    ///   - [`app_name(impl ::std::convert::Into<String>)`](crate::operation::create_app::builders::CreateAppFluentBuilder::app_name) / [`set_app_name(Option<String>)`](crate::operation::create_app::builders::CreateAppFluentBuilder::set_app_name): <p>The name of the app.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_app::builders::CreateAppFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_app::builders::CreateAppFluentBuilder::set_tags): <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource.</p>
    ///   - [`resource_spec(ResourceSpec)`](crate::operation::create_app::builders::CreateAppFluentBuilder::resource_spec) / [`set_resource_spec(Option<ResourceSpec>)`](crate::operation::create_app::builders::CreateAppFluentBuilder::set_resource_spec): <p>The instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance.</p> <note>   <p>The value of <code>InstanceType</code> passed as part of the <code>ResourceSpec</code> in the <code>CreateApp</code> call overrides the value passed as part of the <code>ResourceSpec</code> configured for the user profile or the domain. If <code>InstanceType</code> is not specified in any of those three <code>ResourceSpec</code> values for a <code>KernelGateway</code> app, the <code>CreateApp</code> call fails with a request validation error.</p>  </note>
    ///   - [`space_name(impl ::std::convert::Into<String>)`](crate::operation::create_app::builders::CreateAppFluentBuilder::space_name) / [`set_space_name(Option<String>)`](crate::operation::create_app::builders::CreateAppFluentBuilder::set_space_name): <p>The name of the space. If this value is not set, then <code>UserProfileName</code> must be set.</p>
    /// - On success, responds with [`CreateAppOutput`](crate::operation::create_app::CreateAppOutput) with field(s):
    ///   - [`app_arn(Option<String>)`](crate::operation::create_app::CreateAppOutput::app_arn): <p>The Amazon Resource Name (ARN) of the app.</p>
    /// - On failure, responds with [`SdkError<CreateAppError>`](crate::operation::create_app::CreateAppError)
    pub fn create_app(&self) -> crate::operation::create_app::builders::CreateAppFluentBuilder {
        crate::operation::create_app::builders::CreateAppFluentBuilder::new(self.handle.clone())
    }
}
