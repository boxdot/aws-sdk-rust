// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`QueryLineage`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`start_arns(Vec<String>)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::start_arns) / [`set_start_arns(Option<Vec<String>>)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::set_start_arns): <p>A list of resource Amazon Resource Name (ARN) that represent the starting point for your lineage query.</p>
    ///   - [`direction(Direction)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::direction) / [`set_direction(Option<Direction>)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::set_direction): <p>Associations between lineage entities have a direction. This parameter determines the direction from the StartArn(s) that the query traverses.</p>
    ///   - [`include_edges(bool)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::include_edges) / [`set_include_edges(Option<bool>)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::set_include_edges): <p> Setting this value to <code>True</code> retrieves not only the entities of interest but also the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/lineage-tracking-entities.html">Associations</a> and lineage entities on the path. Set to <code>False</code> to only return lineage entities that match your query.</p>
    ///   - [`filters(QueryFilters)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::filters) / [`set_filters(Option<QueryFilters>)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::set_filters): <p>A set of filtering parameters that allow you to specify which entities should be returned.</p>  <ul>   <li> <p>Properties - Key-value pairs to match on the lineage entities' properties.</p> </li>   <li> <p>LineageTypes - A set of lineage entity types to match on. For example: <code>TrialComponent</code>, <code>Artifact</code>, or <code>Context</code>.</p> </li>   <li> <p>CreatedBefore - Filter entities created before this date.</p> </li>   <li> <p>ModifiedBefore - Filter entities modified before this date.</p> </li>   <li> <p>ModifiedAfter - Filter entities modified after this date.</p> </li>  </ul>
    ///   - [`max_depth(i32)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::max_depth) / [`set_max_depth(Option<i32>)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::set_max_depth): <p>The maximum depth in lineage relationships from the <code>StartArns</code> that are traversed. Depth is a measure of the number of <code>Associations</code> from the <code>StartArn</code> entity to the matched results.</p>
    ///   - [`max_results(i32)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::set_max_results): <p>Limits the number of vertices in the results. Use the <code>NextToken</code> in a response to to retrieve the next page of results.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::query_lineage::builders::QueryLineageFluentBuilder::set_next_token): <p>Limits the number of vertices in the request. Use the <code>NextToken</code> in a response to to retrieve the next page of results.</p>
    /// - On success, responds with [`QueryLineageOutput`](crate::operation::query_lineage::QueryLineageOutput) with field(s):
    ///   - [`vertices(Option<Vec<Vertex>>)`](crate::operation::query_lineage::QueryLineageOutput::vertices): <p>A list of vertices connected to the start entity(ies) in the lineage graph.</p>
    ///   - [`edges(Option<Vec<Edge>>)`](crate::operation::query_lineage::QueryLineageOutput::edges): <p>A list of edges that connect vertices in the response.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::query_lineage::QueryLineageOutput::next_token): <p>Limits the number of vertices in the response. Use the <code>NextToken</code> in a response to to retrieve the next page of results.</p>
    /// - On failure, responds with [`SdkError<QueryLineageError>`](crate::operation::query_lineage::QueryLineageError)
    pub fn query_lineage(
        &self,
    ) -> crate::operation::query_lineage::builders::QueryLineageFluentBuilder {
        crate::operation::query_lineage::builders::QueryLineageFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
