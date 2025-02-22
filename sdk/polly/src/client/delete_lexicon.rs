// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteLexicon`](crate::operation::delete_lexicon::builders::DeleteLexiconFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::delete_lexicon::builders::DeleteLexiconFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_lexicon::builders::DeleteLexiconFluentBuilder::set_name): <p>The name of the lexicon to delete. Must be an existing lexicon in the region.</p>
    /// - On success, responds with [`DeleteLexiconOutput`](crate::operation::delete_lexicon::DeleteLexiconOutput)
    /// - On failure, responds with [`SdkError<DeleteLexiconError>`](crate::operation::delete_lexicon::DeleteLexiconError)
    pub fn delete_lexicon(
        &self,
    ) -> crate::operation::delete_lexicon::builders::DeleteLexiconFluentBuilder {
        crate::operation::delete_lexicon::builders::DeleteLexiconFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
