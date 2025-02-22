// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Configuration setting for monitoring.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MonitoringConfiguration {
    /// <p>Monitoring configurations for the persistent application UI. </p>
    #[doc(hidden)]
    pub persistent_app_ui: ::std::option::Option<crate::types::PersistentAppUi>,
    /// <p>Monitoring configurations for CloudWatch.</p>
    #[doc(hidden)]
    pub cloud_watch_monitoring_configuration:
        ::std::option::Option<crate::types::CloudWatchMonitoringConfiguration>,
    /// <p>Amazon S3 configuration for monitoring log publishing.</p>
    #[doc(hidden)]
    pub s3_monitoring_configuration: ::std::option::Option<crate::types::S3MonitoringConfiguration>,
}
impl MonitoringConfiguration {
    /// <p>Monitoring configurations for the persistent application UI. </p>
    pub fn persistent_app_ui(&self) -> ::std::option::Option<&crate::types::PersistentAppUi> {
        self.persistent_app_ui.as_ref()
    }
    /// <p>Monitoring configurations for CloudWatch.</p>
    pub fn cloud_watch_monitoring_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::CloudWatchMonitoringConfiguration> {
        self.cloud_watch_monitoring_configuration.as_ref()
    }
    /// <p>Amazon S3 configuration for monitoring log publishing.</p>
    pub fn s3_monitoring_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::S3MonitoringConfiguration> {
        self.s3_monitoring_configuration.as_ref()
    }
}
impl MonitoringConfiguration {
    /// Creates a new builder-style object to manufacture [`MonitoringConfiguration`](crate::types::MonitoringConfiguration).
    pub fn builder() -> crate::types::builders::MonitoringConfigurationBuilder {
        crate::types::builders::MonitoringConfigurationBuilder::default()
    }
}

/// A builder for [`MonitoringConfiguration`](crate::types::MonitoringConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MonitoringConfigurationBuilder {
    pub(crate) persistent_app_ui: ::std::option::Option<crate::types::PersistentAppUi>,
    pub(crate) cloud_watch_monitoring_configuration:
        ::std::option::Option<crate::types::CloudWatchMonitoringConfiguration>,
    pub(crate) s3_monitoring_configuration:
        ::std::option::Option<crate::types::S3MonitoringConfiguration>,
}
impl MonitoringConfigurationBuilder {
    /// <p>Monitoring configurations for the persistent application UI. </p>
    pub fn persistent_app_ui(mut self, input: crate::types::PersistentAppUi) -> Self {
        self.persistent_app_ui = ::std::option::Option::Some(input);
        self
    }
    /// <p>Monitoring configurations for the persistent application UI. </p>
    pub fn set_persistent_app_ui(
        mut self,
        input: ::std::option::Option<crate::types::PersistentAppUi>,
    ) -> Self {
        self.persistent_app_ui = input;
        self
    }
    /// <p>Monitoring configurations for CloudWatch.</p>
    pub fn cloud_watch_monitoring_configuration(
        mut self,
        input: crate::types::CloudWatchMonitoringConfiguration,
    ) -> Self {
        self.cloud_watch_monitoring_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Monitoring configurations for CloudWatch.</p>
    pub fn set_cloud_watch_monitoring_configuration(
        mut self,
        input: ::std::option::Option<crate::types::CloudWatchMonitoringConfiguration>,
    ) -> Self {
        self.cloud_watch_monitoring_configuration = input;
        self
    }
    /// <p>Amazon S3 configuration for monitoring log publishing.</p>
    pub fn s3_monitoring_configuration(
        mut self,
        input: crate::types::S3MonitoringConfiguration,
    ) -> Self {
        self.s3_monitoring_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Amazon S3 configuration for monitoring log publishing.</p>
    pub fn set_s3_monitoring_configuration(
        mut self,
        input: ::std::option::Option<crate::types::S3MonitoringConfiguration>,
    ) -> Self {
        self.s3_monitoring_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`MonitoringConfiguration`](crate::types::MonitoringConfiguration).
    pub fn build(self) -> crate::types::MonitoringConfiguration {
        crate::types::MonitoringConfiguration {
            persistent_app_ui: self.persistent_app_ui,
            cloud_watch_monitoring_configuration: self.cloud_watch_monitoring_configuration,
            s3_monitoring_configuration: self.s3_monitoring_configuration,
        }
    }
}
