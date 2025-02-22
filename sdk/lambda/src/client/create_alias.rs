// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateAlias`](crate::operation::create_alias::builders::CreateAliasFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_name(impl ::std::convert::Into<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::set_function_name): <p>The name of the Lambda function.</p>  <p class="title"> <b>Name formats</b> </p>  <ul>   <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li>   <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li>   <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li>  </ul>  <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::set_name): <p>The name of the alias.</p>
    ///   - [`function_version(impl ::std::convert::Into<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::function_version) / [`set_function_version(Option<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::set_function_version): <p>The function version that the alias invokes.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::set_description): <p>A description of the alias.</p>
    ///   - [`routing_config(AliasRoutingConfiguration)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::routing_config) / [`set_routing_config(Option<AliasRoutingConfiguration>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::set_routing_config): <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-aliases.html#configuring-alias-routing">routing configuration</a> of the alias.</p>
    /// - On success, responds with [`CreateAliasOutput`](crate::operation::create_alias::CreateAliasOutput) with field(s):
    ///   - [`alias_arn(Option<String>)`](crate::operation::create_alias::CreateAliasOutput::alias_arn): <p>The Amazon Resource Name (ARN) of the alias.</p>
    ///   - [`name(Option<String>)`](crate::operation::create_alias::CreateAliasOutput::name): <p>The name of the alias.</p>
    ///   - [`function_version(Option<String>)`](crate::operation::create_alias::CreateAliasOutput::function_version): <p>The function version that the alias invokes.</p>
    ///   - [`description(Option<String>)`](crate::operation::create_alias::CreateAliasOutput::description): <p>A description of the alias.</p>
    ///   - [`routing_config(Option<AliasRoutingConfiguration>)`](crate::operation::create_alias::CreateAliasOutput::routing_config): <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-traffic-shifting-using-aliases.html">routing configuration</a> of the alias.</p>
    ///   - [`revision_id(Option<String>)`](crate::operation::create_alias::CreateAliasOutput::revision_id): <p>A unique identifier that changes when you update the alias.</p>
    /// - On failure, responds with [`SdkError<CreateAliasError>`](crate::operation::create_alias::CreateAliasError)
    pub fn create_alias(
        &self,
    ) -> crate::operation::create_alias::builders::CreateAliasFluentBuilder {
        crate::operation::create_alias::builders::CreateAliasFluentBuilder::new(self.handle.clone())
    }
}
