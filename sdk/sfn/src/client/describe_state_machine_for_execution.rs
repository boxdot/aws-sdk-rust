// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeStateMachineForExecution`](crate::operation::describe_state_machine_for_execution::builders::DescribeStateMachineForExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`execution_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_state_machine_for_execution::builders::DescribeStateMachineForExecutionFluentBuilder::execution_arn) / [`set_execution_arn(Option<String>)`](crate::operation::describe_state_machine_for_execution::builders::DescribeStateMachineForExecutionFluentBuilder::set_execution_arn): <p>The Amazon Resource Name (ARN) of the execution you want state machine information for.</p>
    /// - On success, responds with [`DescribeStateMachineForExecutionOutput`](crate::operation::describe_state_machine_for_execution::DescribeStateMachineForExecutionOutput) with field(s):
    ///   - [`state_machine_arn(Option<String>)`](crate::operation::describe_state_machine_for_execution::DescribeStateMachineForExecutionOutput::state_machine_arn): <p>The Amazon Resource Name (ARN) of the state machine associated with the execution.</p>
    ///   - [`name(Option<String>)`](crate::operation::describe_state_machine_for_execution::DescribeStateMachineForExecutionOutput::name): <p>The name of the state machine associated with the execution.</p>
    ///   - [`definition(Option<String>)`](crate::operation::describe_state_machine_for_execution::DescribeStateMachineForExecutionOutput::definition): <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
    ///   - [`role_arn(Option<String>)`](crate::operation::describe_state_machine_for_execution::DescribeStateMachineForExecutionOutput::role_arn): <p>The Amazon Resource Name (ARN) of the IAM role of the State Machine for the execution. </p>
    ///   - [`update_date(Option<DateTime>)`](crate::operation::describe_state_machine_for_execution::DescribeStateMachineForExecutionOutput::update_date): <p>The date and time the state machine associated with an execution was updated. For a newly created state machine, this is the creation date.</p>
    ///   - [`logging_configuration(Option<LoggingConfiguration>)`](crate::operation::describe_state_machine_for_execution::DescribeStateMachineForExecutionOutput::logging_configuration): <p>The <code>LoggingConfiguration</code> data type is used to set CloudWatch Logs options.</p>
    ///   - [`tracing_configuration(Option<TracingConfiguration>)`](crate::operation::describe_state_machine_for_execution::DescribeStateMachineForExecutionOutput::tracing_configuration): <p>Selects whether X-Ray tracing is enabled.</p>
    ///   - [`map_run_arn(Option<String>)`](crate::operation::describe_state_machine_for_execution::DescribeStateMachineForExecutionOutput::map_run_arn): <p>The Amazon Resource Name (ARN) of the Map Run that started the child workflow execution. This field is returned only if the <code>executionArn</code> is a child workflow execution that was started by a Distributed Map state.</p>
    ///   - [`label(Option<String>)`](crate::operation::describe_state_machine_for_execution::DescribeStateMachineForExecutionOutput::label): <p>A user-defined or an auto-generated string that identifies a <code>Map</code> state. This ﬁeld is returned only if the <code>executionArn</code> is a child workflow execution that was started by a Distributed Map state.</p>
    /// - On failure, responds with [`SdkError<DescribeStateMachineForExecutionError>`](crate::operation::describe_state_machine_for_execution::DescribeStateMachineForExecutionError)
    pub fn describe_state_machine_for_execution(&self) -> crate::operation::describe_state_machine_for_execution::builders::DescribeStateMachineForExecutionFluentBuilder{
        crate::operation::describe_state_machine_for_execution::builders::DescribeStateMachineForExecutionFluentBuilder::new(self.handle.clone())
    }
}
