// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SetTaskStatus`](crate::operation::set_task_status::builders::SetTaskStatusFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`task_id(impl ::std::convert::Into<String>)`](crate::operation::set_task_status::builders::SetTaskStatusFluentBuilder::task_id) / [`set_task_id(Option<String>)`](crate::operation::set_task_status::builders::SetTaskStatusFluentBuilder::set_task_id): <p>The ID of the task assigned to the task runner. This value is provided in the response for <code>PollForTask</code>.</p>
    ///   - [`task_status(TaskStatus)`](crate::operation::set_task_status::builders::SetTaskStatusFluentBuilder::task_status) / [`set_task_status(Option<TaskStatus>)`](crate::operation::set_task_status::builders::SetTaskStatusFluentBuilder::set_task_status): <p>If <code>FINISHED</code>, the task successfully completed. If <code>FAILED</code>, the task ended unsuccessfully. Preconditions use false.</p>
    ///   - [`error_id(impl ::std::convert::Into<String>)`](crate::operation::set_task_status::builders::SetTaskStatusFluentBuilder::error_id) / [`set_error_id(Option<String>)`](crate::operation::set_task_status::builders::SetTaskStatusFluentBuilder::set_error_id): <p>If an error occurred during the task, this value specifies the error code. This value is set on the physical attempt object. It is used to display error information to the user. It should not start with string "Service_" which is reserved by the system.</p>
    ///   - [`error_message(impl ::std::convert::Into<String>)`](crate::operation::set_task_status::builders::SetTaskStatusFluentBuilder::error_message) / [`set_error_message(Option<String>)`](crate::operation::set_task_status::builders::SetTaskStatusFluentBuilder::set_error_message): <p>If an error occurred during the task, this value specifies a text description of the error. This value is set on the physical attempt object. It is used to display error information to the user. The web service does not parse this value.</p>
    ///   - [`error_stack_trace(impl ::std::convert::Into<String>)`](crate::operation::set_task_status::builders::SetTaskStatusFluentBuilder::error_stack_trace) / [`set_error_stack_trace(Option<String>)`](crate::operation::set_task_status::builders::SetTaskStatusFluentBuilder::set_error_stack_trace): <p>If an error occurred during the task, this value specifies the stack trace associated with the error. This value is set on the physical attempt object. It is used to display error information to the user. The web service does not parse this value.</p>
    /// - On success, responds with [`SetTaskStatusOutput`](crate::operation::set_task_status::SetTaskStatusOutput)
    /// - On failure, responds with [`SdkError<SetTaskStatusError>`](crate::operation::set_task_status::SetTaskStatusError)
    pub fn set_task_status(
        &self,
    ) -> crate::operation::set_task_status::builders::SetTaskStatusFluentBuilder {
        crate::operation::set_task_status::builders::SetTaskStatusFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
