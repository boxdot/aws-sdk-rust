// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribePiiEntitiesDetectionJobInput {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    #[doc(hidden)]
    pub job_id: ::std::option::Option<::std::string::String>,
}
impl DescribePiiEntitiesDetectionJobInput {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }
}
impl DescribePiiEntitiesDetectionJobInput {
    /// Creates a new builder-style object to manufacture [`DescribePiiEntitiesDetectionJobInput`](crate::operation::describe_pii_entities_detection_job::DescribePiiEntitiesDetectionJobInput).
    pub fn builder() -> crate::operation::describe_pii_entities_detection_job::builders::DescribePiiEntitiesDetectionJobInputBuilder{
        crate::operation::describe_pii_entities_detection_job::builders::DescribePiiEntitiesDetectionJobInputBuilder::default()
    }
}

/// A builder for [`DescribePiiEntitiesDetectionJobInput`](crate::operation::describe_pii_entities_detection_job::DescribePiiEntitiesDetectionJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribePiiEntitiesDetectionJobInputBuilder {
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
}
impl DescribePiiEntitiesDetectionJobInputBuilder {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribePiiEntitiesDetectionJobInput`](crate::operation::describe_pii_entities_detection_job::DescribePiiEntitiesDetectionJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_pii_entities_detection_job::DescribePiiEntitiesDetectionJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_pii_entities_detection_job::DescribePiiEntitiesDetectionJobInput {
                job_id: self.job_id
                ,
            }
        )
    }
}
