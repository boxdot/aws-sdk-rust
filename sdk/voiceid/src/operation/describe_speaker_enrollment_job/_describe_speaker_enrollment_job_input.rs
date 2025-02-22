// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeSpeakerEnrollmentJobInput {
    /// <p>The identifier of the domain that contains the speaker enrollment job.</p>
    #[doc(hidden)]
    pub domain_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the speaker enrollment job you are describing.</p>
    #[doc(hidden)]
    pub job_id: ::std::option::Option<::std::string::String>,
}
impl DescribeSpeakerEnrollmentJobInput {
    /// <p>The identifier of the domain that contains the speaker enrollment job.</p>
    pub fn domain_id(&self) -> ::std::option::Option<&str> {
        self.domain_id.as_deref()
    }
    /// <p>The identifier of the speaker enrollment job you are describing.</p>
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }
}
impl DescribeSpeakerEnrollmentJobInput {
    /// Creates a new builder-style object to manufacture [`DescribeSpeakerEnrollmentJobInput`](crate::operation::describe_speaker_enrollment_job::DescribeSpeakerEnrollmentJobInput).
    pub fn builder() -> crate::operation::describe_speaker_enrollment_job::builders::DescribeSpeakerEnrollmentJobInputBuilder{
        crate::operation::describe_speaker_enrollment_job::builders::DescribeSpeakerEnrollmentJobInputBuilder::default()
    }
}

/// A builder for [`DescribeSpeakerEnrollmentJobInput`](crate::operation::describe_speaker_enrollment_job::DescribeSpeakerEnrollmentJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeSpeakerEnrollmentJobInputBuilder {
    pub(crate) domain_id: ::std::option::Option<::std::string::String>,
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
}
impl DescribeSpeakerEnrollmentJobInputBuilder {
    /// <p>The identifier of the domain that contains the speaker enrollment job.</p>
    pub fn domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the domain that contains the speaker enrollment job.</p>
    pub fn set_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_id = input;
        self
    }
    /// <p>The identifier of the speaker enrollment job you are describing.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the speaker enrollment job you are describing.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeSpeakerEnrollmentJobInput`](crate::operation::describe_speaker_enrollment_job::DescribeSpeakerEnrollmentJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_speaker_enrollment_job::DescribeSpeakerEnrollmentJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_speaker_enrollment_job::DescribeSpeakerEnrollmentJobInput {
                domain_id: self.domain_id,
                job_id: self.job_id,
            },
        )
    }
}
