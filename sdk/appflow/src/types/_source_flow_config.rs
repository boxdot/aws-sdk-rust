// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Contains information about the configuration of the source connector used in the flow. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SourceFlowConfig {
    /// <p> The type of connector, such as Salesforce, Amplitude, and so on. </p>
    #[doc(hidden)]
    pub connector_type: ::std::option::Option<crate::types::ConnectorType>,
    /// <p>The API version of the connector when it's used as a source in the flow.</p>
    #[doc(hidden)]
    pub api_version: ::std::option::Option<::std::string::String>,
    /// <p> The name of the connector profile. This name must be unique for each connector profile in the Amazon Web Services account. </p>
    #[doc(hidden)]
    pub connector_profile_name: ::std::option::Option<::std::string::String>,
    /// <p> Specifies the information that is required to query a particular source connector. </p>
    #[doc(hidden)]
    pub source_connector_properties: ::std::option::Option<crate::types::SourceConnectorProperties>,
    /// <p> Defines the configuration for a scheduled incremental data pull. If a valid configuration is provided, the fields specified in the configuration are used when querying for the incremental data pull. </p>
    #[doc(hidden)]
    pub incremental_pull_config: ::std::option::Option<crate::types::IncrementalPullConfig>,
}
impl SourceFlowConfig {
    /// <p> The type of connector, such as Salesforce, Amplitude, and so on. </p>
    pub fn connector_type(&self) -> ::std::option::Option<&crate::types::ConnectorType> {
        self.connector_type.as_ref()
    }
    /// <p>The API version of the connector when it's used as a source in the flow.</p>
    pub fn api_version(&self) -> ::std::option::Option<&str> {
        self.api_version.as_deref()
    }
    /// <p> The name of the connector profile. This name must be unique for each connector profile in the Amazon Web Services account. </p>
    pub fn connector_profile_name(&self) -> ::std::option::Option<&str> {
        self.connector_profile_name.as_deref()
    }
    /// <p> Specifies the information that is required to query a particular source connector. </p>
    pub fn source_connector_properties(
        &self,
    ) -> ::std::option::Option<&crate::types::SourceConnectorProperties> {
        self.source_connector_properties.as_ref()
    }
    /// <p> Defines the configuration for a scheduled incremental data pull. If a valid configuration is provided, the fields specified in the configuration are used when querying for the incremental data pull. </p>
    pub fn incremental_pull_config(
        &self,
    ) -> ::std::option::Option<&crate::types::IncrementalPullConfig> {
        self.incremental_pull_config.as_ref()
    }
}
impl SourceFlowConfig {
    /// Creates a new builder-style object to manufacture [`SourceFlowConfig`](crate::types::SourceFlowConfig).
    pub fn builder() -> crate::types::builders::SourceFlowConfigBuilder {
        crate::types::builders::SourceFlowConfigBuilder::default()
    }
}

/// A builder for [`SourceFlowConfig`](crate::types::SourceFlowConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SourceFlowConfigBuilder {
    pub(crate) connector_type: ::std::option::Option<crate::types::ConnectorType>,
    pub(crate) api_version: ::std::option::Option<::std::string::String>,
    pub(crate) connector_profile_name: ::std::option::Option<::std::string::String>,
    pub(crate) source_connector_properties:
        ::std::option::Option<crate::types::SourceConnectorProperties>,
    pub(crate) incremental_pull_config: ::std::option::Option<crate::types::IncrementalPullConfig>,
}
impl SourceFlowConfigBuilder {
    /// <p> The type of connector, such as Salesforce, Amplitude, and so on. </p>
    pub fn connector_type(mut self, input: crate::types::ConnectorType) -> Self {
        self.connector_type = ::std::option::Option::Some(input);
        self
    }
    /// <p> The type of connector, such as Salesforce, Amplitude, and so on. </p>
    pub fn set_connector_type(
        mut self,
        input: ::std::option::Option<crate::types::ConnectorType>,
    ) -> Self {
        self.connector_type = input;
        self
    }
    /// <p>The API version of the connector when it's used as a source in the flow.</p>
    pub fn api_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.api_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The API version of the connector when it's used as a source in the flow.</p>
    pub fn set_api_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.api_version = input;
        self
    }
    /// <p> The name of the connector profile. This name must be unique for each connector profile in the Amazon Web Services account. </p>
    pub fn connector_profile_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.connector_profile_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name of the connector profile. This name must be unique for each connector profile in the Amazon Web Services account. </p>
    pub fn set_connector_profile_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.connector_profile_name = input;
        self
    }
    /// <p> Specifies the information that is required to query a particular source connector. </p>
    pub fn source_connector_properties(
        mut self,
        input: crate::types::SourceConnectorProperties,
    ) -> Self {
        self.source_connector_properties = ::std::option::Option::Some(input);
        self
    }
    /// <p> Specifies the information that is required to query a particular source connector. </p>
    pub fn set_source_connector_properties(
        mut self,
        input: ::std::option::Option<crate::types::SourceConnectorProperties>,
    ) -> Self {
        self.source_connector_properties = input;
        self
    }
    /// <p> Defines the configuration for a scheduled incremental data pull. If a valid configuration is provided, the fields specified in the configuration are used when querying for the incremental data pull. </p>
    pub fn incremental_pull_config(mut self, input: crate::types::IncrementalPullConfig) -> Self {
        self.incremental_pull_config = ::std::option::Option::Some(input);
        self
    }
    /// <p> Defines the configuration for a scheduled incremental data pull. If a valid configuration is provided, the fields specified in the configuration are used when querying for the incremental data pull. </p>
    pub fn set_incremental_pull_config(
        mut self,
        input: ::std::option::Option<crate::types::IncrementalPullConfig>,
    ) -> Self {
        self.incremental_pull_config = input;
        self
    }
    /// Consumes the builder and constructs a [`SourceFlowConfig`](crate::types::SourceFlowConfig).
    pub fn build(self) -> crate::types::SourceFlowConfig {
        crate::types::SourceFlowConfig {
            connector_type: self.connector_type,
            api_version: self.api_version,
            connector_profile_name: self.connector_profile_name,
            source_connector_properties: self.source_connector_properties,
            incremental_pull_config: self.incremental_pull_config,
        }
    }
}
