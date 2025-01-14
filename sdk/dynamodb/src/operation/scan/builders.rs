// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::scan::_scan_output::ScanOutputBuilder;

pub use crate::operation::scan::_scan_input::ScanInputBuilder;

/// Fluent builder constructing a request to `Scan`.
///
/// <p>The <code>Scan</code> operation returns one or more items and item attributes by accessing every item in a table or a secondary index. To have DynamoDB return fewer items, you can provide a <code>FilterExpression</code> operation.</p>
/// <p>If the total number of scanned items exceeds the maximum dataset size limit of 1 MB, the scan stops and results are returned to the user as a <code>LastEvaluatedKey</code> value to continue the scan in a subsequent operation. The results also include the number of items exceeding the limit. A scan can result in no table data meeting the filter criteria. </p>
/// <p>A single <code>Scan</code> operation reads up to the maximum number of items set (if using the <code>Limit</code> parameter) or a maximum of 1 MB of data and then apply any filtering to the results using <code>FilterExpression</code>. If <code>LastEvaluatedKey</code> is present in the response, you need to paginate the result set. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Scan.html#Scan.Pagination">Paginating the Results</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p>
/// <p> <code>Scan</code> operations proceed sequentially; however, for faster performance on a large table or secondary index, applications can request a parallel <code>Scan</code> operation by providing the <code>Segment</code> and <code>TotalSegments</code> parameters. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Scan.html#Scan.ParallelScan">Parallel Scan</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
/// <p> <code>Scan</code> uses eventually consistent reads when accessing the data in a table; therefore, the result set might not include the changes to data in the table immediately before the operation began. If you need a consistent copy of the data, as of the time that the <code>Scan</code> begins, you can set the <code>ConsistentRead</code> parameter to <code>true</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ScanFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::scan::builders::ScanInputBuilder,
}
impl ScanFluentBuilder {
    /// Creates a new `Scan`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::scan::Scan,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::scan::ScanError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::scan::ScanOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::scan::ScanError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::scan::ScanOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::scan::ScanError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::scan::Scan,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::scan::ScanError>,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::scan::paginator::ScanPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::scan::paginator::ScanPaginator {
        crate::operation::scan::paginator::ScanPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the table containing the requested items; or, if you provide <code>IndexName</code>, the name of the table to which that index belongs.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table containing the requested items; or, if you provide <code>IndexName</code>, the name of the table to which that index belongs.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>The name of a secondary index to scan. This index can be any local secondary index or global secondary index. Note that if you use the <code>IndexName</code> parameter, you must also provide <code>TableName</code>.</p>
    pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.index_name(input.into());
        self
    }
    /// <p>The name of a secondary index to scan. This index can be any local secondary index or global secondary index. Note that if you use the <code>IndexName</code> parameter, you must also provide <code>TableName</code>.</p>
    pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_index_name(input);
        self
    }
    /// Appends an item to `AttributesToGet`.
    ///
    /// To override the contents of this collection use [`set_attributes_to_get`](Self::set_attributes_to_get).
    ///
    /// <p>This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn attributes_to_get(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.attributes_to_get(input.into());
        self
    }
    /// <p>This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_attributes_to_get(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_attributes_to_get(input);
        self
    }
    /// <p>The maximum number of items to evaluate (not necessarily the number of matching items). If DynamoDB processes the number of items up to the limit while processing the results, it stops the operation and returns the matching values up to that point, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation, so that you can pick up where you left off. Also, if the processed dataset size exceeds 1 MB before DynamoDB reaches this limit, it stops the operation and returns the matching values up to the limit, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation to continue the operation. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html">Working with Queries</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of items to evaluate (not necessarily the number of matching items). If DynamoDB processes the number of items up to the limit while processing the results, it stops the operation and returns the matching values up to that point, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation, so that you can pick up where you left off. Also, if the processed dataset size exceeds 1 MB before DynamoDB reaches this limit, it stops the operation and returns the matching values up to the limit, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation to continue the operation. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html">Working with Queries</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The attributes to be returned in the result. You can retrieve all item attributes, specific item attributes, the count of matching items, or in the case of an index, some or all of the attributes projected into the index.</p>
    /// <ul>
    /// <li> <p> <code>ALL_ATTRIBUTES</code> - Returns all of the item attributes from the specified table or index. If you query a local secondary index, then for each matching item in the index, DynamoDB fetches the entire item from the parent table. If the index is configured to project all item attributes, then all of the data can be obtained from the local secondary index, and no fetching is required.</p> </li>
    /// <li> <p> <code>ALL_PROJECTED_ATTRIBUTES</code> - Allowed only when querying an index. Retrieves all attributes that have been projected into the index. If the index is configured to project all attributes, this return value is equivalent to specifying <code>ALL_ATTRIBUTES</code>.</p> </li>
    /// <li> <p> <code>COUNT</code> - Returns the number of matching items, rather than the matching items themselves. Note that this uses the same quantity of read capacity units as getting the items, and is subject to the same item size calculations.</p> </li>
    /// <li> <p> <code>SPECIFIC_ATTRIBUTES</code> - Returns only the attributes listed in <code>ProjectionExpression</code>. This return value is equivalent to specifying <code>ProjectionExpression</code> without specifying any value for <code>Select</code>.</p> <p>If you query or scan a local secondary index and request only attributes that are projected into that index, the operation reads only the index and not the table. If any of the requested attributes are not projected into the local secondary index, DynamoDB fetches each of these attributes from the parent table. This extra fetching incurs additional throughput cost and latency.</p> <p>If you query or scan a global secondary index, you can only request attributes that are projected into the index. Global secondary index queries cannot fetch attributes from the parent table.</p> </li>
    /// </ul>
    /// <p>If neither <code>Select</code> nor <code>ProjectionExpression</code> are specified, DynamoDB defaults to <code>ALL_ATTRIBUTES</code> when accessing a table, and <code>ALL_PROJECTED_ATTRIBUTES</code> when accessing an index. You cannot use both <code>Select</code> and <code>ProjectionExpression</code> together in a single request, unless the value for <code>Select</code> is <code>SPECIFIC_ATTRIBUTES</code>. (This usage is equivalent to specifying <code>ProjectionExpression</code> without any value for <code>Select</code>.)</p> <note>
    /// <p>If you use the <code>ProjectionExpression</code> parameter, then the value for <code>Select</code> can only be <code>SPECIFIC_ATTRIBUTES</code>. Any other value for <code>Select</code> will return an error.</p>
    /// </note>
    pub fn select(mut self, input: crate::types::Select) -> Self {
        self.inner = self.inner.select(input);
        self
    }
    /// <p>The attributes to be returned in the result. You can retrieve all item attributes, specific item attributes, the count of matching items, or in the case of an index, some or all of the attributes projected into the index.</p>
    /// <ul>
    /// <li> <p> <code>ALL_ATTRIBUTES</code> - Returns all of the item attributes from the specified table or index. If you query a local secondary index, then for each matching item in the index, DynamoDB fetches the entire item from the parent table. If the index is configured to project all item attributes, then all of the data can be obtained from the local secondary index, and no fetching is required.</p> </li>
    /// <li> <p> <code>ALL_PROJECTED_ATTRIBUTES</code> - Allowed only when querying an index. Retrieves all attributes that have been projected into the index. If the index is configured to project all attributes, this return value is equivalent to specifying <code>ALL_ATTRIBUTES</code>.</p> </li>
    /// <li> <p> <code>COUNT</code> - Returns the number of matching items, rather than the matching items themselves. Note that this uses the same quantity of read capacity units as getting the items, and is subject to the same item size calculations.</p> </li>
    /// <li> <p> <code>SPECIFIC_ATTRIBUTES</code> - Returns only the attributes listed in <code>ProjectionExpression</code>. This return value is equivalent to specifying <code>ProjectionExpression</code> without specifying any value for <code>Select</code>.</p> <p>If you query or scan a local secondary index and request only attributes that are projected into that index, the operation reads only the index and not the table. If any of the requested attributes are not projected into the local secondary index, DynamoDB fetches each of these attributes from the parent table. This extra fetching incurs additional throughput cost and latency.</p> <p>If you query or scan a global secondary index, you can only request attributes that are projected into the index. Global secondary index queries cannot fetch attributes from the parent table.</p> </li>
    /// </ul>
    /// <p>If neither <code>Select</code> nor <code>ProjectionExpression</code> are specified, DynamoDB defaults to <code>ALL_ATTRIBUTES</code> when accessing a table, and <code>ALL_PROJECTED_ATTRIBUTES</code> when accessing an index. You cannot use both <code>Select</code> and <code>ProjectionExpression</code> together in a single request, unless the value for <code>Select</code> is <code>SPECIFIC_ATTRIBUTES</code>. (This usage is equivalent to specifying <code>ProjectionExpression</code> without any value for <code>Select</code>.)</p> <note>
    /// <p>If you use the <code>ProjectionExpression</code> parameter, then the value for <code>Select</code> can only be <code>SPECIFIC_ATTRIBUTES</code>. Any other value for <code>Select</code> will return an error.</p>
    /// </note>
    pub fn set_select(mut self, input: ::std::option::Option<crate::types::Select>) -> Self {
        self.inner = self.inner.set_select(input);
        self
    }
    /// Adds a key-value pair to `ScanFilter`.
    ///
    /// To override the contents of this collection use [`set_scan_filter`](Self::set_scan_filter).
    ///
    /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ScanFilter.html">ScanFilter</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn scan_filter(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::Condition,
    ) -> Self {
        self.inner = self.inner.scan_filter(k.into(), v);
        self
    }
    /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ScanFilter.html">ScanFilter</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_scan_filter(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::Condition>,
        >,
    ) -> Self {
        self.inner = self.inner.set_scan_filter(input);
        self
    }
    /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn conditional_operator(mut self, input: crate::types::ConditionalOperator) -> Self {
        self.inner = self.inner.conditional_operator(input);
        self
    }
    /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_conditional_operator(
        mut self,
        input: ::std::option::Option<crate::types::ConditionalOperator>,
    ) -> Self {
        self.inner = self.inner.set_conditional_operator(input);
        self
    }
    /// Adds a key-value pair to `ExclusiveStartKey`.
    ///
    /// To override the contents of this collection use [`set_exclusive_start_key`](Self::set_exclusive_start_key).
    ///
    /// <p>The primary key of the first item that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedKey</code> in the previous operation.</p>
    /// <p>The data type for <code>ExclusiveStartKey</code> must be String, Number or Binary. No set data types are allowed.</p>
    /// <p>In a parallel scan, a <code>Scan</code> request that includes <code>ExclusiveStartKey</code> must specify the same segment whose previous <code>Scan</code> returned the corresponding value of <code>LastEvaluatedKey</code>.</p>
    pub fn exclusive_start_key(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::AttributeValue,
    ) -> Self {
        self.inner = self.inner.exclusive_start_key(k.into(), v);
        self
    }
    /// <p>The primary key of the first item that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedKey</code> in the previous operation.</p>
    /// <p>The data type for <code>ExclusiveStartKey</code> must be String, Number or Binary. No set data types are allowed.</p>
    /// <p>In a parallel scan, a <code>Scan</code> request that includes <code>ExclusiveStartKey</code> must specify the same segment whose previous <code>Scan</code> returned the corresponding value of <code>LastEvaluatedKey</code>.</p>
    pub fn set_exclusive_start_key(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>,
        >,
    ) -> Self {
        self.inner = self.inner.set_exclusive_start_key(input);
        self
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>
    /// <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>
    /// <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>
    /// </ul>
    pub fn return_consumed_capacity(mut self, input: crate::types::ReturnConsumedCapacity) -> Self {
        self.inner = self.inner.return_consumed_capacity(input);
        self
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>
    /// <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>
    /// <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>
    /// </ul>
    pub fn set_return_consumed_capacity(
        mut self,
        input: ::std::option::Option<crate::types::ReturnConsumedCapacity>,
    ) -> Self {
        self.inner = self.inner.set_return_consumed_capacity(input);
        self
    }
    /// <p>For a parallel <code>Scan</code> request, <code>TotalSegments</code> represents the total number of segments into which the <code>Scan</code> operation will be divided. The value of <code>TotalSegments</code> corresponds to the number of application workers that will perform the parallel scan. For example, if you want to use four application threads to scan a table or an index, specify a <code>TotalSegments</code> value of 4.</p>
    /// <p>The value for <code>TotalSegments</code> must be greater than or equal to 1, and less than or equal to 1000000. If you specify a <code>TotalSegments</code> value of 1, the <code>Scan</code> operation will be sequential rather than parallel.</p>
    /// <p>If you specify <code>TotalSegments</code>, you must also specify <code>Segment</code>.</p>
    pub fn total_segments(mut self, input: i32) -> Self {
        self.inner = self.inner.total_segments(input);
        self
    }
    /// <p>For a parallel <code>Scan</code> request, <code>TotalSegments</code> represents the total number of segments into which the <code>Scan</code> operation will be divided. The value of <code>TotalSegments</code> corresponds to the number of application workers that will perform the parallel scan. For example, if you want to use four application threads to scan a table or an index, specify a <code>TotalSegments</code> value of 4.</p>
    /// <p>The value for <code>TotalSegments</code> must be greater than or equal to 1, and less than or equal to 1000000. If you specify a <code>TotalSegments</code> value of 1, the <code>Scan</code> operation will be sequential rather than parallel.</p>
    /// <p>If you specify <code>TotalSegments</code>, you must also specify <code>Segment</code>.</p>
    pub fn set_total_segments(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_total_segments(input);
        self
    }
    /// <p>For a parallel <code>Scan</code> request, <code>Segment</code> identifies an individual segment to be scanned by an application worker.</p>
    /// <p>Segment IDs are zero-based, so the first segment is always 0. For example, if you want to use four application threads to scan a table or an index, then the first thread specifies a <code>Segment</code> value of 0, the second thread specifies 1, and so on.</p>
    /// <p>The value of <code>LastEvaluatedKey</code> returned from a parallel <code>Scan</code> request must be used as <code>ExclusiveStartKey</code> with the same segment ID in a subsequent <code>Scan</code> operation.</p>
    /// <p>The value for <code>Segment</code> must be greater than or equal to 0, and less than the value provided for <code>TotalSegments</code>.</p>
    /// <p>If you provide <code>Segment</code>, you must also provide <code>TotalSegments</code>.</p>
    pub fn segment(mut self, input: i32) -> Self {
        self.inner = self.inner.segment(input);
        self
    }
    /// <p>For a parallel <code>Scan</code> request, <code>Segment</code> identifies an individual segment to be scanned by an application worker.</p>
    /// <p>Segment IDs are zero-based, so the first segment is always 0. For example, if you want to use four application threads to scan a table or an index, then the first thread specifies a <code>Segment</code> value of 0, the second thread specifies 1, and so on.</p>
    /// <p>The value of <code>LastEvaluatedKey</code> returned from a parallel <code>Scan</code> request must be used as <code>ExclusiveStartKey</code> with the same segment ID in a subsequent <code>Scan</code> operation.</p>
    /// <p>The value for <code>Segment</code> must be greater than or equal to 0, and less than the value provided for <code>TotalSegments</code>.</p>
    /// <p>If you provide <code>Segment</code>, you must also provide <code>TotalSegments</code>.</p>
    pub fn set_segment(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_segment(input);
        self
    }
    /// <p>A string that identifies one or more attributes to retrieve from the specified table or index. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p>
    /// <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn projection_expression(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.projection_expression(input.into());
        self
    }
    /// <p>A string that identifies one or more attributes to retrieve from the specified table or index. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p>
    /// <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_projection_expression(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_projection_expression(input);
        self
    }
    /// <p>A string that contains conditions that DynamoDB applies after the <code>Scan</code> operation, but before the data is returned to you. Items that do not satisfy the <code>FilterExpression</code> criteria are not returned.</p> <note>
    /// <p>A <code>FilterExpression</code> is applied after the items have already been read; the process of filtering does not consume any additional read capacity units.</p>
    /// </note>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#Query.FilterExpression">Filter Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn filter_expression(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.filter_expression(input.into());
        self
    }
    /// <p>A string that contains conditions that DynamoDB applies after the <code>Scan</code> operation, but before the data is returned to you. Items that do not satisfy the <code>FilterExpression</code> criteria are not returned.</p> <note>
    /// <p>A <code>FilterExpression</code> is applied after the items have already been read; the process of filtering does not consume any additional read capacity units.</p>
    /// </note>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#Query.FilterExpression">Filter Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_filter_expression(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_filter_expression(input);
        self
    }
    /// Adds a key-value pair to `ExpressionAttributeNames`.
    ///
    /// To override the contents of this collection use [`set_expression_attribute_names`](Self::set_expression_attribute_names).
    ///
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li>
    /// <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li>
    /// <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li>
    /// </ul>
    /// <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>
    /// <ul>
    /// <li> <p> <code>Percentile</code> </p> </li>
    /// </ul>
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li> <p> <code>{"#P":"Percentile"}</code> </p> </li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li> <p> <code>#P = :val</code> </p> </li>
    /// </ul> <note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_names(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.expression_attribute_names(k.into(), v.into());
        self
    }
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li>
    /// <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li>
    /// <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li>
    /// </ul>
    /// <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>
    /// <ul>
    /// <li> <p> <code>Percentile</code> </p> </li>
    /// </ul>
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li> <p> <code>{"#P":"Percentile"}</code> </p> </li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li> <p> <code>#P = :val</code> </p> </li>
    /// </ul> <note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_expression_attribute_names(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_expression_attribute_names(input);
        self
    }
    /// Adds a key-value pair to `ExpressionAttributeValues`.
    ///
    /// To override the contents of this collection use [`set_expression_attribute_values`](Self::set_expression_attribute_values).
    ///
    /// <p>One or more values that can be substituted in an expression.</p>
    /// <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <code>ProductStatus</code> attribute was one of the following: </p>
    /// <p> <code>Available | Backordered | Discontinued</code> </p>
    /// <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p>
    /// <p> <code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code> </p>
    /// <p>You could then use these values in an expression, such as this:</p>
    /// <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p>
    /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_values(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::AttributeValue,
    ) -> Self {
        self.inner = self.inner.expression_attribute_values(k.into(), v);
        self
    }
    /// <p>One or more values that can be substituted in an expression.</p>
    /// <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <code>ProductStatus</code> attribute was one of the following: </p>
    /// <p> <code>Available | Backordered | Discontinued</code> </p>
    /// <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p>
    /// <p> <code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code> </p>
    /// <p>You could then use these values in an expression, such as this:</p>
    /// <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p>
    /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_expression_attribute_values(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>,
        >,
    ) -> Self {
        self.inner = self.inner.set_expression_attribute_values(input);
        self
    }
    /// <p>A Boolean value that determines the read consistency model during the scan:</p>
    /// <ul>
    /// <li> <p>If <code>ConsistentRead</code> is <code>false</code>, then the data returned from <code>Scan</code> might not contain the results from other recently completed write operations (<code>PutItem</code>, <code>UpdateItem</code>, or <code>DeleteItem</code>).</p> </li>
    /// <li> <p>If <code>ConsistentRead</code> is <code>true</code>, then all of the write operations that completed before the <code>Scan</code> began are guaranteed to be contained in the <code>Scan</code> response.</p> </li>
    /// </ul>
    /// <p>The default setting for <code>ConsistentRead</code> is <code>false</code>.</p>
    /// <p>The <code>ConsistentRead</code> parameter is not supported on global secondary indexes. If you scan a global secondary index with <code>ConsistentRead</code> set to true, you will receive a <code>ValidationException</code>.</p>
    pub fn consistent_read(mut self, input: bool) -> Self {
        self.inner = self.inner.consistent_read(input);
        self
    }
    /// <p>A Boolean value that determines the read consistency model during the scan:</p>
    /// <ul>
    /// <li> <p>If <code>ConsistentRead</code> is <code>false</code>, then the data returned from <code>Scan</code> might not contain the results from other recently completed write operations (<code>PutItem</code>, <code>UpdateItem</code>, or <code>DeleteItem</code>).</p> </li>
    /// <li> <p>If <code>ConsistentRead</code> is <code>true</code>, then all of the write operations that completed before the <code>Scan</code> began are guaranteed to be contained in the <code>Scan</code> response.</p> </li>
    /// </ul>
    /// <p>The default setting for <code>ConsistentRead</code> is <code>false</code>.</p>
    /// <p>The <code>ConsistentRead</code> parameter is not supported on global secondary indexes. If you scan a global secondary index with <code>ConsistentRead</code> set to true, you will receive a <code>ValidationException</code>.</p>
    pub fn set_consistent_read(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_consistent_read(input);
        self
    }
}
