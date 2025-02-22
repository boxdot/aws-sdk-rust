// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutRecordOutput {
    /// <p>The ID of the record.</p>
    #[doc(hidden)]
    pub record_id: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    #[doc(hidden)]
    pub encrypted: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl PutRecordOutput {
    /// <p>The ID of the record.</p>
    pub fn record_id(&self) -> ::std::option::Option<&str> {
        self.record_id.as_deref()
    }
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    pub fn encrypted(&self) -> ::std::option::Option<bool> {
        self.encrypted
    }
}
impl ::aws_http::request_id::RequestId for PutRecordOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutRecordOutput {
    /// Creates a new builder-style object to manufacture [`PutRecordOutput`](crate::operation::put_record::PutRecordOutput).
    pub fn builder() -> crate::operation::put_record::builders::PutRecordOutputBuilder {
        crate::operation::put_record::builders::PutRecordOutputBuilder::default()
    }
}

/// A builder for [`PutRecordOutput`](crate::operation::put_record::PutRecordOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutRecordOutputBuilder {
    pub(crate) record_id: ::std::option::Option<::std::string::String>,
    pub(crate) encrypted: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl PutRecordOutputBuilder {
    /// <p>The ID of the record.</p>
    pub fn record_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.record_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the record.</p>
    pub fn set_record_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.record_id = input;
        self
    }
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    pub fn encrypted(mut self, input: bool) -> Self {
        self.encrypted = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    pub fn set_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.encrypted = input;
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
    /// Consumes the builder and constructs a [`PutRecordOutput`](crate::operation::put_record::PutRecordOutput).
    pub fn build(self) -> crate::operation::put_record::PutRecordOutput {
        crate::operation::put_record::PutRecordOutput {
            record_id: self.record_id,
            encrypted: self.encrypted,
            _request_id: self._request_id,
        }
    }
}
