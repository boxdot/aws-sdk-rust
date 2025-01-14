// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_deployment::_create_deployment_output::CreateDeploymentOutputBuilder;

pub use crate::operation::create_deployment::_create_deployment_input::CreateDeploymentInputBuilder;

/// Fluent builder constructing a request to `CreateDeployment`.
///
/// <p>Runs deployment or stack commands. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-deploying.html">Deploying Apps</a> and <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-commands.html">Run Stack Commands</a>.</p>
/// <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Deploy or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDeploymentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_deployment::builders::CreateDeploymentInputBuilder,
}
impl CreateDeploymentFluentBuilder {
    /// Creates a new `CreateDeployment`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_deployment::CreateDeployment,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_deployment::CreateDeploymentError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_deployment::CreateDeploymentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_deployment::CreateDeploymentError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
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
    ) -> ::std::result::Result<
        crate::operation::create_deployment::CreateDeploymentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_deployment::CreateDeploymentError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_deployment::CreateDeployment,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_deployment::CreateDeploymentError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The stack ID.</p>
    pub fn stack_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stack_id(input.into());
        self
    }
    /// <p>The stack ID.</p>
    pub fn set_stack_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stack_id(input);
        self
    }
    /// <p>The app ID. This parameter is required for app deployments, but not for other deployment commands.</p>
    pub fn app_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_id(input.into());
        self
    }
    /// <p>The app ID. This parameter is required for app deployments, but not for other deployment commands.</p>
    pub fn set_app_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_id(input);
        self
    }
    /// Appends an item to `InstanceIds`.
    ///
    /// To override the contents of this collection use [`set_instance_ids`](Self::set_instance_ids).
    ///
    /// <p>The instance IDs for the deployment targets.</p>
    pub fn instance_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_ids(input.into());
        self
    }
    /// <p>The instance IDs for the deployment targets.</p>
    pub fn set_instance_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_instance_ids(input);
        self
    }
    /// Appends an item to `LayerIds`.
    ///
    /// To override the contents of this collection use [`set_layer_ids`](Self::set_layer_ids).
    ///
    /// <p>The layer IDs for the deployment targets.</p>
    pub fn layer_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.layer_ids(input.into());
        self
    }
    /// <p>The layer IDs for the deployment targets.</p>
    pub fn set_layer_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_layer_ids(input);
        self
    }
    /// <p>A <code>DeploymentCommand</code> object that specifies the deployment command and any associated arguments.</p>
    pub fn command(mut self, input: crate::types::DeploymentCommand) -> Self {
        self.inner = self.inner.command(input);
        self
    }
    /// <p>A <code>DeploymentCommand</code> object that specifies the deployment command and any associated arguments.</p>
    pub fn set_command(
        mut self,
        input: ::std::option::Option<crate::types::DeploymentCommand>,
    ) -> Self {
        self.inner = self.inner.set_command(input);
        self
    }
    /// <p>A user-defined comment.</p>
    pub fn comment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.comment(input.into());
        self
    }
    /// <p>A user-defined comment.</p>
    pub fn set_comment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_comment(input);
        self
    }
    /// <p>A string that contains user-defined, custom JSON. You can use this parameter to override some corresponding default stack configuration JSON values. The string should be in the following format:</p>
    /// <p> <code>"{\"key1\": \"value1\", \"key2\": \"value2\",...}"</code> </p>
    /// <p>For more information about custom JSON, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html">Use Custom JSON to Modify the Stack Configuration Attributes</a> and <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook-json-override.html">Overriding Attributes With Custom JSON</a>.</p>
    pub fn custom_json(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.custom_json(input.into());
        self
    }
    /// <p>A string that contains user-defined, custom JSON. You can use this parameter to override some corresponding default stack configuration JSON values. The string should be in the following format:</p>
    /// <p> <code>"{\"key1\": \"value1\", \"key2\": \"value2\",...}"</code> </p>
    /// <p>For more information about custom JSON, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html">Use Custom JSON to Modify the Stack Configuration Attributes</a> and <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook-json-override.html">Overriding Attributes With Custom JSON</a>.</p>
    pub fn set_custom_json(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_custom_json(input);
        self
    }
}
