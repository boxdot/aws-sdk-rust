// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RemoveTagsFromStream`](crate::operation::remove_tags_from_stream::builders::RemoveTagsFromStreamFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`stream_name(impl ::std::convert::Into<String>)`](crate::operation::remove_tags_from_stream::builders::RemoveTagsFromStreamFluentBuilder::stream_name) / [`set_stream_name(Option<String>)`](crate::operation::remove_tags_from_stream::builders::RemoveTagsFromStreamFluentBuilder::set_stream_name): <p>The name of the stream.</p>
    ///   - [`tag_keys(Vec<String>)`](crate::operation::remove_tags_from_stream::builders::RemoveTagsFromStreamFluentBuilder::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::operation::remove_tags_from_stream::builders::RemoveTagsFromStreamFluentBuilder::set_tag_keys): <p>A list of tag keys. Each corresponding tag is removed from the stream.</p>
    ///   - [`stream_arn(impl ::std::convert::Into<String>)`](crate::operation::remove_tags_from_stream::builders::RemoveTagsFromStreamFluentBuilder::stream_arn) / [`set_stream_arn(Option<String>)`](crate::operation::remove_tags_from_stream::builders::RemoveTagsFromStreamFluentBuilder::set_stream_arn): <p>The ARN of the stream.</p>
    /// - On success, responds with [`RemoveTagsFromStreamOutput`](crate::operation::remove_tags_from_stream::RemoveTagsFromStreamOutput)
    /// - On failure, responds with [`SdkError<RemoveTagsFromStreamError>`](crate::operation::remove_tags_from_stream::RemoveTagsFromStreamError)
    pub fn remove_tags_from_stream(
        &self,
    ) -> crate::operation::remove_tags_from_stream::builders::RemoveTagsFromStreamFluentBuilder
    {
        crate::operation::remove_tags_from_stream::builders::RemoveTagsFromStreamFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
