// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListEphemerides`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`satellite_id(impl ::std::convert::Into<String>)`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::satellite_id) / [`set_satellite_id(Option<String>)`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::set_satellite_id): <p>The AWS Ground Station satellite ID to list ephemeris for.</p>
    ///   - [`start_time(DateTime)`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::start_time) / [`set_start_time(Option<DateTime>)`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::set_start_time): <p>The start time to list in UTC. The operation will return an ephemeris if its expiration time is within the time range defined by the <code>startTime</code> and <code>endTime</code>.</p>
    ///   - [`end_time(DateTime)`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::end_time) / [`set_end_time(Option<DateTime>)`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::set_end_time): <p>The end time to list in UTC. The operation will return an ephemeris if its expiration time is within the time range defined by the <code>startTime</code> and <code>endTime</code>.</p>
    ///   - [`status_list(Vec<EphemerisStatus>)`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::status_list) / [`set_status_list(Option<Vec<EphemerisStatus>>)`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::set_status_list): <p>The list of ephemeris status to return.</p>
    ///   - [`max_results(i32)`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::set_max_results): <p>Maximum number of ephemerides to return.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::set_next_token): <p>Pagination token.</p>
    /// - On success, responds with [`ListEphemeridesOutput`](crate::operation::list_ephemerides::ListEphemeridesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_ephemerides::ListEphemeridesOutput::next_token): <p>Pagination token.</p>
    ///   - [`ephemerides(Option<Vec<EphemerisItem>>)`](crate::operation::list_ephemerides::ListEphemeridesOutput::ephemerides): <p>List of ephemerides.</p>
    /// - On failure, responds with [`SdkError<ListEphemeridesError>`](crate::operation::list_ephemerides::ListEphemeridesError)
    pub fn list_ephemerides(
        &self,
    ) -> crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder {
        crate::operation::list_ephemerides::builders::ListEphemeridesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
