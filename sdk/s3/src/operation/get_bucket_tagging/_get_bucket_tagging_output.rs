// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBucketTaggingOutput {
    /// <p>Contains the tag set.</p>
    #[doc(hidden)]
    pub tag_set: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetBucketTaggingOutput {
    /// <p>Contains the tag set.</p>
    pub fn tag_set(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tag_set.as_deref()
    }
}
impl crate::s3_request_id::RequestIdExt for GetBucketTaggingOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetBucketTaggingOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetBucketTaggingOutput {
    /// Creates a new builder-style object to manufacture [`GetBucketTaggingOutput`](crate::operation::get_bucket_tagging::GetBucketTaggingOutput).
    pub fn builder() -> crate::operation::get_bucket_tagging::builders::GetBucketTaggingOutputBuilder
    {
        crate::operation::get_bucket_tagging::builders::GetBucketTaggingOutputBuilder::default()
    }
}

/// A builder for [`GetBucketTaggingOutput`](crate::operation::get_bucket_tagging::GetBucketTaggingOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetBucketTaggingOutputBuilder {
    pub(crate) tag_set: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetBucketTaggingOutputBuilder {
    /// Appends an item to `tag_set`.
    ///
    /// To override the contents of this collection use [`set_tag_set`](Self::set_tag_set).
    ///
    /// <p>Contains the tag set.</p>
    pub fn tag_set(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tag_set.unwrap_or_default();
        v.push(input);
        self.tag_set = ::std::option::Option::Some(v);
        self
    }
    /// <p>Contains the tag set.</p>
    pub fn set_tag_set(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tag_set = input;
        self
    }
    pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {
        self._extended_request_id = Some(extended_request_id.into());
        self
    }

    pub(crate) fn _set_extended_request_id(
        &mut self,
        extended_request_id: Option<String>,
    ) -> &mut Self {
        self._extended_request_id = extended_request_id;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetBucketTaggingOutput`](crate::operation::get_bucket_tagging::GetBucketTaggingOutput).
    pub fn build(self) -> crate::operation::get_bucket_tagging::GetBucketTaggingOutput {
        crate::operation::get_bucket_tagging::GetBucketTaggingOutput {
            tag_set: self.tag_set,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
