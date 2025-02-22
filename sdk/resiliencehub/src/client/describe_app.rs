// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeApp`](crate::operation::describe_app::builders::DescribeAppFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_app::builders::DescribeAppFluentBuilder::app_arn) / [`set_app_arn(Option<String>)`](crate::operation::describe_app::builders::DescribeAppFluentBuilder::set_app_arn): <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    /// - On success, responds with [`DescribeAppOutput`](crate::operation::describe_app::DescribeAppOutput) with field(s):
    ///   - [`app(Option<App>)`](crate::operation::describe_app::DescribeAppOutput::app): <p>The specified application, returned as an object with details including compliance status, creation time, description, resiliency score, and more.</p>
    /// - On failure, responds with [`SdkError<DescribeAppError>`](crate::operation::describe_app::DescribeAppError)
    pub fn describe_app(
        &self,
    ) -> crate::operation::describe_app::builders::DescribeAppFluentBuilder {
        crate::operation::describe_app::builders::DescribeAppFluentBuilder::new(self.handle.clone())
    }
}
