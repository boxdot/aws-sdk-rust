// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) client: aws_smithy_client::Client<
        aws_smithy_client::erase::DynConnector,
        aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
    >,
    pub(crate) conf: crate::Config,
}

/// Client for AWS Control Tower
///
/// Client for invoking operations on AWS Control Tower. Each operation on AWS Control Tower is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_controltower::Client::new(&shared_config);
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
/// let config = aws_sdk_controltower::config::Builder::from(&shared_config)
///   .retry_config(RetryConfig::disabled())
///   .build();
/// let client = aws_sdk_controltower::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`DisableControl`](crate::client::fluent_builders::DisableControl) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`control_identifier(impl Into<String>)`](crate::client::fluent_builders::DisableControl::control_identifier) / [`set_control_identifier(Option<String>)`](crate::client::fluent_builders::DisableControl::set_control_identifier): <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
    ///   - [`target_identifier(impl Into<String>)`](crate::client::fluent_builders::DisableControl::target_identifier) / [`set_target_identifier(Option<String>)`](crate::client::fluent_builders::DisableControl::set_target_identifier): <p>The ARN of the organizational unit.</p>
    /// - On success, responds with [`DisableControlOutput`](crate::output::DisableControlOutput) with field(s):
    ///   - [`operation_identifier(Option<String>)`](crate::output::DisableControlOutput::operation_identifier): <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
    /// - On failure, responds with [`SdkError<DisableControlError>`](crate::error::DisableControlError)
    pub fn disable_control(&self) -> fluent_builders::DisableControl {
        fluent_builders::DisableControl::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`EnableControl`](crate::client::fluent_builders::EnableControl) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`control_identifier(impl Into<String>)`](crate::client::fluent_builders::EnableControl::control_identifier) / [`set_control_identifier(Option<String>)`](crate::client::fluent_builders::EnableControl::set_control_identifier): <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
    ///   - [`target_identifier(impl Into<String>)`](crate::client::fluent_builders::EnableControl::target_identifier) / [`set_target_identifier(Option<String>)`](crate::client::fluent_builders::EnableControl::set_target_identifier): <p>The ARN of the organizational unit.</p>
    /// - On success, responds with [`EnableControlOutput`](crate::output::EnableControlOutput) with field(s):
    ///   - [`operation_identifier(Option<String>)`](crate::output::EnableControlOutput::operation_identifier): <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
    /// - On failure, responds with [`SdkError<EnableControlError>`](crate::error::EnableControlError)
    pub fn enable_control(&self) -> fluent_builders::EnableControl {
        fluent_builders::EnableControl::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`GetControlOperation`](crate::client::fluent_builders::GetControlOperation) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`operation_identifier(impl Into<String>)`](crate::client::fluent_builders::GetControlOperation::operation_identifier) / [`set_operation_identifier(Option<String>)`](crate::client::fluent_builders::GetControlOperation::set_operation_identifier): <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
    /// - On success, responds with [`GetControlOperationOutput`](crate::output::GetControlOperationOutput) with field(s):
    ///   - [`control_operation(Option<ControlOperation>)`](crate::output::GetControlOperationOutput::control_operation): <p></p>
    /// - On failure, responds with [`SdkError<GetControlOperationError>`](crate::error::GetControlOperationError)
    pub fn get_control_operation(&self) -> fluent_builders::GetControlOperation {
        fluent_builders::GetControlOperation::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`ListEnabledControls`](crate::client::fluent_builders::ListEnabledControls) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListEnabledControls::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`target_identifier(impl Into<String>)`](crate::client::fluent_builders::ListEnabledControls::target_identifier) / [`set_target_identifier(Option<String>)`](crate::client::fluent_builders::ListEnabledControls::set_target_identifier): <p>The ARN of the organizational unit.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListEnabledControls::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListEnabledControls::set_next_token): <p>The token to continue the list from a previous API call with the same parameters.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListEnabledControls::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListEnabledControls::set_max_results): <p>How many results to return per API call.</p>
    /// - On success, responds with [`ListEnabledControlsOutput`](crate::output::ListEnabledControlsOutput) with field(s):
    ///   - [`enabled_controls(Option<Vec<EnabledControlSummary>>)`](crate::output::ListEnabledControlsOutput::enabled_controls): <p>Lists the controls enabled by AWS Control Tower on the specified organizational unit and the accounts it contains.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListEnabledControlsOutput::next_token): <p>Retrieves the next page of results. If the string is empty, the current response is the end of the results.</p>
    /// - On failure, responds with [`SdkError<ListEnabledControlsError>`](crate::error::ListEnabledControlsError)
    pub fn list_enabled_controls(&self) -> fluent_builders::ListEnabledControls {
        fluent_builders::ListEnabledControls::new(self.handle.clone())
    }
}
pub mod fluent_builders {

    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `DisableControl`.
    ///
    /// <p>This API call turns off a control. It starts an asynchronous operation that deletes AWS resources on the specified organizational unit and the accounts it contains. The resources will vary according to the control that you specify.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DisableControl {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::disable_control_input::Builder,
    }
    impl DisableControl {
        /// Creates a new `DisableControl`.
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
                crate::operation::DisableControl,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::DisableControlError>,
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
            crate::output::DisableControlOutput,
            aws_smithy_http::result::SdkError<crate::error::DisableControlError>,
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
        /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
        pub fn control_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.control_identifier(input.into());
            self
        }
        /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
        pub fn set_control_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_control_identifier(input);
            self
        }
        /// <p>The ARN of the organizational unit.</p>
        pub fn target_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_identifier(input.into());
            self
        }
        /// <p>The ARN of the organizational unit.</p>
        pub fn set_target_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_target_identifier(input);
            self
        }
    }
    /// Fluent builder constructing a request to `EnableControl`.
    ///
    /// <p>This API call activates a control. It starts an asynchronous operation that creates AWS resources on the specified organizational unit and the accounts it contains. The resources created will vary according to the control that you specify.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct EnableControl {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::enable_control_input::Builder,
    }
    impl EnableControl {
        /// Creates a new `EnableControl`.
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
                crate::operation::EnableControl,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::EnableControlError>,
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
            crate::output::EnableControlOutput,
            aws_smithy_http::result::SdkError<crate::error::EnableControlError>,
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
        /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
        pub fn control_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.control_identifier(input.into());
            self
        }
        /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
        pub fn set_control_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_control_identifier(input);
            self
        }
        /// <p>The ARN of the organizational unit.</p>
        pub fn target_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_identifier(input.into());
            self
        }
        /// <p>The ARN of the organizational unit.</p>
        pub fn set_target_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_target_identifier(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetControlOperation`.
    ///
    /// <p>Returns the status of a particular <code>EnableControl</code> or <code>DisableControl</code> operation. Displays a message in case of error. Details for an operation are available for 90 days.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetControlOperation {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::get_control_operation_input::Builder,
    }
    impl GetControlOperation {
        /// Creates a new `GetControlOperation`.
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
                crate::operation::GetControlOperation,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::GetControlOperationError>,
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
            crate::output::GetControlOperationOutput,
            aws_smithy_http::result::SdkError<crate::error::GetControlOperationError>,
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
        /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
        pub fn operation_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.operation_identifier(input.into());
            self
        }
        /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
        pub fn set_operation_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_operation_identifier(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListEnabledControls`.
    ///
    /// <p>Lists the controls enabled by AWS Control Tower on the specified organizational unit and the accounts it contains.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListEnabledControls {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::list_enabled_controls_input::Builder,
    }
    impl ListEnabledControls {
        /// Creates a new `ListEnabledControls`.
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
                crate::operation::ListEnabledControls,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::ListEnabledControlsError>,
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
            crate::output::ListEnabledControlsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListEnabledControlsError>,
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
        /// Create a paginator for this request
        ///
        /// Paginators are used by calling [`send().await`](crate::paginator::ListEnabledControlsPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
        pub fn into_paginator(self) -> crate::paginator::ListEnabledControlsPaginator {
            crate::paginator::ListEnabledControlsPaginator::new(self.handle, self.inner)
        }
        /// <p>The ARN of the organizational unit.</p>
        pub fn target_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_identifier(input.into());
            self
        }
        /// <p>The ARN of the organizational unit.</p>
        pub fn set_target_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_target_identifier(input);
            self
        }
        /// <p>The token to continue the list from a previous API call with the same parameters.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>The token to continue the list from a previous API call with the same parameters.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>How many results to return per API call.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>How many results to return per API call.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
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
