// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetJourneyRuns`](crate::operation::get_journey_runs::builders::GetJourneyRunsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::get_journey_runs::builders::GetJourneyRunsFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::get_journey_runs::builders::GetJourneyRunsFluentBuilder::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    ///   - [`journey_id(impl ::std::convert::Into<String>)`](crate::operation::get_journey_runs::builders::GetJourneyRunsFluentBuilder::journey_id) / [`set_journey_id(Option<String>)`](crate::operation::get_journey_runs::builders::GetJourneyRunsFluentBuilder::set_journey_id): <p>The unique identifier for the journey.</p>
    ///   - [`page_size(impl ::std::convert::Into<String>)`](crate::operation::get_journey_runs::builders::GetJourneyRunsFluentBuilder::page_size) / [`set_page_size(Option<String>)`](crate::operation::get_journey_runs::builders::GetJourneyRunsFluentBuilder::set_page_size): <p>The maximum number of items to include in each page of a paginated response. This parameter is not supported for application, campaign, and journey metrics.</p>
    ///   - [`token(impl ::std::convert::Into<String>)`](crate::operation::get_journey_runs::builders::GetJourneyRunsFluentBuilder::token) / [`set_token(Option<String>)`](crate::operation::get_journey_runs::builders::GetJourneyRunsFluentBuilder::set_token): <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    /// - On success, responds with [`GetJourneyRunsOutput`](crate::operation::get_journey_runs::GetJourneyRunsOutput) with field(s):
    ///   - [`journey_runs_response(Option<JourneyRunsResponse>)`](crate::operation::get_journey_runs::GetJourneyRunsOutput::journey_runs_response): <p>Provides information from all runs of a journey.</p>
    /// - On failure, responds with [`SdkError<GetJourneyRunsError>`](crate::operation::get_journey_runs::GetJourneyRunsError)
    pub fn get_journey_runs(
        &self,
    ) -> crate::operation::get_journey_runs::builders::GetJourneyRunsFluentBuilder {
        crate::operation::get_journey_runs::builders::GetJourneyRunsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
