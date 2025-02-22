// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchPutPropertyValuesOutput {
    /// <p>Entries that caused errors in the batch put operation.</p>
    #[doc(hidden)]
    pub error_entries:
        ::std::option::Option<::std::vec::Vec<crate::types::BatchPutPropertyErrorEntry>>,
    _request_id: Option<String>,
}
impl BatchPutPropertyValuesOutput {
    /// <p>Entries that caused errors in the batch put operation.</p>
    pub fn error_entries(
        &self,
    ) -> ::std::option::Option<&[crate::types::BatchPutPropertyErrorEntry]> {
        self.error_entries.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for BatchPutPropertyValuesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchPutPropertyValuesOutput {
    /// Creates a new builder-style object to manufacture [`BatchPutPropertyValuesOutput`](crate::operation::batch_put_property_values::BatchPutPropertyValuesOutput).
    pub fn builder(
    ) -> crate::operation::batch_put_property_values::builders::BatchPutPropertyValuesOutputBuilder
    {
        crate::operation::batch_put_property_values::builders::BatchPutPropertyValuesOutputBuilder::default()
    }
}

/// A builder for [`BatchPutPropertyValuesOutput`](crate::operation::batch_put_property_values::BatchPutPropertyValuesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchPutPropertyValuesOutputBuilder {
    pub(crate) error_entries:
        ::std::option::Option<::std::vec::Vec<crate::types::BatchPutPropertyErrorEntry>>,
    _request_id: Option<String>,
}
impl BatchPutPropertyValuesOutputBuilder {
    /// Appends an item to `error_entries`.
    ///
    /// To override the contents of this collection use [`set_error_entries`](Self::set_error_entries).
    ///
    /// <p>Entries that caused errors in the batch put operation.</p>
    pub fn error_entries(mut self, input: crate::types::BatchPutPropertyErrorEntry) -> Self {
        let mut v = self.error_entries.unwrap_or_default();
        v.push(input);
        self.error_entries = ::std::option::Option::Some(v);
        self
    }
    /// <p>Entries that caused errors in the batch put operation.</p>
    pub fn set_error_entries(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::BatchPutPropertyErrorEntry>>,
    ) -> Self {
        self.error_entries = input;
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
    /// Consumes the builder and constructs a [`BatchPutPropertyValuesOutput`](crate::operation::batch_put_property_values::BatchPutPropertyValuesOutput).
    pub fn build(
        self,
    ) -> crate::operation::batch_put_property_values::BatchPutPropertyValuesOutput {
        crate::operation::batch_put_property_values::BatchPutPropertyValuesOutput {
            error_entries: self.error_entries,
            _request_id: self._request_id,
        }
    }
}
