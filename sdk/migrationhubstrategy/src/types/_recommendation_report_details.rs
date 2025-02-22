// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Contains detailed information about a recommendation report. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RecommendationReportDetails {
    /// <p> The status of the recommendation report generation task. </p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::RecommendationReportStatus>,
    /// <p> The status message for recommendation report generation. </p>
    #[doc(hidden)]
    pub status_message: ::std::option::Option<::std::string::String>,
    /// <p> The time that the recommendation report generation task starts. </p>
    #[doc(hidden)]
    pub start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p> The time that the recommendation report generation task completes. </p>
    #[doc(hidden)]
    pub completion_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p> The S3 bucket where the report file is located. </p>
    #[doc(hidden)]
    pub s3_bucket: ::std::option::Option<::std::string::String>,
    /// <p> The Amazon S3 key name of the report file. </p>
    #[doc(hidden)]
    pub s3_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl RecommendationReportDetails {
    /// <p> The status of the recommendation report generation task. </p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::RecommendationReportStatus> {
        self.status.as_ref()
    }
    /// <p> The status message for recommendation report generation. </p>
    pub fn status_message(&self) -> ::std::option::Option<&str> {
        self.status_message.as_deref()
    }
    /// <p> The time that the recommendation report generation task starts. </p>
    pub fn start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p> The time that the recommendation report generation task completes. </p>
    pub fn completion_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.completion_time.as_ref()
    }
    /// <p> The S3 bucket where the report file is located. </p>
    pub fn s3_bucket(&self) -> ::std::option::Option<&str> {
        self.s3_bucket.as_deref()
    }
    /// <p> The Amazon S3 key name of the report file. </p>
    pub fn s3_keys(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.s3_keys.as_deref()
    }
}
impl RecommendationReportDetails {
    /// Creates a new builder-style object to manufacture [`RecommendationReportDetails`](crate::types::RecommendationReportDetails).
    pub fn builder() -> crate::types::builders::RecommendationReportDetailsBuilder {
        crate::types::builders::RecommendationReportDetailsBuilder::default()
    }
}

/// A builder for [`RecommendationReportDetails`](crate::types::RecommendationReportDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RecommendationReportDetailsBuilder {
    pub(crate) status: ::std::option::Option<crate::types::RecommendationReportStatus>,
    pub(crate) status_message: ::std::option::Option<::std::string::String>,
    pub(crate) start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) completion_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) s3_bucket: ::std::option::Option<::std::string::String>,
    pub(crate) s3_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl RecommendationReportDetailsBuilder {
    /// <p> The status of the recommendation report generation task. </p>
    pub fn status(mut self, input: crate::types::RecommendationReportStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p> The status of the recommendation report generation task. </p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::RecommendationReportStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p> The status message for recommendation report generation. </p>
    pub fn status_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.status_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The status message for recommendation report generation. </p>
    pub fn set_status_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.status_message = input;
        self
    }
    /// <p> The time that the recommendation report generation task starts. </p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> The time that the recommendation report generation task starts. </p>
    pub fn set_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.start_time = input;
        self
    }
    /// <p> The time that the recommendation report generation task completes. </p>
    pub fn completion_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.completion_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> The time that the recommendation report generation task completes. </p>
    pub fn set_completion_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.completion_time = input;
        self
    }
    /// <p> The S3 bucket where the report file is located. </p>
    pub fn s3_bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The S3 bucket where the report file is located. </p>
    pub fn set_s3_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_bucket = input;
        self
    }
    /// Appends an item to `s3_keys`.
    ///
    /// To override the contents of this collection use [`set_s3_keys`](Self::set_s3_keys).
    ///
    /// <p> The Amazon S3 key name of the report file. </p>
    pub fn s3_keys(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.s3_keys.unwrap_or_default();
        v.push(input.into());
        self.s3_keys = ::std::option::Option::Some(v);
        self
    }
    /// <p> The Amazon S3 key name of the report file. </p>
    pub fn set_s3_keys(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.s3_keys = input;
        self
    }
    /// Consumes the builder and constructs a [`RecommendationReportDetails`](crate::types::RecommendationReportDetails).
    pub fn build(self) -> crate::types::RecommendationReportDetails {
        crate::types::RecommendationReportDetails {
            status: self.status,
            status_message: self.status_message,
            start_time: self.start_time,
            completion_time: self.completion_time,
            s3_bucket: self.s3_bucket,
            s3_keys: self.s3_keys,
        }
    }
}
