// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetCustomEntityType`](crate::operation::get_custom_entity_type::builders::GetCustomEntityTypeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::get_custom_entity_type::builders::GetCustomEntityTypeFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::get_custom_entity_type::builders::GetCustomEntityTypeFluentBuilder::set_name): <p>The name of the custom pattern that you want to retrieve.</p>
    /// - On success, responds with [`GetCustomEntityTypeOutput`](crate::operation::get_custom_entity_type::GetCustomEntityTypeOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::operation::get_custom_entity_type::GetCustomEntityTypeOutput::name): <p>The name of the custom pattern that you retrieved.</p>
    ///   - [`regex_string(Option<String>)`](crate::operation::get_custom_entity_type::GetCustomEntityTypeOutput::regex_string): <p>A regular expression string that is used for detecting sensitive data in a custom pattern.</p>
    ///   - [`context_words(Option<Vec<String>>)`](crate::operation::get_custom_entity_type::GetCustomEntityTypeOutput::context_words): <p>A list of context words if specified when you created the custom pattern. If none of these context words are found within the vicinity of the regular expression the data will not be detected as sensitive data.</p>
    /// - On failure, responds with [`SdkError<GetCustomEntityTypeError>`](crate::operation::get_custom_entity_type::GetCustomEntityTypeError)
    pub fn get_custom_entity_type(
        &self,
    ) -> crate::operation::get_custom_entity_type::builders::GetCustomEntityTypeFluentBuilder {
        crate::operation::get_custom_entity_type::builders::GetCustomEntityTypeFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
