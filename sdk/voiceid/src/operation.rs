// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Creates a domain that contains all Amazon Connect Voice ID data, such as speakers, fraudsters, customer
/// audio, and voiceprints.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateDomain {
    _private: (),
}
impl CreateDomain {
    /// Creates a new builder-style object to manufacture [`CreateDomainInput`](crate::input::CreateDomainInput)
    pub fn builder() -> crate::input::create_domain_input::Builder {
        crate::input::create_domain_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateDomain {
    type Output =
        std::result::Result<crate::output::CreateDomainOutput, crate::error::CreateDomainError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_domain_error(response)
        } else {
            crate::operation_deser::parse_create_domain_response(response)
        }
    }
}

/// <p>Deletes the specified domain from the Amazon Connect Voice ID system.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteDomain {
    _private: (),
}
impl DeleteDomain {
    /// Creates a new builder-style object to manufacture [`DeleteDomainInput`](crate::input::DeleteDomainInput)
    pub fn builder() -> crate::input::delete_domain_input::Builder {
        crate::input::delete_domain_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteDomain {
    type Output =
        std::result::Result<crate::output::DeleteDomainOutput, crate::error::DeleteDomainError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_domain_error(response)
        } else {
            crate::operation_deser::parse_delete_domain_response(response)
        }
    }
}

/// <p>Deletes the specified fraudster from the Amazon Connect Voice ID system.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteFraudster {
    _private: (),
}
impl DeleteFraudster {
    /// Creates a new builder-style object to manufacture [`DeleteFraudsterInput`](crate::input::DeleteFraudsterInput)
    pub fn builder() -> crate::input::delete_fraudster_input::Builder {
        crate::input::delete_fraudster_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteFraudster {
    type Output = std::result::Result<
        crate::output::DeleteFraudsterOutput,
        crate::error::DeleteFraudsterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_fraudster_error(response)
        } else {
            crate::operation_deser::parse_delete_fraudster_response(response)
        }
    }
}

/// <p>Deletes the specified speaker from the Amazon Connect Voice ID system.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSpeaker {
    _private: (),
}
impl DeleteSpeaker {
    /// Creates a new builder-style object to manufacture [`DeleteSpeakerInput`](crate::input::DeleteSpeakerInput)
    pub fn builder() -> crate::input::delete_speaker_input::Builder {
        crate::input::delete_speaker_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteSpeaker {
    type Output =
        std::result::Result<crate::output::DeleteSpeakerOutput, crate::error::DeleteSpeakerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_speaker_error(response)
        } else {
            crate::operation_deser::parse_delete_speaker_response(response)
        }
    }
}

/// <p>Describes the specified domain.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeDomain {
    _private: (),
}
impl DescribeDomain {
    /// Creates a new builder-style object to manufacture [`DescribeDomainInput`](crate::input::DescribeDomainInput)
    pub fn builder() -> crate::input::describe_domain_input::Builder {
        crate::input::describe_domain_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeDomain {
    type Output =
        std::result::Result<crate::output::DescribeDomainOutput, crate::error::DescribeDomainError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_domain_error(response)
        } else {
            crate::operation_deser::parse_describe_domain_response(response)
        }
    }
}

/// <p>Describes the specified fraudster.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeFraudster {
    _private: (),
}
impl DescribeFraudster {
    /// Creates a new builder-style object to manufacture [`DescribeFraudsterInput`](crate::input::DescribeFraudsterInput)
    pub fn builder() -> crate::input::describe_fraudster_input::Builder {
        crate::input::describe_fraudster_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeFraudster {
    type Output = std::result::Result<
        crate::output::DescribeFraudsterOutput,
        crate::error::DescribeFraudsterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_fraudster_error(response)
        } else {
            crate::operation_deser::parse_describe_fraudster_response(response)
        }
    }
}

/// <p>Describes the specified fraudster registration job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeFraudsterRegistrationJob {
    _private: (),
}
impl DescribeFraudsterRegistrationJob {
    /// Creates a new builder-style object to manufacture [`DescribeFraudsterRegistrationJobInput`](crate::input::DescribeFraudsterRegistrationJobInput)
    pub fn builder() -> crate::input::describe_fraudster_registration_job_input::Builder {
        crate::input::describe_fraudster_registration_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeFraudsterRegistrationJob {
    type Output = std::result::Result<
        crate::output::DescribeFraudsterRegistrationJobOutput,
        crate::error::DescribeFraudsterRegistrationJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_fraudster_registration_job_error(response)
        } else {
            crate::operation_deser::parse_describe_fraudster_registration_job_response(response)
        }
    }
}

/// <p>Describes the specified speaker.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeSpeaker {
    _private: (),
}
impl DescribeSpeaker {
    /// Creates a new builder-style object to manufacture [`DescribeSpeakerInput`](crate::input::DescribeSpeakerInput)
    pub fn builder() -> crate::input::describe_speaker_input::Builder {
        crate::input::describe_speaker_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeSpeaker {
    type Output = std::result::Result<
        crate::output::DescribeSpeakerOutput,
        crate::error::DescribeSpeakerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_speaker_error(response)
        } else {
            crate::operation_deser::parse_describe_speaker_response(response)
        }
    }
}

/// <p>Describes the specified speaker enrollment job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeSpeakerEnrollmentJob {
    _private: (),
}
impl DescribeSpeakerEnrollmentJob {
    /// Creates a new builder-style object to manufacture [`DescribeSpeakerEnrollmentJobInput`](crate::input::DescribeSpeakerEnrollmentJobInput)
    pub fn builder() -> crate::input::describe_speaker_enrollment_job_input::Builder {
        crate::input::describe_speaker_enrollment_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeSpeakerEnrollmentJob {
    type Output = std::result::Result<
        crate::output::DescribeSpeakerEnrollmentJobOutput,
        crate::error::DescribeSpeakerEnrollmentJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_speaker_enrollment_job_error(response)
        } else {
            crate::operation_deser::parse_describe_speaker_enrollment_job_response(response)
        }
    }
}

/// <p>Evaluates a specified session based on audio data accumulated during a streaming Amazon Connect Voice
/// ID call.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct EvaluateSession {
    _private: (),
}
impl EvaluateSession {
    /// Creates a new builder-style object to manufacture [`EvaluateSessionInput`](crate::input::EvaluateSessionInput)
    pub fn builder() -> crate::input::evaluate_session_input::Builder {
        crate::input::evaluate_session_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for EvaluateSession {
    type Output = std::result::Result<
        crate::output::EvaluateSessionOutput,
        crate::error::EvaluateSessionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_evaluate_session_error(response)
        } else {
            crate::operation_deser::parse_evaluate_session_response(response)
        }
    }
}

/// <p>Lists all the domains in the Amazon Web Services account.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDomains {
    _private: (),
}
impl ListDomains {
    /// Creates a new builder-style object to manufacture [`ListDomainsInput`](crate::input::ListDomainsInput)
    pub fn builder() -> crate::input::list_domains_input::Builder {
        crate::input::list_domains_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListDomains {
    type Output =
        std::result::Result<crate::output::ListDomainsOutput, crate::error::ListDomainsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_domains_error(response)
        } else {
            crate::operation_deser::parse_list_domains_response(response)
        }
    }
}

/// <p>Lists all the fraudster registration jobs in the domain with the given <code>JobStatus</code>.
/// If <code>JobStatus</code> is not provided, this lists all fraudster registration jobs in the given
/// domain.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListFraudsterRegistrationJobs {
    _private: (),
}
impl ListFraudsterRegistrationJobs {
    /// Creates a new builder-style object to manufacture [`ListFraudsterRegistrationJobsInput`](crate::input::ListFraudsterRegistrationJobsInput)
    pub fn builder() -> crate::input::list_fraudster_registration_jobs_input::Builder {
        crate::input::list_fraudster_registration_jobs_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListFraudsterRegistrationJobs {
    type Output = std::result::Result<
        crate::output::ListFraudsterRegistrationJobsOutput,
        crate::error::ListFraudsterRegistrationJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_fraudster_registration_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_fraudster_registration_jobs_response(response)
        }
    }
}

/// <p>Lists all the speaker enrollment jobs in the domain with the specified <code>JobStatus</code>. If
/// <code>JobStatus</code> is not provided, this lists all jobs with all possible speaker enrollment job
/// statuses.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListSpeakerEnrollmentJobs {
    _private: (),
}
impl ListSpeakerEnrollmentJobs {
    /// Creates a new builder-style object to manufacture [`ListSpeakerEnrollmentJobsInput`](crate::input::ListSpeakerEnrollmentJobsInput)
    pub fn builder() -> crate::input::list_speaker_enrollment_jobs_input::Builder {
        crate::input::list_speaker_enrollment_jobs_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListSpeakerEnrollmentJobs {
    type Output = std::result::Result<
        crate::output::ListSpeakerEnrollmentJobsOutput,
        crate::error::ListSpeakerEnrollmentJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_speaker_enrollment_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_speaker_enrollment_jobs_response(response)
        }
    }
}

/// <p>Lists all speakers in a specified domain.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListSpeakers {
    _private: (),
}
impl ListSpeakers {
    /// Creates a new builder-style object to manufacture [`ListSpeakersInput`](crate::input::ListSpeakersInput)
    pub fn builder() -> crate::input::list_speakers_input::Builder {
        crate::input::list_speakers_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListSpeakers {
    type Output =
        std::result::Result<crate::output::ListSpeakersOutput, crate::error::ListSpeakersError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_speakers_error(response)
        } else {
            crate::operation_deser::parse_list_speakers_response(response)
        }
    }
}

/// <p>Lists all tags associated with a specified Voice ID resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// <p>Opts out a speaker from Voice ID system. A speaker can be opted out regardless of whether or not they
/// already exist in the system. If they don't yet exist, a new speaker is created in an opted out state.
/// If they already exist, their existing status is overridden and they are opted out. Enrollment and
/// evaluation authentication requests are rejected for opted out speakers, and opted out speakers have
/// no voice embeddings stored in the system.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct OptOutSpeaker {
    _private: (),
}
impl OptOutSpeaker {
    /// Creates a new builder-style object to manufacture [`OptOutSpeakerInput`](crate::input::OptOutSpeakerInput)
    pub fn builder() -> crate::input::opt_out_speaker_input::Builder {
        crate::input::opt_out_speaker_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for OptOutSpeaker {
    type Output =
        std::result::Result<crate::output::OptOutSpeakerOutput, crate::error::OptOutSpeakerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_opt_out_speaker_error(response)
        } else {
            crate::operation_deser::parse_opt_out_speaker_response(response)
        }
    }
}

/// <p>Starts a new batch fraudster registration job using provided details.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartFraudsterRegistrationJob {
    _private: (),
}
impl StartFraudsterRegistrationJob {
    /// Creates a new builder-style object to manufacture [`StartFraudsterRegistrationJobInput`](crate::input::StartFraudsterRegistrationJobInput)
    pub fn builder() -> crate::input::start_fraudster_registration_job_input::Builder {
        crate::input::start_fraudster_registration_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartFraudsterRegistrationJob {
    type Output = std::result::Result<
        crate::output::StartFraudsterRegistrationJobOutput,
        crate::error::StartFraudsterRegistrationJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_fraudster_registration_job_error(response)
        } else {
            crate::operation_deser::parse_start_fraudster_registration_job_response(response)
        }
    }
}

/// <p>Starts a new batch speaker enrollment job using specified details.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartSpeakerEnrollmentJob {
    _private: (),
}
impl StartSpeakerEnrollmentJob {
    /// Creates a new builder-style object to manufacture [`StartSpeakerEnrollmentJobInput`](crate::input::StartSpeakerEnrollmentJobInput)
    pub fn builder() -> crate::input::start_speaker_enrollment_job_input::Builder {
        crate::input::start_speaker_enrollment_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartSpeakerEnrollmentJob {
    type Output = std::result::Result<
        crate::output::StartSpeakerEnrollmentJobOutput,
        crate::error::StartSpeakerEnrollmentJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_speaker_enrollment_job_error(response)
        } else {
            crate::operation_deser::parse_start_speaker_enrollment_job_response(response)
        }
    }
}

/// <p>Tags an Amazon Connect Voice ID resource with the provided list of tags.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// <p>Removes specified tags from a specified Amazon Connect Voice ID resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// <p>Updates the specified domain. This API has clobber behavior, and clears and replaces all attributes.
/// If an optional field, such as 'Description' is not provided, it is removed from the domain.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateDomain {
    _private: (),
}
impl UpdateDomain {
    /// Creates a new builder-style object to manufacture [`UpdateDomainInput`](crate::input::UpdateDomainInput)
    pub fn builder() -> crate::input::update_domain_input::Builder {
        crate::input::update_domain_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateDomain {
    type Output =
        std::result::Result<crate::output::UpdateDomainOutput, crate::error::UpdateDomainError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_domain_error(response)
        } else {
            crate::operation_deser::parse_update_domain_response(response)
        }
    }
}
