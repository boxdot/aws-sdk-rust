// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteKnowledgeBase`](crate::operation::delete_knowledge_base::builders::DeleteKnowledgeBaseFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`knowledge_base_id(impl ::std::convert::Into<String>)`](crate::operation::delete_knowledge_base::builders::DeleteKnowledgeBaseFluentBuilder::knowledge_base_id) / [`set_knowledge_base_id(Option<String>)`](crate::operation::delete_knowledge_base::builders::DeleteKnowledgeBaseFluentBuilder::set_knowledge_base_id): <p>The knowledge base to delete content from. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    /// - On success, responds with [`DeleteKnowledgeBaseOutput`](crate::operation::delete_knowledge_base::DeleteKnowledgeBaseOutput)
    /// - On failure, responds with [`SdkError<DeleteKnowledgeBaseError>`](crate::operation::delete_knowledge_base::DeleteKnowledgeBaseError)
    pub fn delete_knowledge_base(
        &self,
    ) -> crate::operation::delete_knowledge_base::builders::DeleteKnowledgeBaseFluentBuilder {
        crate::operation::delete_knowledge_base::builders::DeleteKnowledgeBaseFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
