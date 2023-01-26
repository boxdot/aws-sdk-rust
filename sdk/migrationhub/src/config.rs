// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Service config.
///
///
/// Service configuration allows for customization of endpoints, region, credentials providers,
/// and retry configuration. Generally, it is constructed automatically for you from a shared
/// configuration loaded by the `aws-config` crate. For example:
///
/// ```ignore
/// // Load a shared config from the environment
/// let shared_config = aws_config::from_env().load().await;
/// // The client constructor automatically converts the shared config into the service config
/// let client = Client::new(&shared_config);
/// ```
///
/// The service config can also be constructed manually using its builder.
///
pub struct Config {
    pub(crate) endpoint_resolver:
        std::sync::Arc<dyn aws_smithy_http::endpoint::ResolveEndpoint<crate::endpoint::Params>>,
    retry_config: Option<aws_smithy_types::retry::RetryConfig>,
    sleep_impl: Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>>,
    timeout_config: Option<aws_smithy_types::timeout::TimeoutConfig>,
    app_name: Option<aws_types::app_name::AppName>,
    #[allow(missing_docs)] // documentation missing in model
    pub(crate) endpoint_url: std::option::Option<std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub(crate) use_dual_stack: std::option::Option<std::primitive::bool>,
    #[allow(missing_docs)] // documentation missing in model
    pub(crate) use_fips: std::option::Option<std::primitive::bool>,
    http_connector: Option<aws_smithy_client::http_connector::HttpConnector>,
    pub(crate) region: Option<aws_types::region::Region>,
    pub(crate) credentials_cache: aws_credential_types::cache::SharedCredentialsCache,
}
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut config = f.debug_struct("Config");
        config.finish()
    }
}
impl Config {
    /// Constructs a config builder.
    pub fn builder() -> Builder {
        Builder::default()
    }
    /// Returns the endpoint resolver.
    pub fn endpoint_resolver(
        &self,
    ) -> std::sync::Arc<dyn aws_smithy_http::endpoint::ResolveEndpoint<crate::endpoint::Params>>
    {
        self.endpoint_resolver.clone()
    }
    /// Return a reference to the retry configuration contained in this config, if any.
    pub fn retry_config(&self) -> Option<&aws_smithy_types::retry::RetryConfig> {
        self.retry_config.as_ref()
    }

    /// Return a cloned Arc containing the async sleep implementation from this config, if any.
    pub fn sleep_impl(
        &self,
    ) -> Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>> {
        self.sleep_impl.clone()
    }

    /// Return a reference to the timeout configuration contained in this config, if any.
    pub fn timeout_config(&self) -> Option<&aws_smithy_types::timeout::TimeoutConfig> {
        self.timeout_config.as_ref()
    }
    /// Returns the name of the app that is using the client, if it was provided.
    ///
    /// This _optional_ name is used to identify the application in the user agent that
    /// gets sent along with requests.
    pub fn app_name(&self) -> Option<&aws_types::app_name::AppName> {
        self.app_name.as_ref()
    }
    /// Return an [`HttpConnector`](aws_smithy_client::http_connector::HttpConnector) to use when making requests, if any.
    pub fn http_connector(&self) -> Option<&aws_smithy_client::http_connector::HttpConnector> {
        self.http_connector.as_ref()
    }
    /// Creates a new [service config](crate::Config) from a [shared `config`](aws_types::sdk_config::SdkConfig).
    pub fn new(config: &aws_types::sdk_config::SdkConfig) -> Self {
        Builder::from(config).build()
    }
    /// The signature version 4 service signing name to use in the credential scope when signing requests.
    ///
    /// The signing service may be overridden by the `Endpoint`, or by specifying a custom
    /// [`SigningService`](aws_types::SigningService) during operation construction
    pub fn signing_service(&self) -> &'static str {
        "mgh"
    }
    /// Returns the AWS region, if it was provided.
    pub fn region(&self) -> Option<&aws_types::region::Region> {
        self.region.as_ref()
    }
    /// Returns the credentials cache.
    pub fn credentials_cache(&self) -> aws_credential_types::cache::SharedCredentialsCache {
        self.credentials_cache.clone()
    }
}
/// Builder for creating a `Config`.
#[derive(Default)]
pub struct Builder {
    endpoint_resolver: Option<
        std::sync::Arc<dyn aws_smithy_http::endpoint::ResolveEndpoint<crate::endpoint::Params>>,
    >,
    retry_config: Option<aws_smithy_types::retry::RetryConfig>,
    sleep_impl: Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>>,
    timeout_config: Option<aws_smithy_types::timeout::TimeoutConfig>,
    app_name: Option<aws_types::app_name::AppName>,
    endpoint_url: std::option::Option<std::string::String>,
    use_dual_stack: std::option::Option<std::primitive::bool>,
    use_fips: std::option::Option<std::primitive::bool>,
    http_connector: Option<aws_smithy_client::http_connector::HttpConnector>,
    region: Option<aws_types::region::Region>,
    credentials_provider: Option<aws_credential_types::provider::SharedCredentialsProvider>,
    credentials_cache: Option<aws_credential_types::cache::CredentialsCache>,
}
impl Builder {
    /// Constructs a config builder.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the endpoint resolver to use when making requests.

    ///
    /// When unset, the client will used a generated endpoint resolver based on the endpoint resolution
    /// rules for `aws_sdk_migrationhub`.
    ///
    /// # Examples
    /// ```no_run
    /// use aws_smithy_http::endpoint;
    /// use aws_sdk_migrationhub::endpoint::{Params as EndpointParams, DefaultResolver};
    /// /// Endpoint resolver which adds a prefix to the generated endpoint
    /// struct PrefixResolver {
    ///     base_resolver: DefaultResolver,
    ///     prefix: String
    /// }
    /// impl endpoint::ResolveEndpoint<EndpointParams> for PrefixResolver {
    ///   fn resolve_endpoint(&self, params: &EndpointParams) -> endpoint::Result {
    ///        self.base_resolver
    ///              .resolve_endpoint(params)
    ///              .map(|ep|{
    ///                   let url = ep.url().to_string();
    ///                   ep.into_builder().url(format!("{}.{}", &self.prefix, url)).build()
    ///               })
    ///   }
    /// }
    /// let prefix_resolver = PrefixResolver {
    ///     base_resolver: DefaultResolver::new(),
    ///     prefix: "subdomain".to_string()
    /// };
    /// let config = aws_sdk_migrationhub::Config::builder().endpoint_resolver(prefix_resolver);
    /// ```

    pub fn endpoint_resolver(
        mut self,
        endpoint_resolver: impl aws_smithy_http::endpoint::ResolveEndpoint<crate::endpoint::Params>
            + 'static,
    ) -> Self {
        self.endpoint_resolver = Some(std::sync::Arc::new(endpoint_resolver) as _);
        self
    }

    /// Sets the endpoint resolver to use when making requests.
    ///
    /// When unset, the client will used a generated endpoint resolver based on the endpoint resolution
    /// rules for `aws_sdk_migrationhub`.
    pub fn set_endpoint_resolver(
        &mut self,
        endpoint_resolver: Option<
            std::sync::Arc<dyn aws_smithy_http::endpoint::ResolveEndpoint<crate::endpoint::Params>>,
        >,
    ) -> &mut Self {
        self.endpoint_resolver = endpoint_resolver;
        self
    }
    /// Set the retry_config for the builder
    ///
    /// # Examples
    /// ```no_run
    /// use aws_sdk_migrationhub::config::Config;
    /// use aws_sdk_migrationhub::config::retry::RetryConfig;
    ///
    /// let retry_config = RetryConfig::standard().with_max_attempts(5);
    /// let config = Config::builder().retry_config(retry_config).build();
    /// ```
    pub fn retry_config(mut self, retry_config: aws_smithy_types::retry::RetryConfig) -> Self {
        self.set_retry_config(Some(retry_config));
        self
    }

    /// Set the retry_config for the builder
    ///
    /// # Examples
    /// ```no_run
    /// use aws_sdk_migrationhub::config::{Builder, Config};
    /// use aws_sdk_migrationhub::config::retry::RetryConfig;
    ///
    /// fn disable_retries(builder: &mut Builder) {
    ///     let retry_config = RetryConfig::standard().with_max_attempts(1);
    ///     builder.set_retry_config(Some(retry_config));
    /// }
    ///
    /// let mut builder = Config::builder();
    /// disable_retries(&mut builder);
    /// let config = builder.build();
    /// ```
    pub fn set_retry_config(
        &mut self,
        retry_config: Option<aws_smithy_types::retry::RetryConfig>,
    ) -> &mut Self {
        self.retry_config = retry_config;
        self
    }

    /// Set the sleep_impl for the builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use aws_sdk_migrationhub::config::{AsyncSleep, Sleep, Config};
    ///
    /// #[derive(Debug)]
    /// pub struct ForeverSleep;
    ///
    /// impl AsyncSleep for ForeverSleep {
    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
    ///         Sleep::new(std::future::pending())
    ///     }
    /// }
    ///
    /// let sleep_impl = std::sync::Arc::new(ForeverSleep);
    /// let config = Config::builder().sleep_impl(sleep_impl).build();
    /// ```
    pub fn sleep_impl(
        mut self,
        sleep_impl: std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>,
    ) -> Self {
        self.set_sleep_impl(Some(sleep_impl));
        self
    }

    /// Set the sleep_impl for the builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use aws_sdk_migrationhub::config::{AsyncSleep, Sleep, Builder, Config};
    ///
    /// #[derive(Debug)]
    /// pub struct ForeverSleep;
    ///
    /// impl AsyncSleep for ForeverSleep {
    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
    ///         Sleep::new(std::future::pending())
    ///     }
    /// }
    ///
    /// fn set_never_ending_sleep_impl(builder: &mut Builder) {
    ///     let sleep_impl = std::sync::Arc::new(ForeverSleep);
    ///     builder.set_sleep_impl(Some(sleep_impl));
    /// }
    ///
    /// let mut builder = Config::builder();
    /// set_never_ending_sleep_impl(&mut builder);
    /// let config = builder.build();
    /// ```
    pub fn set_sleep_impl(
        &mut self,
        sleep_impl: Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>>,
    ) -> &mut Self {
        self.sleep_impl = sleep_impl;
        self
    }

    /// Set the timeout_config for the builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::time::Duration;
    /// use aws_sdk_migrationhub::config::Config;
    /// use aws_sdk_migrationhub::config::timeout::TimeoutConfig;
    ///
    /// let timeout_config = TimeoutConfig::builder()
    ///     .operation_attempt_timeout(Duration::from_secs(1))
    ///     .build();
    /// let config = Config::builder().timeout_config(timeout_config).build();
    /// ```
    pub fn timeout_config(
        mut self,
        timeout_config: aws_smithy_types::timeout::TimeoutConfig,
    ) -> Self {
        self.set_timeout_config(Some(timeout_config));
        self
    }

    /// Set the timeout_config for the builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::time::Duration;
    /// use aws_sdk_migrationhub::config::{Builder, Config};
    /// use aws_sdk_migrationhub::config::timeout::TimeoutConfig;
    ///
    /// fn set_request_timeout(builder: &mut Builder) {
    ///     let timeout_config = TimeoutConfig::builder()
    ///         .operation_attempt_timeout(Duration::from_secs(1))
    ///         .build();
    ///     builder.set_timeout_config(Some(timeout_config));
    /// }
    ///
    /// let mut builder = Config::builder();
    /// set_request_timeout(&mut builder);
    /// let config = builder.build();
    /// ```
    pub fn set_timeout_config(
        &mut self,
        timeout_config: Option<aws_smithy_types::timeout::TimeoutConfig>,
    ) -> &mut Self {
        self.timeout_config = timeout_config;
        self
    }
    /// Overrides the endpoint resolver to use when making requests.
    ///
    /// This method is deprecated, use [`Builder::endpoint_url`] or [`Builder::endpoint_resolver`] instead.
    ///
    /// When unset, the client will used a generated endpoint resolver based on the endpoint metadata
    /// for `aws_sdk_migrationhub`.
    ///
    /// # Examples
    /// ```no_run
    /// # fn wrapper() -> Result<(), aws_smithy_http::endpoint::error::InvalidEndpointError> {
    /// use aws_types::region::Region;
    /// use aws_sdk_migrationhub::config::{Builder, Config};
    /// use aws_sdk_migrationhub::Endpoint;
    ///
    /// let config = aws_sdk_migrationhub::Config::builder()
    ///     .endpoint_resolver(Endpoint::immutable("http://localhost:8080")?)
    ///     .build();
    /// # Ok(())
    /// # }
    /// ```
    #[deprecated(note = "use endpoint_url or set the endpoint resolver directly")]
    pub fn aws_endpoint_resolver(
        mut self,
        endpoint_resolver: impl aws_endpoint::ResolveAwsEndpoint + 'static,
    ) -> Self {
        self.endpoint_resolver = Some(std::sync::Arc::new(
            aws_endpoint::EndpointShim::from_resolver(endpoint_resolver),
        ) as _);
        self
    }

    #[deprecated(note = "use endpoint_url or set the endpoint resolver directly")]
    /// Sets the endpoint resolver to use when making requests.
    ///
    /// This method is deprecated, use [`Builder::endpoint_url`] or [`Builder::endpoint_resolver`] instead.
    pub fn set_aws_endpoint_resolver(
        &mut self,
        endpoint_resolver: Option<std::sync::Arc<dyn aws_endpoint::ResolveAwsEndpoint>>,
    ) -> &mut Self {
        self.endpoint_resolver = endpoint_resolver
            .map(|res| std::sync::Arc::new(aws_endpoint::EndpointShim::from_arc(res)) as _);
        self
    }
    /// Sets the name of the app that is using the client.
    ///
    /// This _optional_ name is used to identify the application in the user agent that
    /// gets sent along with requests.
    pub fn app_name(mut self, app_name: aws_types::app_name::AppName) -> Self {
        self.set_app_name(Some(app_name));
        self
    }

    /// Sets the name of the app that is using the client.
    ///
    /// This _optional_ name is used to identify the application in the user agent that
    /// gets sent along with requests.
    pub fn set_app_name(&mut self, app_name: Option<aws_types::app_name::AppName>) -> &mut Self {
        self.app_name = app_name;
        self
    }
    /// Sets the endpoint url used to communicate with this service

    /// Note: this is used in combination with other endpoint rules, e.g. an API that applies a host-label prefix
    /// will be prefixed onto this URL. To fully override the endpoint resolver, use
    /// [`Builder::endpoint_resolver`].
    pub fn endpoint_url(mut self, endpoint_url: impl Into<std::string::String>) -> Self {
        self.endpoint_url = Some(endpoint_url.into());
        self
    }
    /// Sets the endpoint url used to communicate with this service

    /// Note: this is used in combination with other endpoint rules, e.g. an API that applies a host-label prefix
    /// will be prefixed onto this URL. To fully override the endpoint resolver, use
    /// [`Builder::endpoint_resolver`].
    pub fn set_endpoint_url(&mut self, endpoint_url: Option<std::string::String>) -> &mut Self {
        self.endpoint_url = endpoint_url;
        self
    }
    /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
    pub fn use_dual_stack(mut self, use_dual_stack: impl Into<std::primitive::bool>) -> Self {
        self.use_dual_stack = Some(use_dual_stack.into());
        self
    }
    /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
    pub fn set_use_dual_stack(
        &mut self,
        use_dual_stack: Option<std::primitive::bool>,
    ) -> &mut Self {
        self.use_dual_stack = use_dual_stack;
        self
    }
    /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
    pub fn use_fips(mut self, use_fips: impl Into<std::primitive::bool>) -> Self {
        self.use_fips = Some(use_fips.into());
        self
    }
    /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
    pub fn set_use_fips(&mut self, use_fips: Option<std::primitive::bool>) -> &mut Self {
        self.use_fips = use_fips;
        self
    }
    /// Sets the HTTP connector to use when making requests.
    ///
    /// # Examples
    /// ```no_run
    /// # #[cfg(test)]
    /// # mod tests {
    /// # #[test]
    /// # fn example() {
    /// use std::time::Duration;
    /// use aws_smithy_client::{Client, hyper_ext};
    /// use aws_smithy_client::erase::DynConnector;
    /// use aws_smithy_client::http_connector::ConnectorSettings;
    /// use aws_sdk_migrationhub::config::Config;
    ///
    /// let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
    ///     .with_webpki_roots()
    ///     .https_only()
    ///     .enable_http1()
    ///     .enable_http2()
    ///     .build();
    /// let smithy_connector = hyper_ext::Adapter::builder()
    ///     // Optionally set things like timeouts as well
    ///     .connector_settings(
    ///         ConnectorSettings::builder()
    ///             .connect_timeout(Duration::from_secs(5))
    ///             .build()
    ///     )
    ///     .build(https_connector);
    /// # }
    /// # }
    /// ```
    pub fn http_connector(
        mut self,
        http_connector: impl Into<aws_smithy_client::http_connector::HttpConnector>,
    ) -> Self {
        self.http_connector = Some(http_connector.into());
        self
    }

    /// Sets the HTTP connector to use when making requests.
    ///
    /// # Examples
    /// ```no_run
    /// # #[cfg(test)]
    /// # mod tests {
    /// # #[test]
    /// # fn example() {
    /// use std::time::Duration;
    /// use aws_smithy_client::hyper_ext;
    /// use aws_smithy_client::http_connector::ConnectorSettings;
    /// use crate::sdk_config::{SdkConfig, Builder};
    /// use aws_sdk_migrationhub::config::{Builder, Config};
    ///
    /// fn override_http_connector(builder: &mut Builder) {
    ///     let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
    ///         .with_webpki_roots()
    ///         .https_only()
    ///         .enable_http1()
    ///         .enable_http2()
    ///         .build();
    ///     let smithy_connector = hyper_ext::Adapter::builder()
    ///         // Optionally set things like timeouts as well
    ///         .connector_settings(
    ///             ConnectorSettings::builder()
    ///                 .connect_timeout(Duration::from_secs(5))
    ///                 .build()
    ///         )
    ///         .build(https_connector);
    ///     builder.set_http_connector(Some(smithy_connector));
    /// }
    ///
    /// let mut builder = aws_sdk_migrationhub::Config::builder();
    /// override_http_connector(&mut builder);
    /// let config = builder.build();
    /// # }
    /// # }
    /// ```
    pub fn set_http_connector(
        &mut self,
        http_connector: Option<impl Into<aws_smithy_client::http_connector::HttpConnector>>,
    ) -> &mut Self {
        self.http_connector = http_connector.map(|inner| inner.into());
        self
    }
    /// Sets the AWS region to use when making requests.
    ///
    /// # Examples
    /// ```no_run
    /// use aws_types::region::Region;
    /// use aws_sdk_migrationhub::config::{Builder, Config};
    ///
    /// let config = aws_sdk_migrationhub::Config::builder()
    ///     .region(Region::new("us-east-1"))
    ///     .build();
    /// ```
    pub fn region(mut self, region: impl Into<Option<aws_types::region::Region>>) -> Self {
        self.region = region.into();
        self
    }
    /// Sets the credentials provider for this service
    pub fn credentials_provider(
        mut self,
        credentials_provider: impl aws_credential_types::provider::ProvideCredentials + 'static,
    ) -> Self {
        self.set_credentials_provider(Some(
            aws_credential_types::provider::SharedCredentialsProvider::new(credentials_provider),
        ));
        self
    }

    /// Sets the credentials provider for this service
    pub fn set_credentials_provider(
        &mut self,
        credentials_provider: Option<aws_credential_types::provider::SharedCredentialsProvider>,
    ) -> &mut Self {
        self.credentials_provider = credentials_provider;
        self
    }
    /// Sets the credentials cache for this service
    pub fn credentials_cache(
        mut self,
        credentials_cache: aws_credential_types::cache::CredentialsCache,
    ) -> Self {
        self.set_credentials_cache(Some(credentials_cache));
        self
    }

    /// Sets the credentials cache for this service
    pub fn set_credentials_cache(
        &mut self,
        credentials_cache: Option<aws_credential_types::cache::CredentialsCache>,
    ) -> &mut Self {
        self.credentials_cache = credentials_cache;
        self
    }
    #[cfg(any(feature = "test-util", test))]
    #[allow(unused_mut)]
    /// Apply test defaults to the builder
    pub fn set_test_defaults(&mut self) -> &mut Self {
        self.set_credentials_provider(Some(
            aws_credential_types::provider::SharedCredentialsProvider::new(
                aws_credential_types::Credentials::for_tests(),
            ),
        ));
        self
    }
    #[cfg(any(feature = "test-util", test))]
    #[allow(unused_mut)]
    /// Apply test defaults to the builder
    pub fn with_test_defaults(mut self) -> Self {
        self.set_test_defaults();
        self
    }
    /// Builds a [`Config`].
    pub fn build(self) -> Config {
        Config {
            endpoint_resolver: self
                .endpoint_resolver
                .unwrap_or_else(|| std::sync::Arc::new(crate::endpoint::DefaultResolver::new())),
            retry_config: self.retry_config,
            sleep_impl: self.sleep_impl.clone(),
            timeout_config: self.timeout_config,
            app_name: self.app_name,
            endpoint_url: self.endpoint_url,
            use_dual_stack: self.use_dual_stack,
            use_fips: self.use_fips,
            http_connector: self.http_connector,
            region: self.region,
            credentials_cache: self
                .credentials_cache
                .unwrap_or_else({
                    let sleep = self.sleep_impl.clone();
                    || match sleep {
                        Some(sleep) => {
                            aws_credential_types::cache::CredentialsCache::lazy_builder()
                                .sleep(sleep)
                                .into_credentials_cache()
                        }
                        None => aws_credential_types::cache::CredentialsCache::lazy(),
                    }
                })
                .create_cache(self.credentials_provider.unwrap_or_else(|| {
                    aws_credential_types::provider::SharedCredentialsProvider::new(
                        crate::no_credentials::NoCredentials,
                    )
                })),
        }
    }
}

impl From<&aws_types::sdk_config::SdkConfig> for Builder {
    fn from(input: &aws_types::sdk_config::SdkConfig) -> Self {
        let mut builder = Builder::default();
        builder.set_credentials_cache(input.credentials_cache().cloned());

        builder.set_credentials_provider(input.credentials_provider().cloned());

        builder = builder.region(input.region().cloned());

        builder.set_use_fips(input.use_fips());

        builder.set_use_dual_stack(input.use_dual_stack());

        builder.set_endpoint_url(input.endpoint_url().map(|s| s.to_string()));

        // resiliency
        builder.set_retry_config(input.retry_config().cloned());
        builder.set_timeout_config(input.timeout_config().cloned());
        builder.set_sleep_impl(input.sleep_impl());

        builder.set_http_connector(input.http_connector().cloned());

        builder.set_app_name(input.app_name().cloned());

        builder.set_aws_endpoint_resolver(input.endpoint_resolver().clone());

        builder
    }
}

impl From<&aws_types::sdk_config::SdkConfig> for Config {
    fn from(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
        Builder::from(sdk_config).build()
    }
}

pub use aws_smithy_async::rt::sleep::{AsyncSleep, Sleep};

/// Retry configuration
///
/// These are re-exported from `aws-smithy-types` for convenience.
pub mod retry {
    pub use aws_smithy_types::retry::{RetryConfig, RetryConfigBuilder, RetryMode};
}
/// Timeout configuration
///
/// These are re-exported from `aws-smithy-types` for convenience.
pub mod timeout {
    pub use aws_smithy_types::timeout::{TimeoutConfig, TimeoutConfigBuilder};
}
