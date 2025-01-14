// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Archive Container Settings
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ArchiveContainerSettings {
    /// M2ts Settings
    #[doc(hidden)]
    pub m2ts_settings: ::std::option::Option<crate::types::M2tsSettings>,
    /// Raw Settings
    #[doc(hidden)]
    pub raw_settings: ::std::option::Option<crate::types::RawSettings>,
}
impl ArchiveContainerSettings {
    /// M2ts Settings
    pub fn m2ts_settings(&self) -> ::std::option::Option<&crate::types::M2tsSettings> {
        self.m2ts_settings.as_ref()
    }
    /// Raw Settings
    pub fn raw_settings(&self) -> ::std::option::Option<&crate::types::RawSettings> {
        self.raw_settings.as_ref()
    }
}
impl ArchiveContainerSettings {
    /// Creates a new builder-style object to manufacture [`ArchiveContainerSettings`](crate::types::ArchiveContainerSettings).
    pub fn builder() -> crate::types::builders::ArchiveContainerSettingsBuilder {
        crate::types::builders::ArchiveContainerSettingsBuilder::default()
    }
}

/// A builder for [`ArchiveContainerSettings`](crate::types::ArchiveContainerSettings).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ArchiveContainerSettingsBuilder {
    pub(crate) m2ts_settings: ::std::option::Option<crate::types::M2tsSettings>,
    pub(crate) raw_settings: ::std::option::Option<crate::types::RawSettings>,
}
impl ArchiveContainerSettingsBuilder {
    /// M2ts Settings
    pub fn m2ts_settings(mut self, input: crate::types::M2tsSettings) -> Self {
        self.m2ts_settings = ::std::option::Option::Some(input);
        self
    }
    /// M2ts Settings
    pub fn set_m2ts_settings(
        mut self,
        input: ::std::option::Option<crate::types::M2tsSettings>,
    ) -> Self {
        self.m2ts_settings = input;
        self
    }
    /// Raw Settings
    pub fn raw_settings(mut self, input: crate::types::RawSettings) -> Self {
        self.raw_settings = ::std::option::Option::Some(input);
        self
    }
    /// Raw Settings
    pub fn set_raw_settings(
        mut self,
        input: ::std::option::Option<crate::types::RawSettings>,
    ) -> Self {
        self.raw_settings = input;
        self
    }
    /// Consumes the builder and constructs a [`ArchiveContainerSettings`](crate::types::ArchiveContainerSettings).
    pub fn build(self) -> crate::types::ArchiveContainerSettings {
        crate::types::ArchiveContainerSettings {
            m2ts_settings: self.m2ts_settings,
            raw_settings: self.raw_settings,
        }
    }
}
