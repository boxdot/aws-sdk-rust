// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeChangeSet`](crate::operation::describe_change_set::builders::DescribeChangeSetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`change_set_name(impl ::std::convert::Into<String>)`](crate::operation::describe_change_set::builders::DescribeChangeSetFluentBuilder::change_set_name) / [`set_change_set_name(Option<String>)`](crate::operation::describe_change_set::builders::DescribeChangeSetFluentBuilder::set_change_set_name): <p>The name or Amazon Resource Name (ARN) of the change set that you want to describe.</p>
    ///   - [`stack_name(impl ::std::convert::Into<String>)`](crate::operation::describe_change_set::builders::DescribeChangeSetFluentBuilder::stack_name) / [`set_stack_name(Option<String>)`](crate::operation::describe_change_set::builders::DescribeChangeSetFluentBuilder::set_stack_name): <p>If you specified the name of a change set, specify the stack name or ID (ARN) of the change set you want to describe.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_change_set::builders::DescribeChangeSetFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_change_set::builders::DescribeChangeSetFluentBuilder::set_next_token): <p>A string (provided by the <code>DescribeChangeSet</code> response output) that identifies the next page of information that you want to retrieve.</p>
    /// - On success, responds with [`DescribeChangeSetOutput`](crate::operation::describe_change_set::DescribeChangeSetOutput) with field(s):
    ///   - [`change_set_name(Option<String>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::change_set_name): <p>The name of the change set.</p>
    ///   - [`change_set_id(Option<String>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::change_set_id): <p>The Amazon Resource Name (ARN) of the change set.</p>
    ///   - [`stack_id(Option<String>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::stack_id): <p>The Amazon Resource Name (ARN) of the stack that's associated with the change set.</p>
    ///   - [`stack_name(Option<String>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::stack_name): <p>The name of the stack that's associated with the change set.</p>
    ///   - [`description(Option<String>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::description): <p>Information about the change set.</p>
    ///   - [`parameters(Option<Vec<Parameter>>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::parameters): <p>A list of <code>Parameter</code> structures that describes the input parameters and their values used to create the change set. For more information, see the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html">Parameter</a> data type.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::creation_time): <p>The start time when the change set was created, in UTC.</p>
    ///   - [`execution_status(Option<ExecutionStatus>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::execution_status): <p>If the change set execution status is <code>AVAILABLE</code>, you can execute the change set. If you can't execute the change set, the status indicates why. For example, a change set might be in an <code>UNAVAILABLE</code> state because CloudFormation is still creating it or in an <code>OBSOLETE</code> state because the stack was already updated.</p>
    ///   - [`status(Option<ChangeSetStatus>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::status): <p>The current status of the change set, such as <code>CREATE_IN_PROGRESS</code>, <code>CREATE_COMPLETE</code>, or <code>FAILED</code>.</p>
    ///   - [`status_reason(Option<String>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::status_reason): <p>A description of the change set's status. For example, if your attempt to create a change set failed, CloudFormation shows the error message.</p>
    ///   - [`notification_ar_ns(Option<Vec<String>>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::notification_ar_ns): <p>The ARNs of the Amazon Simple Notification Service (Amazon SNS) topics that will be associated with the stack if you execute the change set.</p>
    ///   - [`rollback_configuration(Option<RollbackConfiguration>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::rollback_configuration): <p>The rollback triggers for CloudFormation to monitor during stack creation and updating operations, and for the specified monitoring period afterwards.</p>
    ///   - [`capabilities(Option<Vec<Capability>>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::capabilities): <p>If you execute the change set, the list of capabilities that were explicitly acknowledged when the change set was created.</p>
    ///   - [`tags(Option<Vec<Tag>>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::tags): <p>If you execute the change set, the tags that will be associated with the stack.</p>
    ///   - [`changes(Option<Vec<Change>>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::changes): <p>A list of <code>Change</code> structures that describes the resources CloudFormation changes if you execute the change set.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::next_token): <p>If the output exceeds 1 MB, a string that identifies the next page of changes. If there is no additional page, this value is null.</p>
    ///   - [`include_nested_stacks(Option<bool>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::include_nested_stacks): <p>Verifies if <code>IncludeNestedStacks</code> is set to <code>True</code>.</p>
    ///   - [`parent_change_set_id(Option<String>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::parent_change_set_id): <p>Specifies the change set ID of the parent change set in the current nested change set hierarchy.</p>
    ///   - [`root_change_set_id(Option<String>)`](crate::operation::describe_change_set::DescribeChangeSetOutput::root_change_set_id): <p>Specifies the change set ID of the root change set in the current nested change set hierarchy.</p>
    /// - On failure, responds with [`SdkError<DescribeChangeSetError>`](crate::operation::describe_change_set::DescribeChangeSetError)
    pub fn describe_change_set(
        &self,
    ) -> crate::operation::describe_change_set::builders::DescribeChangeSetFluentBuilder {
        crate::operation::describe_change_set::builders::DescribeChangeSetFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
