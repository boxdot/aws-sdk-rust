// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListDeliveryStreamsOutput {
    /// <p>The names of the delivery streams.</p>
    #[doc(hidden)]
    pub delivery_stream_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Indicates whether there are more delivery streams available to list.</p>
    #[doc(hidden)]
    pub has_more_delivery_streams: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl ListDeliveryStreamsOutput {
    /// <p>The names of the delivery streams.</p>
    pub fn delivery_stream_names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.delivery_stream_names.as_deref()
    }
    /// <p>Indicates whether there are more delivery streams available to list.</p>
    pub fn has_more_delivery_streams(&self) -> ::std::option::Option<bool> {
        self.has_more_delivery_streams
    }
}
impl ::aws_http::request_id::RequestId for ListDeliveryStreamsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListDeliveryStreamsOutput {
    /// Creates a new builder-style object to manufacture [`ListDeliveryStreamsOutput`](crate::operation::list_delivery_streams::ListDeliveryStreamsOutput).
    pub fn builder(
    ) -> crate::operation::list_delivery_streams::builders::ListDeliveryStreamsOutputBuilder {
        crate::operation::list_delivery_streams::builders::ListDeliveryStreamsOutputBuilder::default(
        )
    }
}

/// A builder for [`ListDeliveryStreamsOutput`](crate::operation::list_delivery_streams::ListDeliveryStreamsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListDeliveryStreamsOutputBuilder {
    pub(crate) delivery_stream_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) has_more_delivery_streams: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl ListDeliveryStreamsOutputBuilder {
    /// Appends an item to `delivery_stream_names`.
    ///
    /// To override the contents of this collection use [`set_delivery_stream_names`](Self::set_delivery_stream_names).
    ///
    /// <p>The names of the delivery streams.</p>
    pub fn delivery_stream_names(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.delivery_stream_names.unwrap_or_default();
        v.push(input.into());
        self.delivery_stream_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>The names of the delivery streams.</p>
    pub fn set_delivery_stream_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.delivery_stream_names = input;
        self
    }
    /// <p>Indicates whether there are more delivery streams available to list.</p>
    pub fn has_more_delivery_streams(mut self, input: bool) -> Self {
        self.has_more_delivery_streams = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether there are more delivery streams available to list.</p>
    pub fn set_has_more_delivery_streams(mut self, input: ::std::option::Option<bool>) -> Self {
        self.has_more_delivery_streams = input;
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
    /// Consumes the builder and constructs a [`ListDeliveryStreamsOutput`](crate::operation::list_delivery_streams::ListDeliveryStreamsOutput).
    pub fn build(self) -> crate::operation::list_delivery_streams::ListDeliveryStreamsOutput {
        crate::operation::list_delivery_streams::ListDeliveryStreamsOutput {
            delivery_stream_names: self.delivery_stream_names,
            has_more_delivery_streams: self.has_more_delivery_streams,
            _request_id: self._request_id,
        }
    }
}
