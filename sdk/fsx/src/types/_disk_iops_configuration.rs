// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The SSD IOPS (input/output operations per second) configuration for an Amazon FSx for NetApp ONTAP or Amazon FSx for OpenZFS file system. The default is 3 IOPS per GB of storage capacity, but you can provision additional IOPS per GB of storage. The configuration consists of the total number of provisioned SSD IOPS and how the amount was provisioned (by the customer or by the system).</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DiskIopsConfiguration {
    /// <p>Specifies whether the number of IOPS for the file system is using the system default (<code>AUTOMATIC</code>) or was provisioned by the customer (<code>USER_PROVISIONED</code>).</p>
    #[doc(hidden)]
    pub mode: ::std::option::Option<crate::types::DiskIopsConfigurationMode>,
    /// <p>The total number of SSD IOPS provisioned for the file system.</p>
    #[doc(hidden)]
    pub iops: ::std::option::Option<i64>,
}
impl DiskIopsConfiguration {
    /// <p>Specifies whether the number of IOPS for the file system is using the system default (<code>AUTOMATIC</code>) or was provisioned by the customer (<code>USER_PROVISIONED</code>).</p>
    pub fn mode(&self) -> ::std::option::Option<&crate::types::DiskIopsConfigurationMode> {
        self.mode.as_ref()
    }
    /// <p>The total number of SSD IOPS provisioned for the file system.</p>
    pub fn iops(&self) -> ::std::option::Option<i64> {
        self.iops
    }
}
impl DiskIopsConfiguration {
    /// Creates a new builder-style object to manufacture [`DiskIopsConfiguration`](crate::types::DiskIopsConfiguration).
    pub fn builder() -> crate::types::builders::DiskIopsConfigurationBuilder {
        crate::types::builders::DiskIopsConfigurationBuilder::default()
    }
}

/// A builder for [`DiskIopsConfiguration`](crate::types::DiskIopsConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DiskIopsConfigurationBuilder {
    pub(crate) mode: ::std::option::Option<crate::types::DiskIopsConfigurationMode>,
    pub(crate) iops: ::std::option::Option<i64>,
}
impl DiskIopsConfigurationBuilder {
    /// <p>Specifies whether the number of IOPS for the file system is using the system default (<code>AUTOMATIC</code>) or was provisioned by the customer (<code>USER_PROVISIONED</code>).</p>
    pub fn mode(mut self, input: crate::types::DiskIopsConfigurationMode) -> Self {
        self.mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether the number of IOPS for the file system is using the system default (<code>AUTOMATIC</code>) or was provisioned by the customer (<code>USER_PROVISIONED</code>).</p>
    pub fn set_mode(
        mut self,
        input: ::std::option::Option<crate::types::DiskIopsConfigurationMode>,
    ) -> Self {
        self.mode = input;
        self
    }
    /// <p>The total number of SSD IOPS provisioned for the file system.</p>
    pub fn iops(mut self, input: i64) -> Self {
        self.iops = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total number of SSD IOPS provisioned for the file system.</p>
    pub fn set_iops(mut self, input: ::std::option::Option<i64>) -> Self {
        self.iops = input;
        self
    }
    /// Consumes the builder and constructs a [`DiskIopsConfiguration`](crate::types::DiskIopsConfiguration).
    pub fn build(self) -> crate::types::DiskIopsConfiguration {
        crate::types::DiskIopsConfiguration {
            mode: self.mode,
            iops: self.iops,
        }
    }
}
