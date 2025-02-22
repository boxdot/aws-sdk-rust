// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDecoderManifestSignals`](crate::operation::list_decoder_manifest_signals::builders::ListDecoderManifestSignalsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_decoder_manifest_signals::builders::ListDecoderManifestSignalsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::list_decoder_manifest_signals::builders::ListDecoderManifestSignalsFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::list_decoder_manifest_signals::builders::ListDecoderManifestSignalsFluentBuilder::set_name): <p> The name of the decoder manifest to list information about. </p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_decoder_manifest_signals::builders::ListDecoderManifestSignalsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_decoder_manifest_signals::builders::ListDecoderManifestSignalsFluentBuilder::set_next_token): <p>A pagination token for the next set of results.</p>  <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next set of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value. </p>
    ///   - [`max_results(i32)`](crate::operation::list_decoder_manifest_signals::builders::ListDecoderManifestSignalsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_decoder_manifest_signals::builders::ListDecoderManifestSignalsFluentBuilder::set_max_results): <p> The maximum number of items to return, between 1 and 100, inclusive. </p>
    /// - On success, responds with [`ListDecoderManifestSignalsOutput`](crate::operation::list_decoder_manifest_signals::ListDecoderManifestSignalsOutput) with field(s):
    ///   - [`signal_decoders(Option<Vec<SignalDecoder>>)`](crate::operation::list_decoder_manifest_signals::ListDecoderManifestSignalsOutput::signal_decoders): <p> Information about a list of signals to decode. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_decoder_manifest_signals::ListDecoderManifestSignalsOutput::next_token): <p> The token to retrieve the next set of results, or <code>null</code> if there are no more results. </p>
    /// - On failure, responds with [`SdkError<ListDecoderManifestSignalsError>`](crate::operation::list_decoder_manifest_signals::ListDecoderManifestSignalsError)
    pub fn list_decoder_manifest_signals(&self) -> crate::operation::list_decoder_manifest_signals::builders::ListDecoderManifestSignalsFluentBuilder{
        crate::operation::list_decoder_manifest_signals::builders::ListDecoderManifestSignalsFluentBuilder::new(self.handle.clone())
    }
}
