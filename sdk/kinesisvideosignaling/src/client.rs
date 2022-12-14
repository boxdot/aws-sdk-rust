// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) client: aws_smithy_client::Client<
        aws_smithy_client::erase::DynConnector,
        aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
    >,
    pub(crate) conf: crate::Config,
}

/// Client for Amazon Kinesis Video Signaling Channels
///
/// Client for invoking operations on Amazon Kinesis Video Signaling Channels. Each operation on Amazon Kinesis Video Signaling Channels is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_kinesisvideosignaling::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::retry::RetryConfig;
/// # async fn docs() {
/// let shared_config = aws_config::load_from_env().await;
/// let config = aws_sdk_kinesisvideosignaling::config::Builder::from(&shared_config)
///   .retry_config(RetryConfig::disabled())
///   .build();
/// let client = aws_sdk_kinesisvideosignaling::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client {
    handle: std::sync::Arc<Handle>,
}

impl std::clone::Clone for Client {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl
    From<
        aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    > for Client
{
    fn from(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    ) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl Client {
    /// Creates a client with the given service configuration.
    pub fn with_config(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
        conf: crate::Config,
    ) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    /// Constructs a fluent builder for the [`GetIceServerConfig`](crate::client::fluent_builders::GetIceServerConfig) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::client::fluent_builders::GetIceServerConfig::channel_arn) / [`set_channel_arn(Option<String>)`](crate::client::fluent_builders::GetIceServerConfig::set_channel_arn): <p>The ARN of the signaling channel to be used for the peer-to-peer connection between configured peers. </p>
    ///   - [`client_id(impl Into<String>)`](crate::client::fluent_builders::GetIceServerConfig::client_id) / [`set_client_id(Option<String>)`](crate::client::fluent_builders::GetIceServerConfig::set_client_id): <p>Unique identifier for the viewer. Must be unique within the signaling channel.</p>
    ///   - [`service(Service)`](crate::client::fluent_builders::GetIceServerConfig::service) / [`set_service(Option<Service>)`](crate::client::fluent_builders::GetIceServerConfig::set_service): <p>Specifies the desired service. Currently, <code>TURN</code> is the only valid value.</p>
    ///   - [`username(impl Into<String>)`](crate::client::fluent_builders::GetIceServerConfig::username) / [`set_username(Option<String>)`](crate::client::fluent_builders::GetIceServerConfig::set_username): <p>An optional user ID to be associated with the credentials.</p>
    /// - On success, responds with [`GetIceServerConfigOutput`](crate::output::GetIceServerConfigOutput) with field(s):
    ///   - [`ice_server_list(Option<Vec<IceServer>>)`](crate::output::GetIceServerConfigOutput::ice_server_list): <p>The list of ICE server information objects.</p>
    /// - On failure, responds with [`SdkError<GetIceServerConfigError>`](crate::error::GetIceServerConfigError)
    pub fn get_ice_server_config(&self) -> fluent_builders::GetIceServerConfig {
        fluent_builders::GetIceServerConfig::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`SendAlexaOfferToMaster`](crate::client::fluent_builders::SendAlexaOfferToMaster) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::client::fluent_builders::SendAlexaOfferToMaster::channel_arn) / [`set_channel_arn(Option<String>)`](crate::client::fluent_builders::SendAlexaOfferToMaster::set_channel_arn): <p>The ARN of the signaling channel by which Alexa and the master peer communicate.</p>
    ///   - [`sender_client_id(impl Into<String>)`](crate::client::fluent_builders::SendAlexaOfferToMaster::sender_client_id) / [`set_sender_client_id(Option<String>)`](crate::client::fluent_builders::SendAlexaOfferToMaster::set_sender_client_id): <p>The unique identifier for the sender client.</p>
    ///   - [`message_payload(impl Into<String>)`](crate::client::fluent_builders::SendAlexaOfferToMaster::message_payload) / [`set_message_payload(Option<String>)`](crate::client::fluent_builders::SendAlexaOfferToMaster::set_message_payload): <p>The base64-encoded SDP offer content.</p>
    /// - On success, responds with [`SendAlexaOfferToMasterOutput`](crate::output::SendAlexaOfferToMasterOutput) with field(s):
    ///   - [`answer(Option<String>)`](crate::output::SendAlexaOfferToMasterOutput::answer): <p>The base64-encoded SDP answer content.</p>
    /// - On failure, responds with [`SdkError<SendAlexaOfferToMasterError>`](crate::error::SendAlexaOfferToMasterError)
    pub fn send_alexa_offer_to_master(&self) -> fluent_builders::SendAlexaOfferToMaster {
        fluent_builders::SendAlexaOfferToMaster::new(self.handle.clone())
    }
}
pub mod fluent_builders {

    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `GetIceServerConfig`.
    ///
    /// <p>Gets the Interactive Connectivity Establishment (ICE) server configuration information, including URIs, username, and password which can be used to configure the WebRTC connection. The ICE component uses this configuration information to setup the WebRTC connection, including authenticating with the Traversal Using Relays around NAT (TURN) relay server. </p>
    /// <p>TURN is a protocol that is used to improve the connectivity of peer-to-peer applications. By providing a cloud-based relay service, TURN ensures that a connection can be established even when one or more peers are incapable of a direct peer-to-peer connection. For more information, see <a href="https://tools.ietf.org/html/draft-uberti-rtcweb-turn-rest-00">A REST API For Access To TURN Services</a>.</p>
    /// <p> You can invoke this API to establish a fallback mechanism in case either of the peers is unable to establish a direct peer-to-peer connection over a signaling channel. You must specify either a signaling channel ARN or the client ID in order to invoke this API.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetIceServerConfig {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::get_ice_server_config_input::Builder,
    }
    impl GetIceServerConfig {
        /// Creates a new `GetIceServerConfig`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Consume this builder, creating a customizable operation that can be modified before being
        /// sent. The operation's inner [http::Request] can be modified as well.
        pub async fn customize(
            self,
        ) -> std::result::Result<
            crate::operation::customize::CustomizableOperation<
                crate::operation::GetIceServerConfig,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::GetIceServerConfigError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            Ok(crate::operation::customize::CustomizableOperation { handle, operation })
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
        ) -> std::result::Result<
            crate::output::GetIceServerConfigOutput,
            aws_smithy_http::result::SdkError<crate::error::GetIceServerConfigError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The ARN of the signaling channel to be used for the peer-to-peer connection between configured peers. </p>
        pub fn channel_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.channel_arn(input.into());
            self
        }
        /// <p>The ARN of the signaling channel to be used for the peer-to-peer connection between configured peers. </p>
        pub fn set_channel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_channel_arn(input);
            self
        }
        /// <p>Unique identifier for the viewer. Must be unique within the signaling channel.</p>
        pub fn client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_id(input.into());
            self
        }
        /// <p>Unique identifier for the viewer. Must be unique within the signaling channel.</p>
        pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_id(input);
            self
        }
        /// <p>Specifies the desired service. Currently, <code>TURN</code> is the only valid value.</p>
        pub fn service(mut self, input: crate::model::Service) -> Self {
            self.inner = self.inner.service(input);
            self
        }
        /// <p>Specifies the desired service. Currently, <code>TURN</code> is the only valid value.</p>
        pub fn set_service(mut self, input: std::option::Option<crate::model::Service>) -> Self {
            self.inner = self.inner.set_service(input);
            self
        }
        /// <p>An optional user ID to be associated with the credentials.</p>
        pub fn username(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.username(input.into());
            self
        }
        /// <p>An optional user ID to be associated with the credentials.</p>
        pub fn set_username(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_username(input);
            self
        }
    }
    /// Fluent builder constructing a request to `SendAlexaOfferToMaster`.
    ///
    /// <p>This API allows you to connect WebRTC-enabled devices with Alexa display devices. When invoked, it sends the Alexa Session Description Protocol (SDP) offer to the master peer. The offer is delivered as soon as the master is connected to the specified signaling channel. This API returns the SDP answer from the connected master. If the master is not connected to the signaling channel, redelivery requests are made until the message expires.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct SendAlexaOfferToMaster {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::send_alexa_offer_to_master_input::Builder,
    }
    impl SendAlexaOfferToMaster {
        /// Creates a new `SendAlexaOfferToMaster`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Consume this builder, creating a customizable operation that can be modified before being
        /// sent. The operation's inner [http::Request] can be modified as well.
        pub async fn customize(
            self,
        ) -> std::result::Result<
            crate::operation::customize::CustomizableOperation<
                crate::operation::SendAlexaOfferToMaster,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::SendAlexaOfferToMasterError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            Ok(crate::operation::customize::CustomizableOperation { handle, operation })
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
        ) -> std::result::Result<
            crate::output::SendAlexaOfferToMasterOutput,
            aws_smithy_http::result::SdkError<crate::error::SendAlexaOfferToMasterError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The ARN of the signaling channel by which Alexa and the master peer communicate.</p>
        pub fn channel_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.channel_arn(input.into());
            self
        }
        /// <p>The ARN of the signaling channel by which Alexa and the master peer communicate.</p>
        pub fn set_channel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_channel_arn(input);
            self
        }
        /// <p>The unique identifier for the sender client.</p>
        pub fn sender_client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.sender_client_id(input.into());
            self
        }
        /// <p>The unique identifier for the sender client.</p>
        pub fn set_sender_client_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_sender_client_id(input);
            self
        }
        /// <p>The base64-encoded SDP offer content.</p>
        pub fn message_payload(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.message_payload(input.into());
            self
        }
        /// <p>The base64-encoded SDP offer content.</p>
        pub fn set_message_payload(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_message_payload(input);
            self
        }
    }
}

impl Client {
    /// Creates a new client from an [SDK Config](aws_types::sdk_config::SdkConfig).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf
            .retry_config()
            .cloned()
            .unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
        let timeout_config = conf
            .timeout_config()
            .cloned()
            .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
        let sleep_impl = conf.sleep_impl();
        if (retry_config.has_retry() || timeout_config.has_timeouts()) && sleep_impl.is_none() {
            panic!("An async sleep implementation is required for retries or timeouts to work. \
                                    Set the `sleep_impl` on the Config passed into this function to fix this panic.");
        }

        let connector = conf.http_connector().and_then(|c| {
            let timeout_config = conf
                .timeout_config()
                .cloned()
                .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
            let connector_settings =
                aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                    &timeout_config,
                );
            c.connector(&connector_settings, conf.sleep_impl())
        });

        let builder = aws_smithy_client::Builder::new();
        let builder = match connector {
            // Use provided connector
            Some(c) => builder.connector(c),
            // Use default connector based on enabled features
            None => builder.dyn_https_connector(
                aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                    &timeout_config,
                ),
            ),
        };
        let mut builder = builder
            .middleware(aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ))
            .retry_config(retry_config.into())
            .operation_timeout_config(timeout_config.into());
        builder.set_sleep_impl(sleep_impl);
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
