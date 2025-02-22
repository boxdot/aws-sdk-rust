// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl PutLexiconInput {
    /// Consumes the builder and constructs an Operation<[`PutLexicon`](crate::operation::put_lexicon::PutLexicon)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::put_lexicon::PutLexicon,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(_config.use_dual_stack)
            .set_use_fips(_config.use_fips)
            .set_endpoint(_config.endpoint_url.clone())
            .build()
            .map_err(|err| {
                ::aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            ::std::result::Result::Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                ::std::option::Option::Some(params),
            ),
            ::std::result::Result::Err(e) => {
                (::std::result::Result::Err(e), ::std::option::Option::None)
            }
        };
        let mut request = {
            fn uri_base(
                _input: &crate::operation::put_lexicon::PutLexiconInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                let input_1 = &_input.name;
                let input_1 = input_1.as_ref().ok_or_else(|| {
                    ::aws_smithy_http::operation::error::BuildError::missing_field(
                        "name",
                        "cannot be empty or unset",
                    )
                })?;
                let name = ::aws_smithy_http::label::fmt_string(
                    input_1,
                    ::aws_smithy_http::label::EncodingStrategy::Default,
                );
                if name.is_empty() {
                    return ::std::result::Result::Err(
                        ::aws_smithy_http::operation::error::BuildError::missing_field(
                            "name",
                            "cannot be empty or unset",
                        ),
                    );
                }
                ::std::write!(output, "/v1/lexicons/{Name}", Name = name)
                    .expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::put_lexicon::PutLexiconInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<
                ::http::request::Builder,
                ::aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("PUT").uri(uri))
            }
            let mut builder = update_http_builder(&self, ::http::request::Builder::new())?;
            builder = ::aws_smithy_http::header::set_request_header_if_absent(
                builder,
                ::http::header::CONTENT_TYPE,
                "application/json",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_put_lexicon::ser_put_lexicon_input(&self)?,
        );
        if let ::std::option::Option::Some(content_length) = body.content_length() {
            request = ::aws_smithy_http::header::set_request_header_if_absent(
                request,
                ::http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = ::aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let ::std::option::Option::Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(::aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = ::aws_http::user_agent::AwsUserAgent::new_from_environment(
            ::aws_types::os_shim_internal::Env::real(),
            crate::meta::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = ::aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(::aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(::aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        ::aws_http::auth::set_credentials_cache(
            &mut request.properties_mut(),
            _config.credentials_cache.clone(),
        );
        let op = ::aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::put_lexicon::PutLexicon::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "PutLexicon",
            "polly",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `PutLexicon`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct PutLexicon;
impl PutLexicon {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for PutLexicon {
    type Output = ::std::result::Result<
        crate::operation::put_lexicon::PutLexiconOutput,
        crate::operation::put_lexicon::PutLexiconError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_put_lexicon::de_put_lexicon_http_error(
                status, headers, body,
            )
        } else {
            crate::protocol_serde::shape_put_lexicon::de_put_lexicon_http_response_with_props(
                status, headers, body,
            )
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type PutLexiconErrorKind = PutLexiconError;
/// Error type for the `PutLexiconError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum PutLexiconError {
    /// <p>Amazon Polly can't find the specified lexicon. Verify that the lexicon's name is spelled correctly, and then try again.</p>
    InvalidLexiconException(crate::types::error::InvalidLexiconException),
    /// <p>The maximum size of the specified lexicon would be exceeded by this operation.</p>
    LexiconSizeExceededException(crate::types::error::LexiconSizeExceededException),
    /// <p>The maximum size of the lexeme would be exceeded by this operation.</p>
    MaxLexemeLengthExceededException(crate::types::error::MaxLexemeLengthExceededException),
    /// <p>The maximum number of lexicons would be exceeded by this operation.</p>
    MaxLexiconsNumberExceededException(crate::types::error::MaxLexiconsNumberExceededException),
    /// <p>An unknown condition has caused a service failure.</p>
    ServiceFailureException(crate::types::error::ServiceFailureException),
    /// <p>The alphabet specified by the lexicon is not a supported alphabet. Valid values are <code>x-sampa</code> and <code>ipa</code>.</p>
    UnsupportedPlsAlphabetException(crate::types::error::UnsupportedPlsAlphabetException),
    /// <p>The language specified in the lexicon is unsupported. For a list of supported languages, see <a href="https://docs.aws.amazon.com/polly/latest/dg/API_LexiconAttributes.html">Lexicon Attributes</a>.</p>
    UnsupportedPlsLanguageException(crate::types::error::UnsupportedPlsLanguageException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for PutLexiconError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<
            dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
        >,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled({
            let mut builder = ::aws_smithy_types::error::Unhandled::builder().source(source);
            builder.set_meta(meta);
            builder.build()
        })
    }
}
impl ::std::fmt::Display for PutLexiconError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::InvalidLexiconException(_inner) => _inner.fmt(f),
            Self::LexiconSizeExceededException(_inner) => _inner.fmt(f),
            Self::MaxLexemeLengthExceededException(_inner) => _inner.fmt(f),
            Self::MaxLexiconsNumberExceededException(_inner) => _inner.fmt(f),
            Self::ServiceFailureException(_inner) => _inner.fmt(f),
            Self::UnsupportedPlsAlphabetException(_inner) => _inner.fmt(f),
            Self::UnsupportedPlsLanguageException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for PutLexiconError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InvalidLexiconException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::LexiconSizeExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::MaxLexemeLengthExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::MaxLexiconsNumberExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ServiceFailureException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UnsupportedPlsAlphabetException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UnsupportedPlsLanguageException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId for crate::operation::put_lexicon::PutLexiconError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for PutLexiconError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl PutLexiconError {
    /// Creates the `PutLexiconError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<
            ::std::boxed::Box<
                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
            >,
        >,
    ) -> Self {
        Self::Unhandled(
            ::aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `PutLexiconError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(
            ::aws_smithy_types::error::Unhandled::builder()
                .source(err.clone())
                .meta(err)
                .build(),
        )
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::InvalidLexiconException(e) => e.meta(),
            Self::LexiconSizeExceededException(e) => e.meta(),
            Self::MaxLexemeLengthExceededException(e) => e.meta(),
            Self::MaxLexiconsNumberExceededException(e) => e.meta(),
            Self::ServiceFailureException(e) => e.meta(),
            Self::UnsupportedPlsAlphabetException(e) => e.meta(),
            Self::UnsupportedPlsLanguageException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `PutLexiconError::InvalidLexiconException`.
    pub fn is_invalid_lexicon_exception(&self) -> bool {
        matches!(self, Self::InvalidLexiconException(_))
    }
    /// Returns `true` if the error kind is `PutLexiconError::LexiconSizeExceededException`.
    pub fn is_lexicon_size_exceeded_exception(&self) -> bool {
        matches!(self, Self::LexiconSizeExceededException(_))
    }
    /// Returns `true` if the error kind is `PutLexiconError::MaxLexemeLengthExceededException`.
    pub fn is_max_lexeme_length_exceeded_exception(&self) -> bool {
        matches!(self, Self::MaxLexemeLengthExceededException(_))
    }
    /// Returns `true` if the error kind is `PutLexiconError::MaxLexiconsNumberExceededException`.
    pub fn is_max_lexicons_number_exceeded_exception(&self) -> bool {
        matches!(self, Self::MaxLexiconsNumberExceededException(_))
    }
    /// Returns `true` if the error kind is `PutLexiconError::ServiceFailureException`.
    pub fn is_service_failure_exception(&self) -> bool {
        matches!(self, Self::ServiceFailureException(_))
    }
    /// Returns `true` if the error kind is `PutLexiconError::UnsupportedPlsAlphabetException`.
    pub fn is_unsupported_pls_alphabet_exception(&self) -> bool {
        matches!(self, Self::UnsupportedPlsAlphabetException(_))
    }
    /// Returns `true` if the error kind is `PutLexiconError::UnsupportedPlsLanguageException`.
    pub fn is_unsupported_pls_language_exception(&self) -> bool {
        matches!(self, Self::UnsupportedPlsLanguageException(_))
    }
}
impl ::std::error::Error for PutLexiconError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::InvalidLexiconException(_inner) => ::std::option::Option::Some(_inner),
            Self::LexiconSizeExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::MaxLexemeLengthExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::MaxLexiconsNumberExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::ServiceFailureException(_inner) => ::std::option::Option::Some(_inner),
            Self::UnsupportedPlsAlphabetException(_inner) => ::std::option::Option::Some(_inner),
            Self::UnsupportedPlsLanguageException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::put_lexicon::_put_lexicon_output::PutLexiconOutput;

pub use crate::operation::put_lexicon::_put_lexicon_input::PutLexiconInput;

mod _put_lexicon_input;

mod _put_lexicon_output;

/// Builders
pub mod builders;
