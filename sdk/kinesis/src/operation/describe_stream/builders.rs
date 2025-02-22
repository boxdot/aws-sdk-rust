// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_stream::_describe_stream_output::DescribeStreamOutputBuilder;

pub use crate::operation::describe_stream::_describe_stream_input::DescribeStreamInputBuilder;

/// Fluent builder constructing a request to `DescribeStream`.
///
/// <p>Describes the specified Kinesis data stream.</p> <note>
/// <p>This API has been revised. It's highly recommended that you use the <code>DescribeStreamSummary</code> API to get a summarized description of the specified Kinesis data stream and the <code>ListShards</code> API to list the shards in a specified data stream and obtain information about each shard. </p>
/// </note> <note>
/// <p>When invoking this API, it is recommended you use the <code>StreamARN</code> input parameter rather than the <code>StreamName</code> input parameter.</p>
/// </note>
/// <p>The information returned includes the stream name, Amazon Resource Name (ARN), creation time, enhanced metric configuration, and shard map. The shard map is an array of shard objects. For each shard object, there is the hash key and sequence number ranges that the shard spans, and the IDs of any earlier shards that played in a role in creating the shard. Every record ingested in the stream is identified by a sequence number, which is assigned when the record is put into the stream.</p>
/// <p>You can limit the number of shards returned by each call. For more information, see <a href="https://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-retrieve-shards.html">Retrieving Shards from a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p>
/// <p>There are no guarantees about the chronological order shards returned. To process shards in chronological order, use the ID of the parent shard to track the lineage to the oldest shard.</p>
/// <p>This operation has a limit of 10 transactions per second per account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeStreamFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_stream::builders::DescribeStreamInputBuilder,
}
impl DescribeStreamFluentBuilder {
    /// Creates a new `DescribeStream`.
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
            crate::operation::describe_stream::DescribeStream,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_stream::DescribeStreamError>,
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
        crate::operation::describe_stream::DescribeStreamOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_stream::DescribeStreamError>,
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
        crate::operation::describe_stream::DescribeStreamOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_stream::DescribeStreamError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_stream::DescribeStream,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_stream::DescribeStreamError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the stream to describe.</p>
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_name(input.into());
        self
    }
    /// <p>The name of the stream to describe.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_name(input);
        self
    }
    /// <p>The maximum number of shards to return in a single call. The default value is 100. If you specify a value greater than 100, at most 100 results are returned.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of shards to return in a single call. The default value is 100. If you specify a value greater than 100, at most 100 results are returned.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The shard ID of the shard to start with.</p>
    /// <p>Specify this parameter to indicate that you want to describe the stream starting with the shard whose ID immediately follows <code>ExclusiveStartShardId</code>.</p>
    /// <p>If you don't specify this parameter, the default behavior for <code>DescribeStream</code> is to describe the stream starting with the first shard in the stream.</p>
    pub fn exclusive_start_shard_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.exclusive_start_shard_id(input.into());
        self
    }
    /// <p>The shard ID of the shard to start with.</p>
    /// <p>Specify this parameter to indicate that you want to describe the stream starting with the shard whose ID immediately follows <code>ExclusiveStartShardId</code>.</p>
    /// <p>If you don't specify this parameter, the default behavior for <code>DescribeStream</code> is to describe the stream starting with the first shard in the stream.</p>
    pub fn set_exclusive_start_shard_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_exclusive_start_shard_id(input);
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_arn(input.into());
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_arn(input);
        self
    }
}
