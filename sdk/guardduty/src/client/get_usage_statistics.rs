// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetUsageStatistics`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`detector_id(impl ::std::convert::Into<String>)`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::detector_id) / [`set_detector_id(Option<String>)`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::set_detector_id): <p>The ID of the detector that specifies the GuardDuty service whose usage statistics you want to retrieve.</p>
    ///   - [`usage_statistic_type(UsageStatisticType)`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::usage_statistic_type) / [`set_usage_statistic_type(Option<UsageStatisticType>)`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::set_usage_statistic_type): <p>The type of usage statistics to retrieve.</p>
    ///   - [`usage_criteria(UsageCriteria)`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::usage_criteria) / [`set_usage_criteria(Option<UsageCriteria>)`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::set_usage_criteria): <p>Represents the criteria used for querying usage.</p>
    ///   - [`unit(impl ::std::convert::Into<String>)`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::unit) / [`set_unit(Option<String>)`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::set_unit): <p>The currency unit you would like to view your usage statistics in. Current valid values are USD.</p>
    ///   - [`max_results(i32)`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::set_max_results): <p>The maximum number of results to return in the response.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::set_next_token): <p>A token to use for paginating results that are returned in the response. Set the value of this parameter to null for the first request to a list action. For subsequent calls, use the NextToken value returned from the previous request to continue listing results after the first page.</p>
    /// - On success, responds with [`GetUsageStatisticsOutput`](crate::operation::get_usage_statistics::GetUsageStatisticsOutput) with field(s):
    ///   - [`usage_statistics(Option<UsageStatistics>)`](crate::operation::get_usage_statistics::GetUsageStatisticsOutput::usage_statistics): <p>The usage statistics object. If a UsageStatisticType was provided, the objects representing other types will be null.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_usage_statistics::GetUsageStatisticsOutput::next_token): <p>The pagination parameter to be used on the next list operation to retrieve more items.</p>
    /// - On failure, responds with [`SdkError<GetUsageStatisticsError>`](crate::operation::get_usage_statistics::GetUsageStatisticsError)
    pub fn get_usage_statistics(
        &self,
    ) -> crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder {
        crate::operation::get_usage_statistics::builders::GetUsageStatisticsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
