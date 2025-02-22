// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAssistants`](crate::operation::list_assistants::builders::ListAssistantsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_assistants::builders::ListAssistantsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_assistants::builders::ListAssistantsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_assistants::builders::ListAssistantsFluentBuilder::set_next_token): <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_assistants::builders::ListAssistantsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_assistants::builders::ListAssistantsFluentBuilder::set_max_results): <p>The maximum number of results to return per page.</p>
    /// - On success, responds with [`ListAssistantsOutput`](crate::operation::list_assistants::ListAssistantsOutput) with field(s):
    ///   - [`assistant_summaries(Option<Vec<AssistantSummary>>)`](crate::operation::list_assistants::ListAssistantsOutput::assistant_summaries): <p>Information about the assistants.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_assistants::ListAssistantsOutput::next_token): <p>If there are additional results, this is the token for the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListAssistantsError>`](crate::operation::list_assistants::ListAssistantsError)
    pub fn list_assistants(
        &self,
    ) -> crate::operation::list_assistants::builders::ListAssistantsFluentBuilder {
        crate::operation::list_assistants::builders::ListAssistantsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
