// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PublishSchema`](crate::operation::publish_schema::builders::PublishSchemaFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`development_schema_arn(impl ::std::convert::Into<String>)`](crate::operation::publish_schema::builders::PublishSchemaFluentBuilder::development_schema_arn) / [`set_development_schema_arn(Option<String>)`](crate::operation::publish_schema::builders::PublishSchemaFluentBuilder::set_development_schema_arn): <p>The Amazon Resource Name (ARN) that is associated with the development schema. For more information, see <code>arns</code>.</p>
    ///   - [`version(impl ::std::convert::Into<String>)`](crate::operation::publish_schema::builders::PublishSchemaFluentBuilder::version) / [`set_version(Option<String>)`](crate::operation::publish_schema::builders::PublishSchemaFluentBuilder::set_version): <p>The major version under which the schema will be published. Schemas have both a major and minor version associated with them.</p>
    ///   - [`minor_version(impl ::std::convert::Into<String>)`](crate::operation::publish_schema::builders::PublishSchemaFluentBuilder::minor_version) / [`set_minor_version(Option<String>)`](crate::operation::publish_schema::builders::PublishSchemaFluentBuilder::set_minor_version): <p>The minor version under which the schema will be published. This parameter is recommended. Schemas have both a major and minor version associated with them.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::publish_schema::builders::PublishSchemaFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::publish_schema::builders::PublishSchemaFluentBuilder::set_name): <p>The new name under which the schema will be published. If this is not provided, the development schema is considered.</p>
    /// - On success, responds with [`PublishSchemaOutput`](crate::operation::publish_schema::PublishSchemaOutput) with field(s):
    ///   - [`published_schema_arn(Option<String>)`](crate::operation::publish_schema::PublishSchemaOutput::published_schema_arn): <p>The ARN that is associated with the published schema. For more information, see <code>arns</code>.</p>
    /// - On failure, responds with [`SdkError<PublishSchemaError>`](crate::operation::publish_schema::PublishSchemaError)
    pub fn publish_schema(
        &self,
    ) -> crate::operation::publish_schema::builders::PublishSchemaFluentBuilder {
        crate::operation::publish_schema::builders::PublishSchemaFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
