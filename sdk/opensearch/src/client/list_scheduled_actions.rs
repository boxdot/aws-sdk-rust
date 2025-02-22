// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListScheduledActions`](crate::operation::list_scheduled_actions::builders::ListScheduledActionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_scheduled_actions::builders::ListScheduledActionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::list_scheduled_actions::builders::ListScheduledActionsFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::list_scheduled_actions::builders::ListScheduledActionsFluentBuilder::set_domain_name): <p>The name of the domain.</p>
    ///   - [`max_results(i32)`](crate::operation::list_scheduled_actions::builders::ListScheduledActionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_scheduled_actions::builders::ListScheduledActionsFluentBuilder::set_max_results): <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_scheduled_actions::builders::ListScheduledActionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_scheduled_actions::builders::ListScheduledActionsFluentBuilder::set_next_token): <p>If your initial <code>ListScheduledActions</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListScheduledActions</code> operations, which returns results in the next page.</p>
    /// - On success, responds with [`ListScheduledActionsOutput`](crate::operation::list_scheduled_actions::ListScheduledActionsOutput) with field(s):
    ///   - [`scheduled_actions(Option<Vec<ScheduledAction>>)`](crate::operation::list_scheduled_actions::ListScheduledActionsOutput::scheduled_actions): <p>A list of actions that are scheduled for the domain.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_scheduled_actions::ListScheduledActionsOutput::next_token): <p>When <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
    /// - On failure, responds with [`SdkError<ListScheduledActionsError>`](crate::operation::list_scheduled_actions::ListScheduledActionsError)
    pub fn list_scheduled_actions(
        &self,
    ) -> crate::operation::list_scheduled_actions::builders::ListScheduledActionsFluentBuilder {
        crate::operation::list_scheduled_actions::builders::ListScheduledActionsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
