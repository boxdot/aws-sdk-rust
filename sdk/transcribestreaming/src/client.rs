// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    client: smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// An ergonomic service client for `Transcribe`.
///
/// This client allows ergonomic access to a `Transcribe`-shaped service.
/// Each method corresponds to an endpoint defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
///
/// # Using a Client
///
/// Once you have a client set up, you can access the service's endpoints
/// by calling the appropriate method on [`Client`]. Each such method
/// returns a request builder for that endpoint, with methods for setting
/// the various fields of the request. Once your request is complete, use
/// the `send` method to send the request. `send` returns a future, which
/// you then have to `.await` to get the service's response.
///
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder
/// [SigV4-signed requests]: https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
#[derive(std::fmt::Debug)]
pub struct Client<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use smithy_client::Builder;

impl<C, M, R> From<smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    pub fn with_config(client: smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: smithy_client::bounds::SmithyConnector,
    M: smithy_client::bounds::SmithyMiddleware<C>,
    R: smithy_client::retry::NewRequestPolicy,
{
    pub fn start_medical_stream_transcription(
        &self,
    ) -> fluent_builders::StartMedicalStreamTranscription<C, M, R> {
        fluent_builders::StartMedicalStreamTranscription::new(self.handle.clone())
    }
    pub fn start_stream_transcription(&self) -> fluent_builders::StartStreamTranscription<C, M, R> {
        fluent_builders::StartStreamTranscription::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct StartMedicalStreamTranscription<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::start_medical_stream_transcription_input::Builder,
    }
    impl<C, M, R> StartMedicalStreamTranscription<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::StartMedicalStreamTranscriptionOutput,
            smithy_http::result::SdkError<crate::error::StartMedicalStreamTranscriptionError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::StartMedicalStreamTranscriptionInputOperationOutputAlias,
                crate::output::StartMedicalStreamTranscriptionOutput,
                crate::error::StartMedicalStreamTranscriptionError,
                crate::input::StartMedicalStreamTranscriptionInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p> Indicates the source language used in the input audio stream. For Amazon Transcribe Medical, this is US
        /// English (en-US). </p>
        pub fn language_code(mut self, inp: crate::model::LanguageCode) -> Self {
            self.inner = self.inner.language_code(inp);
            self
        }
        pub fn set_language_code(
            mut self,
            input: std::option::Option<crate::model::LanguageCode>,
        ) -> Self {
            self.inner = self.inner.set_language_code(input);
            self
        }
        /// <p>The sample rate of the input audio in Hertz.</p>
        pub fn media_sample_rate_hertz(mut self, inp: i32) -> Self {
            self.inner = self.inner.media_sample_rate_hertz(inp);
            self
        }
        pub fn set_media_sample_rate_hertz(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_media_sample_rate_hertz(input);
            self
        }
        /// <p>The encoding used for the input audio.</p>
        pub fn media_encoding(mut self, inp: crate::model::MediaEncoding) -> Self {
            self.inner = self.inner.media_encoding(inp);
            self
        }
        pub fn set_media_encoding(
            mut self,
            input: std::option::Option<crate::model::MediaEncoding>,
        ) -> Self {
            self.inner = self.inner.set_media_encoding(input);
            self
        }
        /// <p>The name of the medical custom vocabulary to use when processing the real-time
        /// stream.</p>
        pub fn vocabulary_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.vocabulary_name(inp);
            self
        }
        pub fn set_vocabulary_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_vocabulary_name(input);
            self
        }
        /// <p>The medical specialty of the clinician or provider.</p>
        pub fn specialty(mut self, inp: crate::model::Specialty) -> Self {
            self.inner = self.inner.specialty(inp);
            self
        }
        pub fn set_specialty(
            mut self,
            input: std::option::Option<crate::model::Specialty>,
        ) -> Self {
            self.inner = self.inner.set_specialty(input);
            self
        }
        /// <p>The type of input audio. Choose <code>DICTATION</code> for a provider dictating
        /// patient notes. Choose <code>CONVERSATION</code> for a dialogue between a patient and one
        /// or more medical professionanls.</p>
        pub fn r#type(mut self, inp: crate::model::Type) -> Self {
            self.inner = self.inner.r#type(inp);
            self
        }
        pub fn set_type(mut self, input: std::option::Option<crate::model::Type>) -> Self {
            self.inner = self.inner.set_type(input);
            self
        }
        /// <p>When <code>true</code>, enables speaker identification in your real-time
        /// stream.</p>
        pub fn show_speaker_label(mut self, inp: bool) -> Self {
            self.inner = self.inner.show_speaker_label(inp);
            self
        }
        pub fn set_show_speaker_label(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_show_speaker_label(input);
            self
        }
        /// <p> Optional. An identifier for the transcription session. If you don't provide a session
        /// ID, Amazon Transcribe generates one for you and returns it in the response. </p>
        pub fn session_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.session_id(inp);
            self
        }
        pub fn set_session_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_session_id(input);
            self
        }
        /// <p>Represents the audio stream from your application to Amazon Transcribe.</p>
        pub fn audio_stream(
            mut self,
            inp: smithy_http::event_stream::EventStreamInput<crate::model::AudioStream>,
        ) -> Self {
            self.inner = self.inner.audio_stream(inp);
            self
        }
        pub fn set_audio_stream(
            mut self,
            input: std::option::Option<
                smithy_http::event_stream::EventStreamInput<crate::model::AudioStream>,
            >,
        ) -> Self {
            self.inner = self.inner.set_audio_stream(input);
            self
        }
        /// <p>When <code>true</code>, instructs Amazon Transcribe Medical to process each audio channel separately and
        /// then merge the transcription output of each channel into a single transcription.</p>
        /// <p>Amazon Transcribe Medical also produces a transcription of each item. An item includes the start time,
        /// end time, and any alternative transcriptions.</p>
        /// <p>You can't set both <code>ShowSpeakerLabel</code> and
        /// <code>EnableChannelIdentification</code> in the same request. If you set both, your
        /// request returns a <code>BadRequestException</code>.</p>
        pub fn enable_channel_identification(mut self, inp: bool) -> Self {
            self.inner = self.inner.enable_channel_identification(inp);
            self
        }
        pub fn set_enable_channel_identification(
            mut self,
            input: std::option::Option<bool>,
        ) -> Self {
            self.inner = self.inner.set_enable_channel_identification(input);
            self
        }
        /// <p>The number of channels that are in your audio stream.</p>
        pub fn number_of_channels(mut self, inp: i32) -> Self {
            self.inner = self.inner.number_of_channels(inp);
            self
        }
        pub fn set_number_of_channels(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_number_of_channels(input);
            self
        }
        /// <p>Set this field to <code>PHI</code> to identify personal health information in the
        /// transcription output.</p>
        pub fn content_identification_type(
            mut self,
            inp: crate::model::MedicalContentIdentificationType,
        ) -> Self {
            self.inner = self.inner.content_identification_type(inp);
            self
        }
        pub fn set_content_identification_type(
            mut self,
            input: std::option::Option<crate::model::MedicalContentIdentificationType>,
        ) -> Self {
            self.inner = self.inner.set_content_identification_type(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct StartStreamTranscription<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::start_stream_transcription_input::Builder,
    }
    impl<C, M, R> StartStreamTranscription<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::StartStreamTranscriptionOutput,
            smithy_http::result::SdkError<crate::error::StartStreamTranscriptionError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::StartStreamTranscriptionInputOperationOutputAlias,
                crate::output::StartStreamTranscriptionOutput,
                crate::error::StartStreamTranscriptionError,
                crate::input::StartStreamTranscriptionInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Indicates the source language used in the input audio stream.</p>
        pub fn language_code(mut self, inp: crate::model::LanguageCode) -> Self {
            self.inner = self.inner.language_code(inp);
            self
        }
        pub fn set_language_code(
            mut self,
            input: std::option::Option<crate::model::LanguageCode>,
        ) -> Self {
            self.inner = self.inner.set_language_code(input);
            self
        }
        /// <p>The sample rate, in Hertz, of the input audio. We suggest that you use 8,000 Hz for low
        /// quality audio and 16,000 Hz for high quality audio.</p>
        pub fn media_sample_rate_hertz(mut self, inp: i32) -> Self {
            self.inner = self.inner.media_sample_rate_hertz(inp);
            self
        }
        pub fn set_media_sample_rate_hertz(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_media_sample_rate_hertz(input);
            self
        }
        /// <p>The encoding used for the input audio.</p>
        pub fn media_encoding(mut self, inp: crate::model::MediaEncoding) -> Self {
            self.inner = self.inner.media_encoding(inp);
            self
        }
        pub fn set_media_encoding(
            mut self,
            input: std::option::Option<crate::model::MediaEncoding>,
        ) -> Self {
            self.inner = self.inner.set_media_encoding(input);
            self
        }
        /// <p>The name of the vocabulary to use when processing the transcription job.</p>
        pub fn vocabulary_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.vocabulary_name(inp);
            self
        }
        pub fn set_vocabulary_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_vocabulary_name(input);
            self
        }
        /// <p>A identifier for the transcription session. Use this parameter when you want to retry a
        /// session. If you don't provide a session ID, Amazon Transcribe will generate one for you and return it in
        /// the response.</p>
        pub fn session_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.session_id(inp);
            self
        }
        pub fn set_session_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_session_id(input);
            self
        }
        /// <p>PCM-encoded stream of audio blobs. The audio stream is encoded as an HTTP/2 data
        /// frame.</p>
        pub fn audio_stream(
            mut self,
            inp: smithy_http::event_stream::EventStreamInput<crate::model::AudioStream>,
        ) -> Self {
            self.inner = self.inner.audio_stream(inp);
            self
        }
        pub fn set_audio_stream(
            mut self,
            input: std::option::Option<
                smithy_http::event_stream::EventStreamInput<crate::model::AudioStream>,
            >,
        ) -> Self {
            self.inner = self.inner.set_audio_stream(input);
            self
        }
        /// <p>The name of the vocabulary filter you've created that is unique to your account.
        /// Provide the name in this field to successfully use it in a stream.</p>
        pub fn vocabulary_filter_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.vocabulary_filter_name(inp);
            self
        }
        pub fn set_vocabulary_filter_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_vocabulary_filter_name(input);
            self
        }
        /// <p>The manner in which you use your vocabulary filter to filter words in your transcript.
        /// <code>Remove</code> removes filtered words from your transcription results.
        /// <code>Mask</code> masks filtered words with a <code>***</code> in your transcription results.
        /// <code>Tag</code> keeps the filtered words in your transcription results and tags them. The
        /// tag appears as <code>VocabularyFilterMatch</code> equal to <code>True</code>
        /// </p>
        pub fn vocabulary_filter_method(
            mut self,
            inp: crate::model::VocabularyFilterMethod,
        ) -> Self {
            self.inner = self.inner.vocabulary_filter_method(inp);
            self
        }
        pub fn set_vocabulary_filter_method(
            mut self,
            input: std::option::Option<crate::model::VocabularyFilterMethod>,
        ) -> Self {
            self.inner = self.inner.set_vocabulary_filter_method(input);
            self
        }
        /// <p>When <code>true</code>, enables speaker identification in your real-time stream.</p>
        pub fn show_speaker_label(mut self, inp: bool) -> Self {
            self.inner = self.inner.show_speaker_label(inp);
            self
        }
        pub fn set_show_speaker_label(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_show_speaker_label(input);
            self
        }
        /// <p>When <code>true</code>, instructs Amazon Transcribe to process each audio channel separately and then
        /// merge the transcription output of each channel into a single transcription.</p>
        /// <p>Amazon Transcribe also produces a transcription of each item. An item includes the start time, end
        /// time, and any alternative transcriptions.</p>
        /// <p>You can't set both <code>ShowSpeakerLabel</code> and
        /// <code>EnableChannelIdentification</code> in the same request. If you set both, your request
        /// returns a <code>BadRequestException</code>.</p>
        pub fn enable_channel_identification(mut self, inp: bool) -> Self {
            self.inner = self.inner.enable_channel_identification(inp);
            self
        }
        pub fn set_enable_channel_identification(
            mut self,
            input: std::option::Option<bool>,
        ) -> Self {
            self.inner = self.inner.set_enable_channel_identification(input);
            self
        }
        /// <p>The number of channels that are in your audio stream.</p>
        pub fn number_of_channels(mut self, inp: i32) -> Self {
            self.inner = self.inner.number_of_channels(inp);
            self
        }
        pub fn set_number_of_channels(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_number_of_channels(input);
            self
        }
        /// <p>When <code>true</code>, instructs Amazon Transcribe to present transcription results that have the
        /// partial results stabilized. Normally, any word or phrase from one partial result can change in
        /// a subsequent partial result. With partial results stabilization enabled, only the last few
        /// words of one partial result can change in another partial result.</p>
        pub fn enable_partial_results_stabilization(mut self, inp: bool) -> Self {
            self.inner = self.inner.enable_partial_results_stabilization(inp);
            self
        }
        pub fn set_enable_partial_results_stabilization(
            mut self,
            input: std::option::Option<bool>,
        ) -> Self {
            self.inner = self.inner.set_enable_partial_results_stabilization(input);
            self
        }
        /// <p>You can use this field to set the stability level of the transcription results. A higher
        /// stability level means that the transcription results are less likely to change. Higher
        /// stability levels can come with lower overall transcription accuracy.</p>
        pub fn partial_results_stability(
            mut self,
            inp: crate::model::PartialResultsStability,
        ) -> Self {
            self.inner = self.inner.partial_results_stability(inp);
            self
        }
        pub fn set_partial_results_stability(
            mut self,
            input: std::option::Option<crate::model::PartialResultsStability>,
        ) -> Self {
            self.inner = self.inner.set_partial_results_stability(input);
            self
        }
        /// <p>Set this field to PII to identify personally identifiable information (PII) in the transcription output. Content identification is performed only upon complete transcription of the audio segments.</p>
        /// <p>You can’t set both <code>ContentIdentificationType</code> and <code>ContentRedactionType</code> in the same request. If you set both, your request returns a <code>BadRequestException</code>.</p>
        pub fn content_identification_type(
            mut self,
            inp: crate::model::ContentIdentificationType,
        ) -> Self {
            self.inner = self.inner.content_identification_type(inp);
            self
        }
        pub fn set_content_identification_type(
            mut self,
            input: std::option::Option<crate::model::ContentIdentificationType>,
        ) -> Self {
            self.inner = self.inner.set_content_identification_type(input);
            self
        }
        /// <p>Set this field to PII to redact personally identifiable information (PII) in the transcription output. Content redaction is performed only upon complete transcription of the audio segments.</p>
        /// <p>You can’t set both <code>ContentRedactionType</code> and <code>ContentIdentificationType</code> in the same request. If you set both, your request returns a <code>BadRequestException</code>.</p>
        pub fn content_redaction_type(mut self, inp: crate::model::ContentRedactionType) -> Self {
            self.inner = self.inner.content_redaction_type(inp);
            self
        }
        pub fn set_content_redaction_type(
            mut self,
            input: std::option::Option<crate::model::ContentRedactionType>,
        ) -> Self {
            self.inner = self.inner.set_content_redaction_type(input);
            self
        }
        /// <p>List the PII entity types you want to identify or redact. In order to specify entity types, you must have
        /// either <code>ContentIdentificationType</code> or <code>ContentRedactionType</code> enabled.</p>
        /// <p>
        /// <code>PIIEntityTypes</code> must be comma-separated; the available values are:
        /// <code>BANK_ACCOUNT_NUMBER</code>, <code>BANK_ROUTING</code>,
        /// <code>CREDIT_DEBIT_NUMBER</code>, <code>CREDIT_DEBIT_CVV</code>,
        /// <code>CREDIT_DEBIT_EXPIRY</code>, <code>PIN</code>, <code>EMAIL</code>,
        /// <code>ADDRESS</code>, <code>NAME</code>, <code>PHONE</code>,
        /// <code>SSN</code>, and <code>ALL</code>.</p>
        /// <p>
        /// <code>PiiEntityTypes</code> is an optional parameter with a default value of <code>ALL</code>.</p>
        pub fn pii_entity_types(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.pii_entity_types(inp);
            self
        }
        pub fn set_pii_entity_types(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_pii_entity_types(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, smithy_client::retry::Standard> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        smithy_client::retry::Standard,
    >
{
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let client = aws_hyper::Client::https();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
