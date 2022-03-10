// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) client: aws_smithy_client::Client<
        aws_smithy_client::erase::DynConnector,
        aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
    >,
    pub(crate) conf: crate::Config,
}

/// Client for AWS IoT Jobs Data Plane
///
/// Client for invoking operations on AWS IoT Jobs Data Plane. Each operation on AWS IoT Jobs Data Plane is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_iotjobsdataplane::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::RetryConfig;
/// # async fn docs() {
///     let shared_config = aws_config::load_from_env().await;
///     let config = aws_sdk_iotjobsdataplane::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_iotjobsdataplane::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`DescribeJobExecution`](crate::client::fluent_builders::DescribeJobExecution) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`job_id(impl Into<String>)`](crate::client::fluent_builders::DescribeJobExecution::job_id) / [`set_job_id(Option<String>)`](crate::client::fluent_builders::DescribeJobExecution::set_job_id): <p>The unique identifier assigned to this job when it was created.</p>
    ///   - [`thing_name(impl Into<String>)`](crate::client::fluent_builders::DescribeJobExecution::thing_name) / [`set_thing_name(Option<String>)`](crate::client::fluent_builders::DescribeJobExecution::set_thing_name): <p>The thing name associated with the device the job execution is running on.</p>
    ///   - [`include_job_document(bool)`](crate::client::fluent_builders::DescribeJobExecution::include_job_document) / [`set_include_job_document(Option<bool>)`](crate::client::fluent_builders::DescribeJobExecution::set_include_job_document): <p>Optional. When set to true, the response contains the job document. The default is false.</p>
    ///   - [`execution_number(i64)`](crate::client::fluent_builders::DescribeJobExecution::execution_number) / [`set_execution_number(Option<i64>)`](crate::client::fluent_builders::DescribeJobExecution::set_execution_number): <p>Optional. A number that identifies a particular job execution on a particular device. If not specified, the latest job execution is returned.</p>
    /// - On success, responds with [`DescribeJobExecutionOutput`](crate::output::DescribeJobExecutionOutput) with field(s):
    ///   - [`execution(Option<JobExecution>)`](crate::output::DescribeJobExecutionOutput::execution): <p>Contains data about a job execution.</p>
    /// - On failure, responds with [`SdkError<DescribeJobExecutionError>`](crate::error::DescribeJobExecutionError)
    pub fn describe_job_execution(&self) -> fluent_builders::DescribeJobExecution {
        fluent_builders::DescribeJobExecution::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`GetPendingJobExecutions`](crate::client::fluent_builders::GetPendingJobExecutions) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`thing_name(impl Into<String>)`](crate::client::fluent_builders::GetPendingJobExecutions::thing_name) / [`set_thing_name(Option<String>)`](crate::client::fluent_builders::GetPendingJobExecutions::set_thing_name): <p>The name of the thing that is executing the job.</p>
    /// - On success, responds with [`GetPendingJobExecutionsOutput`](crate::output::GetPendingJobExecutionsOutput) with field(s):
    ///   - [`in_progress_jobs(Option<Vec<JobExecutionSummary>>)`](crate::output::GetPendingJobExecutionsOutput::in_progress_jobs): <p>A list of JobExecutionSummary objects with status IN_PROGRESS.</p>
    ///   - [`queued_jobs(Option<Vec<JobExecutionSummary>>)`](crate::output::GetPendingJobExecutionsOutput::queued_jobs): <p>A list of JobExecutionSummary objects with status QUEUED.</p>
    /// - On failure, responds with [`SdkError<GetPendingJobExecutionsError>`](crate::error::GetPendingJobExecutionsError)
    pub fn get_pending_job_executions(&self) -> fluent_builders::GetPendingJobExecutions {
        fluent_builders::GetPendingJobExecutions::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`StartNextPendingJobExecution`](crate::client::fluent_builders::StartNextPendingJobExecution) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`thing_name(impl Into<String>)`](crate::client::fluent_builders::StartNextPendingJobExecution::thing_name) / [`set_thing_name(Option<String>)`](crate::client::fluent_builders::StartNextPendingJobExecution::set_thing_name): <p>The name of the thing associated with the device.</p>
    ///   - [`status_details(HashMap<String, String>)`](crate::client::fluent_builders::StartNextPendingJobExecution::status_details) / [`set_status_details(Option<HashMap<String, String>>)`](crate::client::fluent_builders::StartNextPendingJobExecution::set_status_details): <p>A collection of name/value pairs that describe the status of the job execution. If not specified, the statusDetails are unchanged.</p>
    ///   - [`step_timeout_in_minutes(i64)`](crate::client::fluent_builders::StartNextPendingJobExecution::step_timeout_in_minutes) / [`set_step_timeout_in_minutes(Option<i64>)`](crate::client::fluent_builders::StartNextPendingJobExecution::set_step_timeout_in_minutes): <p>Specifies the amount of time this device has to finish execution of this job. If the job execution status is not set to a terminal state before this timer expires, or before the timer is reset (by calling <code>UpdateJobExecution</code>, setting the status to <code>IN_PROGRESS</code> and specifying a new timeout value in field <code>stepTimeoutInMinutes</code>) the job execution status will be automatically set to <code>TIMED_OUT</code>. Note that setting this timeout has no effect on that job execution timeout which may have been specified when the job was created (<code>CreateJob</code> using field <code>timeoutConfig</code>).</p>
    /// - On success, responds with [`StartNextPendingJobExecutionOutput`](crate::output::StartNextPendingJobExecutionOutput) with field(s):
    ///   - [`execution(Option<JobExecution>)`](crate::output::StartNextPendingJobExecutionOutput::execution): <p>A JobExecution object.</p>
    /// - On failure, responds with [`SdkError<StartNextPendingJobExecutionError>`](crate::error::StartNextPendingJobExecutionError)
    pub fn start_next_pending_job_execution(
        &self,
    ) -> fluent_builders::StartNextPendingJobExecution {
        fluent_builders::StartNextPendingJobExecution::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`UpdateJobExecution`](crate::client::fluent_builders::UpdateJobExecution) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`job_id(impl Into<String>)`](crate::client::fluent_builders::UpdateJobExecution::job_id) / [`set_job_id(Option<String>)`](crate::client::fluent_builders::UpdateJobExecution::set_job_id): <p>The unique identifier assigned to this job when it was created.</p>
    ///   - [`thing_name(impl Into<String>)`](crate::client::fluent_builders::UpdateJobExecution::thing_name) / [`set_thing_name(Option<String>)`](crate::client::fluent_builders::UpdateJobExecution::set_thing_name): <p>The name of the thing associated with the device.</p>
    ///   - [`status(JobExecutionStatus)`](crate::client::fluent_builders::UpdateJobExecution::status) / [`set_status(Option<JobExecutionStatus>)`](crate::client::fluent_builders::UpdateJobExecution::set_status): <p>The new status for the job execution (IN_PROGRESS, FAILED, SUCCESS, or REJECTED). This must be specified on every update.</p>
    ///   - [`status_details(HashMap<String, String>)`](crate::client::fluent_builders::UpdateJobExecution::status_details) / [`set_status_details(Option<HashMap<String, String>>)`](crate::client::fluent_builders::UpdateJobExecution::set_status_details): <p> Optional. A collection of name/value pairs that describe the status of the job execution. If not specified, the statusDetails are unchanged.</p>
    ///   - [`step_timeout_in_minutes(i64)`](crate::client::fluent_builders::UpdateJobExecution::step_timeout_in_minutes) / [`set_step_timeout_in_minutes(Option<i64>)`](crate::client::fluent_builders::UpdateJobExecution::set_step_timeout_in_minutes): <p>Specifies the amount of time this device has to finish execution of this job. If the job execution status is not set to a terminal state before this timer expires, or before the timer is reset (by again calling <code>UpdateJobExecution</code>, setting the status to <code>IN_PROGRESS</code> and specifying a new timeout value in this field) the job execution status will be automatically set to <code>TIMED_OUT</code>. Note that setting or resetting this timeout has no effect on that job execution timeout which may have been specified when the job was created (<code>CreateJob</code> using field <code>timeoutConfig</code>).</p>
    ///   - [`expected_version(i64)`](crate::client::fluent_builders::UpdateJobExecution::expected_version) / [`set_expected_version(Option<i64>)`](crate::client::fluent_builders::UpdateJobExecution::set_expected_version): <p>Optional. The expected current version of the job execution. Each time you update the job execution, its version is incremented. If the version of the job execution stored in Jobs does not match, the update is rejected with a VersionMismatch error, and an ErrorResponse that contains the current job execution status data is returned. (This makes it unnecessary to perform a separate DescribeJobExecution request in order to obtain the job execution status data.)</p>
    ///   - [`include_job_execution_state(bool)`](crate::client::fluent_builders::UpdateJobExecution::include_job_execution_state) / [`set_include_job_execution_state(Option<bool>)`](crate::client::fluent_builders::UpdateJobExecution::set_include_job_execution_state): <p>Optional. When included and set to true, the response contains the JobExecutionState data. The default is false.</p>
    ///   - [`include_job_document(bool)`](crate::client::fluent_builders::UpdateJobExecution::include_job_document) / [`set_include_job_document(Option<bool>)`](crate::client::fluent_builders::UpdateJobExecution::set_include_job_document): <p>Optional. When set to true, the response contains the job document. The default is false.</p>
    ///   - [`execution_number(i64)`](crate::client::fluent_builders::UpdateJobExecution::execution_number) / [`set_execution_number(Option<i64>)`](crate::client::fluent_builders::UpdateJobExecution::set_execution_number): <p>Optional. A number that identifies a particular job execution on a particular device.</p>
    /// - On success, responds with [`UpdateJobExecutionOutput`](crate::output::UpdateJobExecutionOutput) with field(s):
    ///   - [`execution_state(Option<JobExecutionState>)`](crate::output::UpdateJobExecutionOutput::execution_state): <p>A JobExecutionState object.</p>
    ///   - [`job_document(Option<String>)`](crate::output::UpdateJobExecutionOutput::job_document): <p>The contents of the Job Documents.</p>
    /// - On failure, responds with [`SdkError<UpdateJobExecutionError>`](crate::error::UpdateJobExecutionError)
    pub fn update_job_execution(&self) -> fluent_builders::UpdateJobExecution {
        fluent_builders::UpdateJobExecution::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `DescribeJobExecution`.
    ///
    /// <p>Gets details of a job execution.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DescribeJobExecution {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::describe_job_execution_input::Builder,
    }
    impl DescribeJobExecution {
        /// Creates a new `DescribeJobExecution`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
            crate::output::DescribeJobExecutionOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeJobExecutionError>,
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
        /// <p>The unique identifier assigned to this job when it was created.</p>
        pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.job_id(input.into());
            self
        }
        /// <p>The unique identifier assigned to this job when it was created.</p>
        pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_job_id(input);
            self
        }
        /// <p>The thing name associated with the device the job execution is running on.</p>
        pub fn thing_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.thing_name(input.into());
            self
        }
        /// <p>The thing name associated with the device the job execution is running on.</p>
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_thing_name(input);
            self
        }
        /// <p>Optional. When set to true, the response contains the job document. The default is false.</p>
        pub fn include_job_document(mut self, input: bool) -> Self {
            self.inner = self.inner.include_job_document(input);
            self
        }
        /// <p>Optional. When set to true, the response contains the job document. The default is false.</p>
        pub fn set_include_job_document(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_include_job_document(input);
            self
        }
        /// <p>Optional. A number that identifies a particular job execution on a particular device. If not specified, the latest job execution is returned.</p>
        pub fn execution_number(mut self, input: i64) -> Self {
            self.inner = self.inner.execution_number(input);
            self
        }
        /// <p>Optional. A number that identifies a particular job execution on a particular device. If not specified, the latest job execution is returned.</p>
        pub fn set_execution_number(mut self, input: std::option::Option<i64>) -> Self {
            self.inner = self.inner.set_execution_number(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetPendingJobExecutions`.
    ///
    /// <p>Gets the list of all jobs for a thing that are not in a terminal status.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetPendingJobExecutions {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::get_pending_job_executions_input::Builder,
    }
    impl GetPendingJobExecutions {
        /// Creates a new `GetPendingJobExecutions`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
            crate::output::GetPendingJobExecutionsOutput,
            aws_smithy_http::result::SdkError<crate::error::GetPendingJobExecutionsError>,
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
        /// <p>The name of the thing that is executing the job.</p>
        pub fn thing_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.thing_name(input.into());
            self
        }
        /// <p>The name of the thing that is executing the job.</p>
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_thing_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `StartNextPendingJobExecution`.
    ///
    /// <p>Gets and starts the next pending (status IN_PROGRESS or QUEUED) job execution for a thing.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct StartNextPendingJobExecution {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::start_next_pending_job_execution_input::Builder,
    }
    impl StartNextPendingJobExecution {
        /// Creates a new `StartNextPendingJobExecution`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
            crate::output::StartNextPendingJobExecutionOutput,
            aws_smithy_http::result::SdkError<crate::error::StartNextPendingJobExecutionError>,
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
        /// <p>The name of the thing associated with the device.</p>
        pub fn thing_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.thing_name(input.into());
            self
        }
        /// <p>The name of the thing associated with the device.</p>
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_thing_name(input);
            self
        }
        /// Adds a key-value pair to `statusDetails`.
        ///
        /// To override the contents of this collection use [`set_status_details`](Self::set_status_details).
        ///
        /// <p>A collection of name/value pairs that describe the status of the job execution. If not specified, the statusDetails are unchanged.</p>
        pub fn status_details(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.status_details(k.into(), v.into());
            self
        }
        /// <p>A collection of name/value pairs that describe the status of the job execution. If not specified, the statusDetails are unchanged.</p>
        pub fn set_status_details(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_status_details(input);
            self
        }
        /// <p>Specifies the amount of time this device has to finish execution of this job. If the job execution status is not set to a terminal state before this timer expires, or before the timer is reset (by calling <code>UpdateJobExecution</code>, setting the status to <code>IN_PROGRESS</code> and specifying a new timeout value in field <code>stepTimeoutInMinutes</code>) the job execution status will be automatically set to <code>TIMED_OUT</code>. Note that setting this timeout has no effect on that job execution timeout which may have been specified when the job was created (<code>CreateJob</code> using field <code>timeoutConfig</code>).</p>
        pub fn step_timeout_in_minutes(mut self, input: i64) -> Self {
            self.inner = self.inner.step_timeout_in_minutes(input);
            self
        }
        /// <p>Specifies the amount of time this device has to finish execution of this job. If the job execution status is not set to a terminal state before this timer expires, or before the timer is reset (by calling <code>UpdateJobExecution</code>, setting the status to <code>IN_PROGRESS</code> and specifying a new timeout value in field <code>stepTimeoutInMinutes</code>) the job execution status will be automatically set to <code>TIMED_OUT</code>. Note that setting this timeout has no effect on that job execution timeout which may have been specified when the job was created (<code>CreateJob</code> using field <code>timeoutConfig</code>).</p>
        pub fn set_step_timeout_in_minutes(mut self, input: std::option::Option<i64>) -> Self {
            self.inner = self.inner.set_step_timeout_in_minutes(input);
            self
        }
    }
    /// Fluent builder constructing a request to `UpdateJobExecution`.
    ///
    /// <p>Updates the status of a job execution.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct UpdateJobExecution {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::update_job_execution_input::Builder,
    }
    impl UpdateJobExecution {
        /// Creates a new `UpdateJobExecution`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
            crate::output::UpdateJobExecutionOutput,
            aws_smithy_http::result::SdkError<crate::error::UpdateJobExecutionError>,
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
        /// <p>The unique identifier assigned to this job when it was created.</p>
        pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.job_id(input.into());
            self
        }
        /// <p>The unique identifier assigned to this job when it was created.</p>
        pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_job_id(input);
            self
        }
        /// <p>The name of the thing associated with the device.</p>
        pub fn thing_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.thing_name(input.into());
            self
        }
        /// <p>The name of the thing associated with the device.</p>
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_thing_name(input);
            self
        }
        /// <p>The new status for the job execution (IN_PROGRESS, FAILED, SUCCESS, or REJECTED). This must be specified on every update.</p>
        pub fn status(mut self, input: crate::model::JobExecutionStatus) -> Self {
            self.inner = self.inner.status(input);
            self
        }
        /// <p>The new status for the job execution (IN_PROGRESS, FAILED, SUCCESS, or REJECTED). This must be specified on every update.</p>
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::JobExecutionStatus>,
        ) -> Self {
            self.inner = self.inner.set_status(input);
            self
        }
        /// Adds a key-value pair to `statusDetails`.
        ///
        /// To override the contents of this collection use [`set_status_details`](Self::set_status_details).
        ///
        /// <p> Optional. A collection of name/value pairs that describe the status of the job execution. If not specified, the statusDetails are unchanged.</p>
        pub fn status_details(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.status_details(k.into(), v.into());
            self
        }
        /// <p> Optional. A collection of name/value pairs that describe the status of the job execution. If not specified, the statusDetails are unchanged.</p>
        pub fn set_status_details(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_status_details(input);
            self
        }
        /// <p>Specifies the amount of time this device has to finish execution of this job. If the job execution status is not set to a terminal state before this timer expires, or before the timer is reset (by again calling <code>UpdateJobExecution</code>, setting the status to <code>IN_PROGRESS</code> and specifying a new timeout value in this field) the job execution status will be automatically set to <code>TIMED_OUT</code>. Note that setting or resetting this timeout has no effect on that job execution timeout which may have been specified when the job was created (<code>CreateJob</code> using field <code>timeoutConfig</code>).</p>
        pub fn step_timeout_in_minutes(mut self, input: i64) -> Self {
            self.inner = self.inner.step_timeout_in_minutes(input);
            self
        }
        /// <p>Specifies the amount of time this device has to finish execution of this job. If the job execution status is not set to a terminal state before this timer expires, or before the timer is reset (by again calling <code>UpdateJobExecution</code>, setting the status to <code>IN_PROGRESS</code> and specifying a new timeout value in this field) the job execution status will be automatically set to <code>TIMED_OUT</code>. Note that setting or resetting this timeout has no effect on that job execution timeout which may have been specified when the job was created (<code>CreateJob</code> using field <code>timeoutConfig</code>).</p>
        pub fn set_step_timeout_in_minutes(mut self, input: std::option::Option<i64>) -> Self {
            self.inner = self.inner.set_step_timeout_in_minutes(input);
            self
        }
        /// <p>Optional. The expected current version of the job execution. Each time you update the job execution, its version is incremented. If the version of the job execution stored in Jobs does not match, the update is rejected with a VersionMismatch error, and an ErrorResponse that contains the current job execution status data is returned. (This makes it unnecessary to perform a separate DescribeJobExecution request in order to obtain the job execution status data.)</p>
        pub fn expected_version(mut self, input: i64) -> Self {
            self.inner = self.inner.expected_version(input);
            self
        }
        /// <p>Optional. The expected current version of the job execution. Each time you update the job execution, its version is incremented. If the version of the job execution stored in Jobs does not match, the update is rejected with a VersionMismatch error, and an ErrorResponse that contains the current job execution status data is returned. (This makes it unnecessary to perform a separate DescribeJobExecution request in order to obtain the job execution status data.)</p>
        pub fn set_expected_version(mut self, input: std::option::Option<i64>) -> Self {
            self.inner = self.inner.set_expected_version(input);
            self
        }
        /// <p>Optional. When included and set to true, the response contains the JobExecutionState data. The default is false.</p>
        pub fn include_job_execution_state(mut self, input: bool) -> Self {
            self.inner = self.inner.include_job_execution_state(input);
            self
        }
        /// <p>Optional. When included and set to true, the response contains the JobExecutionState data. The default is false.</p>
        pub fn set_include_job_execution_state(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_include_job_execution_state(input);
            self
        }
        /// <p>Optional. When set to true, the response contains the job document. The default is false.</p>
        pub fn include_job_document(mut self, input: bool) -> Self {
            self.inner = self.inner.include_job_document(input);
            self
        }
        /// <p>Optional. When set to true, the response contains the job document. The default is false.</p>
        pub fn set_include_job_document(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_include_job_document(input);
            self
        }
        /// <p>Optional. A number that identifies a particular job execution on a particular device.</p>
        pub fn execution_number(mut self, input: i64) -> Self {
            self.inner = self.inner.execution_number(input);
            self
        }
        /// <p>Optional. A number that identifies a particular job execution on a particular device.</p>
        pub fn set_execution_number(mut self, input: std::option::Option<i64>) -> Self {
            self.inner = self.inner.set_execution_number(input);
            self
        }
    }
}

impl Client {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn<C, E>(conf: crate::Config, conn: C) -> Self
    where
        C: aws_smithy_client::bounds::SmithyConnector<Error = E> + Send + 'static,
        E: Into<aws_smithy_http::result::ConnectorError>,
    {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::new()
            .connector(aws_smithy_client::erase::DynConnector::new(conn))
            .middleware(aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ));
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::dyn_https().middleware(
            aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ),
        );
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        // the builder maintains a try-state. To avoid suppressing the warning when sleep is unset,
        // only set it if we actually have a sleep impl.
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
