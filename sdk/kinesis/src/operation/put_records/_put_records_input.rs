// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A <code>PutRecords</code> request.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutRecordsInput {
    /// <p>The records associated with the request.</p>
    #[doc(hidden)]
    pub records: ::std::option::Option<::std::vec::Vec<crate::types::PutRecordsRequestEntry>>,
    /// <p>The stream name associated with the request.</p>
    #[doc(hidden)]
    pub stream_name: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the stream.</p>
    #[doc(hidden)]
    pub stream_arn: ::std::option::Option<::std::string::String>,
}
impl PutRecordsInput {
    /// <p>The records associated with the request.</p>
    pub fn records(&self) -> ::std::option::Option<&[crate::types::PutRecordsRequestEntry]> {
        self.records.as_deref()
    }
    /// <p>The stream name associated with the request.</p>
    pub fn stream_name(&self) -> ::std::option::Option<&str> {
        self.stream_name.as_deref()
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(&self) -> ::std::option::Option<&str> {
        self.stream_arn.as_deref()
    }
}
impl PutRecordsInput {
    /// Creates a new builder-style object to manufacture [`PutRecordsInput`](crate::operation::put_records::PutRecordsInput).
    pub fn builder() -> crate::operation::put_records::builders::PutRecordsInputBuilder {
        crate::operation::put_records::builders::PutRecordsInputBuilder::default()
    }
}

/// A builder for [`PutRecordsInput`](crate::operation::put_records::PutRecordsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutRecordsInputBuilder {
    pub(crate) records:
        ::std::option::Option<::std::vec::Vec<crate::types::PutRecordsRequestEntry>>,
    pub(crate) stream_name: ::std::option::Option<::std::string::String>,
    pub(crate) stream_arn: ::std::option::Option<::std::string::String>,
}
impl PutRecordsInputBuilder {
    /// Appends an item to `records`.
    ///
    /// To override the contents of this collection use [`set_records`](Self::set_records).
    ///
    /// <p>The records associated with the request.</p>
    pub fn records(mut self, input: crate::types::PutRecordsRequestEntry) -> Self {
        let mut v = self.records.unwrap_or_default();
        v.push(input);
        self.records = ::std::option::Option::Some(v);
        self
    }
    /// <p>The records associated with the request.</p>
    pub fn set_records(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PutRecordsRequestEntry>>,
    ) -> Self {
        self.records = input;
        self
    }
    /// <p>The stream name associated with the request.</p>
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The stream name associated with the request.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_name = input;
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`PutRecordsInput`](crate::operation::put_records::PutRecordsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_records::PutRecordsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::put_records::PutRecordsInput {
            records: self.records,
            stream_name: self.stream_name,
            stream_arn: self.stream_arn,
        })
    }
}
