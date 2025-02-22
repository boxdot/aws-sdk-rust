// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAppInstances`](crate::operation::list_app_instances::builders::ListAppInstancesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_app_instances::builders::ListAppInstancesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_app_instances::builders::ListAppInstancesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_app_instances::builders::ListAppInstancesFluentBuilder::set_max_results): <p>The maximum number of <code>AppInstance</code>s that you want to return.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_app_instances::builders::ListAppInstancesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_app_instances::builders::ListAppInstancesFluentBuilder::set_next_token): <p>The token passed by previous API requests until you reach the maximum number of <code>AppInstance</code>s.</p>
    /// - On success, responds with [`ListAppInstancesOutput`](crate::operation::list_app_instances::ListAppInstancesOutput) with field(s):
    ///   - [`app_instances(Option<Vec<AppInstanceSummary>>)`](crate::operation::list_app_instances::ListAppInstancesOutput::app_instances): <p>The information for each <code>AppInstance</code>.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_app_instances::ListAppInstancesOutput::next_token): <p>The token passed by previous API requests until the maximum number of <code>AppInstance</code>s is reached.</p>
    /// - On failure, responds with [`SdkError<ListAppInstancesError>`](crate::operation::list_app_instances::ListAppInstancesError)
    pub fn list_app_instances(
        &self,
    ) -> crate::operation::list_app_instances::builders::ListAppInstancesFluentBuilder {
        crate::operation::list_app_instances::builders::ListAppInstancesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
