// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeFleetAttributes`](crate::operation::describe_fleet_attributes::builders::DescribeFleetAttributesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_fleet_attributes::builders::DescribeFleetAttributesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`fleet_ids(Vec<String>)`](crate::operation::describe_fleet_attributes::builders::DescribeFleetAttributesFluentBuilder::fleet_ids) / [`set_fleet_ids(Option<Vec<String>>)`](crate::operation::describe_fleet_attributes::builders::DescribeFleetAttributesFluentBuilder::set_fleet_ids): <p>A list of unique fleet identifiers to retrieve attributes for. You can use either the fleet ID or ARN value. To retrieve attributes for all current fleets, do not include this parameter. </p>
    ///   - [`limit(i32)`](crate::operation::describe_fleet_attributes::builders::DescribeFleetAttributesFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::describe_fleet_attributes::builders::DescribeFleetAttributesFluentBuilder::set_limit): <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_fleet_attributes::builders::DescribeFleetAttributesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_fleet_attributes::builders::DescribeFleetAttributesFluentBuilder::set_next_token): <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>
    /// - On success, responds with [`DescribeFleetAttributesOutput`](crate::operation::describe_fleet_attributes::DescribeFleetAttributesOutput) with field(s):
    ///   - [`fleet_attributes(Option<Vec<FleetAttributes>>)`](crate::operation::describe_fleet_attributes::DescribeFleetAttributesOutput::fleet_attributes): <p>A collection of objects containing attribute metadata for each requested fleet ID. Attribute objects are returned only for fleets that currently exist.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_fleet_attributes::DescribeFleetAttributesOutput::next_token): <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    /// - On failure, responds with [`SdkError<DescribeFleetAttributesError>`](crate::operation::describe_fleet_attributes::DescribeFleetAttributesError)
    pub fn describe_fleet_attributes(
        &self,
    ) -> crate::operation::describe_fleet_attributes::builders::DescribeFleetAttributesFluentBuilder
    {
        crate::operation::describe_fleet_attributes::builders::DescribeFleetAttributesFluentBuilder::new(self.handle.clone())
    }
}
