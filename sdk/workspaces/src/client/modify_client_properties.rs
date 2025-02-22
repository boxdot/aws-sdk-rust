// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyClientProperties`](crate::operation::modify_client_properties::builders::ModifyClientPropertiesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_id(impl ::std::convert::Into<String>)`](crate::operation::modify_client_properties::builders::ModifyClientPropertiesFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::modify_client_properties::builders::ModifyClientPropertiesFluentBuilder::set_resource_id): <p>The resource identifiers, in the form of directory IDs.</p>
    ///   - [`client_properties(ClientProperties)`](crate::operation::modify_client_properties::builders::ModifyClientPropertiesFluentBuilder::client_properties) / [`set_client_properties(Option<ClientProperties>)`](crate::operation::modify_client_properties::builders::ModifyClientPropertiesFluentBuilder::set_client_properties): <p>Information about the Amazon WorkSpaces client.</p>
    /// - On success, responds with [`ModifyClientPropertiesOutput`](crate::operation::modify_client_properties::ModifyClientPropertiesOutput)
    /// - On failure, responds with [`SdkError<ModifyClientPropertiesError>`](crate::operation::modify_client_properties::ModifyClientPropertiesError)
    pub fn modify_client_properties(
        &self,
    ) -> crate::operation::modify_client_properties::builders::ModifyClientPropertiesFluentBuilder
    {
        crate::operation::modify_client_properties::builders::ModifyClientPropertiesFluentBuilder::new(self.handle.clone())
    }
}
