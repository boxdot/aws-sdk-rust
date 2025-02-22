// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about agents or connectors associated with the user’s Amazon Web Services account. Information includes agent or connector IDs, IP addresses, media access control (MAC) addresses, agent or connector health, hostname where the agent or connector resides, and agent version for each agent.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AgentInfo {
    /// <p>The agent or connector ID.</p>
    #[doc(hidden)]
    pub agent_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the host where the agent or connector resides. The host can be a server or virtual machine.</p>
    #[doc(hidden)]
    pub host_name: ::std::option::Option<::std::string::String>,
    /// <p>Network details about the host where the agent or connector resides.</p>
    #[doc(hidden)]
    pub agent_network_info_list:
        ::std::option::Option<::std::vec::Vec<crate::types::AgentNetworkInfo>>,
    /// <p>The ID of the connector.</p>
    #[doc(hidden)]
    pub connector_id: ::std::option::Option<::std::string::String>,
    /// <p>The agent or connector version.</p>
    #[doc(hidden)]
    pub version: ::std::option::Option<::std::string::String>,
    /// <p>The health of the agent or connector.</p>
    #[doc(hidden)]
    pub health: ::std::option::Option<crate::types::AgentStatus>,
    /// <p>Time since agent or connector health was reported.</p>
    #[doc(hidden)]
    pub last_health_ping_time: ::std::option::Option<::std::string::String>,
    /// <p>Status of the collection process for an agent or connector.</p>
    #[doc(hidden)]
    pub collection_status: ::std::option::Option<::std::string::String>,
    /// <p>Type of agent.</p>
    #[doc(hidden)]
    pub agent_type: ::std::option::Option<::std::string::String>,
    /// <p>Agent's first registration timestamp in UTC.</p>
    #[doc(hidden)]
    pub registered_time: ::std::option::Option<::std::string::String>,
}
impl AgentInfo {
    /// <p>The agent or connector ID.</p>
    pub fn agent_id(&self) -> ::std::option::Option<&str> {
        self.agent_id.as_deref()
    }
    /// <p>The name of the host where the agent or connector resides. The host can be a server or virtual machine.</p>
    pub fn host_name(&self) -> ::std::option::Option<&str> {
        self.host_name.as_deref()
    }
    /// <p>Network details about the host where the agent or connector resides.</p>
    pub fn agent_network_info_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::AgentNetworkInfo]> {
        self.agent_network_info_list.as_deref()
    }
    /// <p>The ID of the connector.</p>
    pub fn connector_id(&self) -> ::std::option::Option<&str> {
        self.connector_id.as_deref()
    }
    /// <p>The agent or connector version.</p>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
    /// <p>The health of the agent or connector.</p>
    pub fn health(&self) -> ::std::option::Option<&crate::types::AgentStatus> {
        self.health.as_ref()
    }
    /// <p>Time since agent or connector health was reported.</p>
    pub fn last_health_ping_time(&self) -> ::std::option::Option<&str> {
        self.last_health_ping_time.as_deref()
    }
    /// <p>Status of the collection process for an agent or connector.</p>
    pub fn collection_status(&self) -> ::std::option::Option<&str> {
        self.collection_status.as_deref()
    }
    /// <p>Type of agent.</p>
    pub fn agent_type(&self) -> ::std::option::Option<&str> {
        self.agent_type.as_deref()
    }
    /// <p>Agent's first registration timestamp in UTC.</p>
    pub fn registered_time(&self) -> ::std::option::Option<&str> {
        self.registered_time.as_deref()
    }
}
impl AgentInfo {
    /// Creates a new builder-style object to manufacture [`AgentInfo`](crate::types::AgentInfo).
    pub fn builder() -> crate::types::builders::AgentInfoBuilder {
        crate::types::builders::AgentInfoBuilder::default()
    }
}

/// A builder for [`AgentInfo`](crate::types::AgentInfo).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AgentInfoBuilder {
    pub(crate) agent_id: ::std::option::Option<::std::string::String>,
    pub(crate) host_name: ::std::option::Option<::std::string::String>,
    pub(crate) agent_network_info_list:
        ::std::option::Option<::std::vec::Vec<crate::types::AgentNetworkInfo>>,
    pub(crate) connector_id: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
    pub(crate) health: ::std::option::Option<crate::types::AgentStatus>,
    pub(crate) last_health_ping_time: ::std::option::Option<::std::string::String>,
    pub(crate) collection_status: ::std::option::Option<::std::string::String>,
    pub(crate) agent_type: ::std::option::Option<::std::string::String>,
    pub(crate) registered_time: ::std::option::Option<::std::string::String>,
}
impl AgentInfoBuilder {
    /// <p>The agent or connector ID.</p>
    pub fn agent_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.agent_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The agent or connector ID.</p>
    pub fn set_agent_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.agent_id = input;
        self
    }
    /// <p>The name of the host where the agent or connector resides. The host can be a server or virtual machine.</p>
    pub fn host_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.host_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the host where the agent or connector resides. The host can be a server or virtual machine.</p>
    pub fn set_host_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.host_name = input;
        self
    }
    /// Appends an item to `agent_network_info_list`.
    ///
    /// To override the contents of this collection use [`set_agent_network_info_list`](Self::set_agent_network_info_list).
    ///
    /// <p>Network details about the host where the agent or connector resides.</p>
    pub fn agent_network_info_list(mut self, input: crate::types::AgentNetworkInfo) -> Self {
        let mut v = self.agent_network_info_list.unwrap_or_default();
        v.push(input);
        self.agent_network_info_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>Network details about the host where the agent or connector resides.</p>
    pub fn set_agent_network_info_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AgentNetworkInfo>>,
    ) -> Self {
        self.agent_network_info_list = input;
        self
    }
    /// <p>The ID of the connector.</p>
    pub fn connector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.connector_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the connector.</p>
    pub fn set_connector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.connector_id = input;
        self
    }
    /// <p>The agent or connector version.</p>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The agent or connector version.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// <p>The health of the agent or connector.</p>
    pub fn health(mut self, input: crate::types::AgentStatus) -> Self {
        self.health = ::std::option::Option::Some(input);
        self
    }
    /// <p>The health of the agent or connector.</p>
    pub fn set_health(mut self, input: ::std::option::Option<crate::types::AgentStatus>) -> Self {
        self.health = input;
        self
    }
    /// <p>Time since agent or connector health was reported.</p>
    pub fn last_health_ping_time(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.last_health_ping_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Time since agent or connector health was reported.</p>
    pub fn set_last_health_ping_time(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.last_health_ping_time = input;
        self
    }
    /// <p>Status of the collection process for an agent or connector.</p>
    pub fn collection_status(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.collection_status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Status of the collection process for an agent or connector.</p>
    pub fn set_collection_status(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.collection_status = input;
        self
    }
    /// <p>Type of agent.</p>
    pub fn agent_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.agent_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Type of agent.</p>
    pub fn set_agent_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.agent_type = input;
        self
    }
    /// <p>Agent's first registration timestamp in UTC.</p>
    pub fn registered_time(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.registered_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Agent's first registration timestamp in UTC.</p>
    pub fn set_registered_time(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.registered_time = input;
        self
    }
    /// Consumes the builder and constructs a [`AgentInfo`](crate::types::AgentInfo).
    pub fn build(self) -> crate::types::AgentInfo {
        crate::types::AgentInfo {
            agent_id: self.agent_id,
            host_name: self.host_name,
            agent_network_info_list: self.agent_network_info_list,
            connector_id: self.connector_id,
            version: self.version,
            health: self.health,
            last_health_ping_time: self.last_health_ping_time,
            collection_status: self.collection_status,
            agent_type: self.agent_type,
            registered_time: self.registered_time,
        }
    }
}
