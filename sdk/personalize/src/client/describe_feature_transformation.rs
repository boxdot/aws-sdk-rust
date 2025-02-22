// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeFeatureTransformation`](crate::operation::describe_feature_transformation::builders::DescribeFeatureTransformationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`feature_transformation_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_feature_transformation::builders::DescribeFeatureTransformationFluentBuilder::feature_transformation_arn) / [`set_feature_transformation_arn(Option<String>)`](crate::operation::describe_feature_transformation::builders::DescribeFeatureTransformationFluentBuilder::set_feature_transformation_arn): <p>The Amazon Resource Name (ARN) of the feature transformation to describe.</p>
    /// - On success, responds with [`DescribeFeatureTransformationOutput`](crate::operation::describe_feature_transformation::DescribeFeatureTransformationOutput) with field(s):
    ///   - [`feature_transformation(Option<FeatureTransformation>)`](crate::operation::describe_feature_transformation::DescribeFeatureTransformationOutput::feature_transformation): <p>A listing of the FeatureTransformation properties.</p>
    /// - On failure, responds with [`SdkError<DescribeFeatureTransformationError>`](crate::operation::describe_feature_transformation::DescribeFeatureTransformationError)
    pub fn describe_feature_transformation(&self) -> crate::operation::describe_feature_transformation::builders::DescribeFeatureTransformationFluentBuilder{
        crate::operation::describe_feature_transformation::builders::DescribeFeatureTransformationFluentBuilder::new(self.handle.clone())
    }
}
