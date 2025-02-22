// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_permission::_create_permission_output::CreatePermissionOutputBuilder;

pub use crate::operation::create_permission::_create_permission_input::CreatePermissionInputBuilder;

/// Fluent builder constructing a request to `CreatePermission`.
///
/// <p>Creates a customer managed permission for a specified resource type that you can attach to resource shares. It is created in the Amazon Web Services Region in which you call the operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreatePermissionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_permission::builders::CreatePermissionInputBuilder,
}
impl CreatePermissionFluentBuilder {
    /// Creates a new `CreatePermission`.
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
            crate::operation::create_permission::CreatePermission,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_permission::CreatePermissionError,
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
        crate::operation::create_permission::CreatePermissionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_permission::CreatePermissionError,
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
        crate::operation::create_permission::CreatePermissionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_permission::CreatePermissionError,
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
            crate::operation::create_permission::CreatePermission,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_permission::CreatePermissionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Specifies the name of the customer managed permission. The name must be unique within the Amazon Web Services Region.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>Specifies the name of the customer managed permission. The name must be unique within the Amazon Web Services Region.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>Specifies the name of the resource type that this customer managed permission applies to.</p>
    /// <p>The format is <code> <i>
    /// <service-code></service-code></i>:<i>
    /// <resource-type></resource-type></i> </code> and is not case sensitive. For example, to specify an Amazon EC2 Subnet, you can use the string <code>ec2:subnet</code>. To see the list of valid values for this parameter, query the <code>ListResourceTypes</code> operation.</p>
    pub fn resource_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.resource_type(input.into());
        self
    }
    /// <p>Specifies the name of the resource type that this customer managed permission applies to.</p>
    /// <p>The format is <code> <i>
    /// <service-code></service-code></i>:<i>
    /// <resource-type></resource-type></i> </code> and is not case sensitive. For example, to specify an Amazon EC2 Subnet, you can use the string <code>ec2:subnet</code>. To see the list of valid values for this parameter, query the <code>ListResourceTypes</code> operation.</p>
    pub fn set_resource_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// <p>A string in JSON format string that contains the following elements of a resource-based policy:</p>
    /// <ul>
    /// <li> <p> <b>Effect</b>: must be set to <code>ALLOW</code>.</p> </li>
    /// <li> <p> <b>Action</b>: specifies the actions that are allowed by this customer managed permission. The list must contain only actions that are supported by the specified resource type. For a list of all actions supported by each resource type, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the <i>Identity and Access Management User Guide</i>.</p> </li>
    /// <li> <p> <b>Condition</b>: (optional) specifies conditional parameters that must evaluate to true when a user attempts an action for that action to be allowed. For more information about the Condition element, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition.html">IAM policies: Condition element</a> in the <i>Identity and Access Management User Guide</i>.</p> </li>
    /// </ul>
    /// <p>This template can't include either the <code>Resource</code> or <code>Principal</code> elements. Those are both filled in by RAM when it instantiates the resource-based policy on each resource shared using this managed permission. The <code>Resource</code> comes from the ARN of the specific resource that you are sharing. The <code>Principal</code> comes from the list of identities added to the resource share.</p>
    pub fn policy_template(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.policy_template(input.into());
        self
    }
    /// <p>A string in JSON format string that contains the following elements of a resource-based policy:</p>
    /// <ul>
    /// <li> <p> <b>Effect</b>: must be set to <code>ALLOW</code>.</p> </li>
    /// <li> <p> <b>Action</b>: specifies the actions that are allowed by this customer managed permission. The list must contain only actions that are supported by the specified resource type. For a list of all actions supported by each resource type, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the <i>Identity and Access Management User Guide</i>.</p> </li>
    /// <li> <p> <b>Condition</b>: (optional) specifies conditional parameters that must evaluate to true when a user attempts an action for that action to be allowed. For more information about the Condition element, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition.html">IAM policies: Condition element</a> in the <i>Identity and Access Management User Guide</i>.</p> </li>
    /// </ul>
    /// <p>This template can't include either the <code>Resource</code> or <code>Principal</code> elements. Those are both filled in by RAM when it instantiates the resource-based policy on each resource shared using this managed permission. The <code>Resource</code> comes from the ARN of the specific resource that you are sharing. The <code>Principal</code> comes from the list of identities added to the resource share.</p>
    pub fn set_policy_template(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_policy_template(input);
        self
    }
    /// <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value.</a>.</p>
    /// <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>
    /// <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value.</a>.</p>
    /// <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>
    /// <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Specifies a list of one or more tag key and value pairs to attach to the permission.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Specifies a list of one or more tag key and value pairs to attach to the permission.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
