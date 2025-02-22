// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeLabel`](crate::operation::describe_label::builders::DescribeLabelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`label_group_name(impl ::std::convert::Into<String>)`](crate::operation::describe_label::builders::DescribeLabelFluentBuilder::label_group_name) / [`set_label_group_name(Option<String>)`](crate::operation::describe_label::builders::DescribeLabelFluentBuilder::set_label_group_name): <p> Returns the name of the group containing the label. </p>
    ///   - [`label_id(impl ::std::convert::Into<String>)`](crate::operation::describe_label::builders::DescribeLabelFluentBuilder::label_id) / [`set_label_id(Option<String>)`](crate::operation::describe_label::builders::DescribeLabelFluentBuilder::set_label_id): <p> Returns the ID of the label. </p>
    /// - On success, responds with [`DescribeLabelOutput`](crate::operation::describe_label::DescribeLabelOutput) with field(s):
    ///   - [`label_group_name(Option<String>)`](crate::operation::describe_label::DescribeLabelOutput::label_group_name): <p> The name of the requested label group. </p>
    ///   - [`label_group_arn(Option<String>)`](crate::operation::describe_label::DescribeLabelOutput::label_group_arn): <p> The ARN of the requested label group. </p>
    ///   - [`label_id(Option<String>)`](crate::operation::describe_label::DescribeLabelOutput::label_id): <p> The ID of the requested label. </p>
    ///   - [`start_time(Option<DateTime>)`](crate::operation::describe_label::DescribeLabelOutput::start_time): <p> The start time of the requested label. </p>
    ///   - [`end_time(Option<DateTime>)`](crate::operation::describe_label::DescribeLabelOutput::end_time): <p> The end time of the requested label. </p>
    ///   - [`rating(Option<LabelRating>)`](crate::operation::describe_label::DescribeLabelOutput::rating): <p> Indicates whether a labeled event represents an anomaly. </p>
    ///   - [`fault_code(Option<String>)`](crate::operation::describe_label::DescribeLabelOutput::fault_code): <p> Indicates the type of anomaly associated with the label. </p>  <p>Data in this field will be retained for service usage. Follow best practices for the security of your data.</p>
    ///   - [`notes(Option<String>)`](crate::operation::describe_label::DescribeLabelOutput::notes): <p>Metadata providing additional information about the label.</p>  <p>Data in this field will be retained for service usage. Follow best practices for the security of your data.</p>
    ///   - [`equipment(Option<String>)`](crate::operation::describe_label::DescribeLabelOutput::equipment): <p> Indicates that a label pertains to a particular piece of equipment. </p>
    ///   - [`created_at(Option<DateTime>)`](crate::operation::describe_label::DescribeLabelOutput::created_at): <p> The time at which the label was created. </p>
    /// - On failure, responds with [`SdkError<DescribeLabelError>`](crate::operation::describe_label::DescribeLabelError)
    pub fn describe_label(
        &self,
    ) -> crate::operation::describe_label::builders::DescribeLabelFluentBuilder {
        crate::operation::describe_label::builders::DescribeLabelFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
