// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the configuration for experiment logging.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateExperimentTemplateLogConfigurationInput {
    /// <p>The configuration for experiment logging to Amazon CloudWatch Logs.</p>
    #[doc(hidden)]
    pub cloud_watch_logs_configuration:
        ::std::option::Option<crate::types::ExperimentTemplateCloudWatchLogsLogConfigurationInput>,
    /// <p>The configuration for experiment logging to Amazon S3.</p>
    #[doc(hidden)]
    pub s3_configuration:
        ::std::option::Option<crate::types::ExperimentTemplateS3LogConfigurationInput>,
    /// <p>The schema version.</p>
    #[doc(hidden)]
    pub log_schema_version: ::std::option::Option<i32>,
}
impl UpdateExperimentTemplateLogConfigurationInput {
    /// <p>The configuration for experiment logging to Amazon CloudWatch Logs.</p>
    pub fn cloud_watch_logs_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::ExperimentTemplateCloudWatchLogsLogConfigurationInput>
    {
        self.cloud_watch_logs_configuration.as_ref()
    }
    /// <p>The configuration for experiment logging to Amazon S3.</p>
    pub fn s3_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::ExperimentTemplateS3LogConfigurationInput> {
        self.s3_configuration.as_ref()
    }
    /// <p>The schema version.</p>
    pub fn log_schema_version(&self) -> ::std::option::Option<i32> {
        self.log_schema_version
    }
}
impl UpdateExperimentTemplateLogConfigurationInput {
    /// Creates a new builder-style object to manufacture [`UpdateExperimentTemplateLogConfigurationInput`](crate::types::UpdateExperimentTemplateLogConfigurationInput).
    pub fn builder() -> crate::types::builders::UpdateExperimentTemplateLogConfigurationInputBuilder
    {
        crate::types::builders::UpdateExperimentTemplateLogConfigurationInputBuilder::default()
    }
}

/// A builder for [`UpdateExperimentTemplateLogConfigurationInput`](crate::types::UpdateExperimentTemplateLogConfigurationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateExperimentTemplateLogConfigurationInputBuilder {
    pub(crate) cloud_watch_logs_configuration:
        ::std::option::Option<crate::types::ExperimentTemplateCloudWatchLogsLogConfigurationInput>,
    pub(crate) s3_configuration:
        ::std::option::Option<crate::types::ExperimentTemplateS3LogConfigurationInput>,
    pub(crate) log_schema_version: ::std::option::Option<i32>,
}
impl UpdateExperimentTemplateLogConfigurationInputBuilder {
    /// <p>The configuration for experiment logging to Amazon CloudWatch Logs.</p>
    pub fn cloud_watch_logs_configuration(
        mut self,
        input: crate::types::ExperimentTemplateCloudWatchLogsLogConfigurationInput,
    ) -> Self {
        self.cloud_watch_logs_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration for experiment logging to Amazon CloudWatch Logs.</p>
    pub fn set_cloud_watch_logs_configuration(
        mut self,
        input: ::std::option::Option<
            crate::types::ExperimentTemplateCloudWatchLogsLogConfigurationInput,
        >,
    ) -> Self {
        self.cloud_watch_logs_configuration = input;
        self
    }
    /// <p>The configuration for experiment logging to Amazon S3.</p>
    pub fn s3_configuration(
        mut self,
        input: crate::types::ExperimentTemplateS3LogConfigurationInput,
    ) -> Self {
        self.s3_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration for experiment logging to Amazon S3.</p>
    pub fn set_s3_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ExperimentTemplateS3LogConfigurationInput>,
    ) -> Self {
        self.s3_configuration = input;
        self
    }
    /// <p>The schema version.</p>
    pub fn log_schema_version(mut self, input: i32) -> Self {
        self.log_schema_version = ::std::option::Option::Some(input);
        self
    }
    /// <p>The schema version.</p>
    pub fn set_log_schema_version(mut self, input: ::std::option::Option<i32>) -> Self {
        self.log_schema_version = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateExperimentTemplateLogConfigurationInput`](crate::types::UpdateExperimentTemplateLogConfigurationInput).
    pub fn build(self) -> crate::types::UpdateExperimentTemplateLogConfigurationInput {
        crate::types::UpdateExperimentTemplateLogConfigurationInput {
            cloud_watch_logs_configuration: self.cloud_watch_logs_configuration,
            s3_configuration: self.s3_configuration,
            log_schema_version: self.log_schema_version,
        }
    }
}
