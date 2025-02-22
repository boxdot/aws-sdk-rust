// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SetTypeDefaultVersion`](crate::operation::set_type_default_version::builders::SetTypeDefaultVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl ::std::convert::Into<String>)`](crate::operation::set_type_default_version::builders::SetTypeDefaultVersionFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::set_type_default_version::builders::SetTypeDefaultVersionFluentBuilder::set_arn): <p>The Amazon Resource Name (ARN) of the extension for which you want version summary information.</p>  <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    ///   - [`r#type(RegistryType)`](crate::operation::set_type_default_version::builders::SetTypeDefaultVersionFluentBuilder::type) / [`set_type(Option<RegistryType>)`](crate::operation::set_type_default_version::builders::SetTypeDefaultVersionFluentBuilder::set_type): <p>The kind of extension.</p>  <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    ///   - [`type_name(impl ::std::convert::Into<String>)`](crate::operation::set_type_default_version::builders::SetTypeDefaultVersionFluentBuilder::type_name) / [`set_type_name(Option<String>)`](crate::operation::set_type_default_version::builders::SetTypeDefaultVersionFluentBuilder::set_type_name): <p>The name of the extension.</p>  <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    ///   - [`version_id(impl ::std::convert::Into<String>)`](crate::operation::set_type_default_version::builders::SetTypeDefaultVersionFluentBuilder::version_id) / [`set_version_id(Option<String>)`](crate::operation::set_type_default_version::builders::SetTypeDefaultVersionFluentBuilder::set_version_id): <p>The ID of a specific version of the extension. The version ID is the value at the end of the Amazon Resource Name (ARN) assigned to the extension version when it is registered.</p>
    /// - On success, responds with [`SetTypeDefaultVersionOutput`](crate::operation::set_type_default_version::SetTypeDefaultVersionOutput)
    /// - On failure, responds with [`SdkError<SetTypeDefaultVersionError>`](crate::operation::set_type_default_version::SetTypeDefaultVersionError)
    pub fn set_type_default_version(
        &self,
    ) -> crate::operation::set_type_default_version::builders::SetTypeDefaultVersionFluentBuilder
    {
        crate::operation::set_type_default_version::builders::SetTypeDefaultVersionFluentBuilder::new(self.handle.clone())
    }
}
