// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Source server properties.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SourceProperties {
    /// <p>Source server last update date and time.</p>
    #[doc(hidden)]
    pub last_updated_date_time: ::std::option::Option<::std::string::String>,
    /// <p>Source server recommended instance type.</p>
    #[doc(hidden)]
    pub recommended_instance_type: ::std::option::Option<::std::string::String>,
    /// <p>Source server identification hints.</p>
    #[doc(hidden)]
    pub identification_hints: ::std::option::Option<crate::types::IdentificationHints>,
    /// <p>Source server network interfaces.</p>
    #[doc(hidden)]
    pub network_interfaces: ::std::option::Option<::std::vec::Vec<crate::types::NetworkInterface>>,
    /// <p>Source Server disks.</p>
    #[doc(hidden)]
    pub disks: ::std::option::Option<::std::vec::Vec<crate::types::Disk>>,
    /// <p>Source Server CPUs.</p>
    #[doc(hidden)]
    pub cpus: ::std::option::Option<::std::vec::Vec<crate::types::Cpu>>,
    /// <p>Source server RAM in bytes.</p>
    #[doc(hidden)]
    pub ram_bytes: i64,
    /// <p>Source server OS.</p>
    #[doc(hidden)]
    pub os: ::std::option::Option<crate::types::Os>,
}
impl SourceProperties {
    /// <p>Source server last update date and time.</p>
    pub fn last_updated_date_time(&self) -> ::std::option::Option<&str> {
        self.last_updated_date_time.as_deref()
    }
    /// <p>Source server recommended instance type.</p>
    pub fn recommended_instance_type(&self) -> ::std::option::Option<&str> {
        self.recommended_instance_type.as_deref()
    }
    /// <p>Source server identification hints.</p>
    pub fn identification_hints(
        &self,
    ) -> ::std::option::Option<&crate::types::IdentificationHints> {
        self.identification_hints.as_ref()
    }
    /// <p>Source server network interfaces.</p>
    pub fn network_interfaces(&self) -> ::std::option::Option<&[crate::types::NetworkInterface]> {
        self.network_interfaces.as_deref()
    }
    /// <p>Source Server disks.</p>
    pub fn disks(&self) -> ::std::option::Option<&[crate::types::Disk]> {
        self.disks.as_deref()
    }
    /// <p>Source Server CPUs.</p>
    pub fn cpus(&self) -> ::std::option::Option<&[crate::types::Cpu]> {
        self.cpus.as_deref()
    }
    /// <p>Source server RAM in bytes.</p>
    pub fn ram_bytes(&self) -> i64 {
        self.ram_bytes
    }
    /// <p>Source server OS.</p>
    pub fn os(&self) -> ::std::option::Option<&crate::types::Os> {
        self.os.as_ref()
    }
}
impl SourceProperties {
    /// Creates a new builder-style object to manufacture [`SourceProperties`](crate::types::SourceProperties).
    pub fn builder() -> crate::types::builders::SourcePropertiesBuilder {
        crate::types::builders::SourcePropertiesBuilder::default()
    }
}

/// A builder for [`SourceProperties`](crate::types::SourceProperties).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SourcePropertiesBuilder {
    pub(crate) last_updated_date_time: ::std::option::Option<::std::string::String>,
    pub(crate) recommended_instance_type: ::std::option::Option<::std::string::String>,
    pub(crate) identification_hints: ::std::option::Option<crate::types::IdentificationHints>,
    pub(crate) network_interfaces:
        ::std::option::Option<::std::vec::Vec<crate::types::NetworkInterface>>,
    pub(crate) disks: ::std::option::Option<::std::vec::Vec<crate::types::Disk>>,
    pub(crate) cpus: ::std::option::Option<::std::vec::Vec<crate::types::Cpu>>,
    pub(crate) ram_bytes: ::std::option::Option<i64>,
    pub(crate) os: ::std::option::Option<crate::types::Os>,
}
impl SourcePropertiesBuilder {
    /// <p>Source server last update date and time.</p>
    pub fn last_updated_date_time(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.last_updated_date_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Source server last update date and time.</p>
    pub fn set_last_updated_date_time(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.last_updated_date_time = input;
        self
    }
    /// <p>Source server recommended instance type.</p>
    pub fn recommended_instance_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.recommended_instance_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Source server recommended instance type.</p>
    pub fn set_recommended_instance_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.recommended_instance_type = input;
        self
    }
    /// <p>Source server identification hints.</p>
    pub fn identification_hints(mut self, input: crate::types::IdentificationHints) -> Self {
        self.identification_hints = ::std::option::Option::Some(input);
        self
    }
    /// <p>Source server identification hints.</p>
    pub fn set_identification_hints(
        mut self,
        input: ::std::option::Option<crate::types::IdentificationHints>,
    ) -> Self {
        self.identification_hints = input;
        self
    }
    /// Appends an item to `network_interfaces`.
    ///
    /// To override the contents of this collection use [`set_network_interfaces`](Self::set_network_interfaces).
    ///
    /// <p>Source server network interfaces.</p>
    pub fn network_interfaces(mut self, input: crate::types::NetworkInterface) -> Self {
        let mut v = self.network_interfaces.unwrap_or_default();
        v.push(input);
        self.network_interfaces = ::std::option::Option::Some(v);
        self
    }
    /// <p>Source server network interfaces.</p>
    pub fn set_network_interfaces(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::NetworkInterface>>,
    ) -> Self {
        self.network_interfaces = input;
        self
    }
    /// Appends an item to `disks`.
    ///
    /// To override the contents of this collection use [`set_disks`](Self::set_disks).
    ///
    /// <p>Source Server disks.</p>
    pub fn disks(mut self, input: crate::types::Disk) -> Self {
        let mut v = self.disks.unwrap_or_default();
        v.push(input);
        self.disks = ::std::option::Option::Some(v);
        self
    }
    /// <p>Source Server disks.</p>
    pub fn set_disks(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Disk>>,
    ) -> Self {
        self.disks = input;
        self
    }
    /// Appends an item to `cpus`.
    ///
    /// To override the contents of this collection use [`set_cpus`](Self::set_cpus).
    ///
    /// <p>Source Server CPUs.</p>
    pub fn cpus(mut self, input: crate::types::Cpu) -> Self {
        let mut v = self.cpus.unwrap_or_default();
        v.push(input);
        self.cpus = ::std::option::Option::Some(v);
        self
    }
    /// <p>Source Server CPUs.</p>
    pub fn set_cpus(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Cpu>>,
    ) -> Self {
        self.cpus = input;
        self
    }
    /// <p>Source server RAM in bytes.</p>
    pub fn ram_bytes(mut self, input: i64) -> Self {
        self.ram_bytes = ::std::option::Option::Some(input);
        self
    }
    /// <p>Source server RAM in bytes.</p>
    pub fn set_ram_bytes(mut self, input: ::std::option::Option<i64>) -> Self {
        self.ram_bytes = input;
        self
    }
    /// <p>Source server OS.</p>
    pub fn os(mut self, input: crate::types::Os) -> Self {
        self.os = ::std::option::Option::Some(input);
        self
    }
    /// <p>Source server OS.</p>
    pub fn set_os(mut self, input: ::std::option::Option<crate::types::Os>) -> Self {
        self.os = input;
        self
    }
    /// Consumes the builder and constructs a [`SourceProperties`](crate::types::SourceProperties).
    pub fn build(self) -> crate::types::SourceProperties {
        crate::types::SourceProperties {
            last_updated_date_time: self.last_updated_date_time,
            recommended_instance_type: self.recommended_instance_type,
            identification_hints: self.identification_hints,
            network_interfaces: self.network_interfaces,
            disks: self.disks,
            cpus: self.cpus,
            ram_bytes: self.ram_bytes.unwrap_or_default(),
            os: self.os,
        }
    }
}
