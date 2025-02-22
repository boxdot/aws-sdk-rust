// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDevicesJobs`](crate::operation::list_devices_jobs::builders::ListDevicesJobsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_devices_jobs::builders::ListDevicesJobsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`device_id(impl ::std::convert::Into<String>)`](crate::operation::list_devices_jobs::builders::ListDevicesJobsFluentBuilder::device_id) / [`set_device_id(Option<String>)`](crate::operation::list_devices_jobs::builders::ListDevicesJobsFluentBuilder::set_device_id): <p>Filter results by the job's target device ID.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_devices_jobs::builders::ListDevicesJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_devices_jobs::builders::ListDevicesJobsFluentBuilder::set_next_token): <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_devices_jobs::builders::ListDevicesJobsFluentBuilder::max_results) / [`set_max_results(i32)`](crate::operation::list_devices_jobs::builders::ListDevicesJobsFluentBuilder::set_max_results): <p>The maximum number of device jobs to return in one page of results.</p>
    /// - On success, responds with [`ListDevicesJobsOutput`](crate::operation::list_devices_jobs::ListDevicesJobsOutput) with field(s):
    ///   - [`device_jobs(Option<Vec<DeviceJob>>)`](crate::operation::list_devices_jobs::ListDevicesJobsOutput::device_jobs): <p>A list of jobs.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_devices_jobs::ListDevicesJobsOutput::next_token): <p>A pagination token that's included if more results are available.</p>
    /// - On failure, responds with [`SdkError<ListDevicesJobsError>`](crate::operation::list_devices_jobs::ListDevicesJobsError)
    pub fn list_devices_jobs(
        &self,
    ) -> crate::operation::list_devices_jobs::builders::ListDevicesJobsFluentBuilder {
        crate::operation::list_devices_jobs::builders::ListDevicesJobsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
