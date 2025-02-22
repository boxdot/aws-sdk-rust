// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeEventDetailsForOrganization`](crate::operation::describe_event_details_for_organization::builders::DescribeEventDetailsForOrganizationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`organization_event_detail_filters(Vec<EventAccountFilter>)`](crate::operation::describe_event_details_for_organization::builders::DescribeEventDetailsForOrganizationFluentBuilder::organization_event_detail_filters) / [`set_organization_event_detail_filters(Option<Vec<EventAccountFilter>>)`](crate::operation::describe_event_details_for_organization::builders::DescribeEventDetailsForOrganizationFluentBuilder::set_organization_event_detail_filters): <p>A set of JSON elements that includes the <code>awsAccountId</code> and the <code>eventArn</code>.</p>
    ///   - [`locale(impl ::std::convert::Into<String>)`](crate::operation::describe_event_details_for_organization::builders::DescribeEventDetailsForOrganizationFluentBuilder::locale) / [`set_locale(Option<String>)`](crate::operation::describe_event_details_for_organization::builders::DescribeEventDetailsForOrganizationFluentBuilder::set_locale): <p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>
    /// - On success, responds with [`DescribeEventDetailsForOrganizationOutput`](crate::operation::describe_event_details_for_organization::DescribeEventDetailsForOrganizationOutput) with field(s):
    ///   - [`successful_set(Option<Vec<OrganizationEventDetails>>)`](crate::operation::describe_event_details_for_organization::DescribeEventDetailsForOrganizationOutput::successful_set): <p>Information about the events that could be retrieved.</p>
    ///   - [`failed_set(Option<Vec<OrganizationEventDetailsErrorItem>>)`](crate::operation::describe_event_details_for_organization::DescribeEventDetailsForOrganizationOutput::failed_set): <p>Error messages for any events that could not be retrieved.</p>
    /// - On failure, responds with [`SdkError<DescribeEventDetailsForOrganizationError>`](crate::operation::describe_event_details_for_organization::DescribeEventDetailsForOrganizationError)
    pub fn describe_event_details_for_organization(&self) -> crate::operation::describe_event_details_for_organization::builders::DescribeEventDetailsForOrganizationFluentBuilder{
        crate::operation::describe_event_details_for_organization::builders::DescribeEventDetailsForOrganizationFluentBuilder::new(self.handle.clone())
    }
}
