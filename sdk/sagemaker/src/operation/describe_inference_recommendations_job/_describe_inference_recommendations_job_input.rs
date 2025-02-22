// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeInferenceRecommendationsJobInput {
    /// <p>The name of the job. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p>
    #[doc(hidden)]
    pub job_name: ::std::option::Option<::std::string::String>,
}
impl DescribeInferenceRecommendationsJobInput {
    /// <p>The name of the job. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p>
    pub fn job_name(&self) -> ::std::option::Option<&str> {
        self.job_name.as_deref()
    }
}
impl DescribeInferenceRecommendationsJobInput {
    /// Creates a new builder-style object to manufacture [`DescribeInferenceRecommendationsJobInput`](crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobInput).
    pub fn builder() -> crate::operation::describe_inference_recommendations_job::builders::DescribeInferenceRecommendationsJobInputBuilder{
        crate::operation::describe_inference_recommendations_job::builders::DescribeInferenceRecommendationsJobInputBuilder::default()
    }
}

/// A builder for [`DescribeInferenceRecommendationsJobInput`](crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeInferenceRecommendationsJobInputBuilder {
    pub(crate) job_name: ::std::option::Option<::std::string::String>,
}
impl DescribeInferenceRecommendationsJobInputBuilder {
    /// <p>The name of the job. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p>
    pub fn job_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the job. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p>
    pub fn set_job_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeInferenceRecommendationsJobInput`](crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobInput {
                job_name: self.job_name
                ,
            }
        )
    }
}
