// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_provisioned_product_plan::_delete_provisioned_product_plan_output::DeleteProvisionedProductPlanOutputBuilder;

pub use crate::operation::delete_provisioned_product_plan::_delete_provisioned_product_plan_input::DeleteProvisionedProductPlanInputBuilder;

/// Fluent builder constructing a request to `DeleteProvisionedProductPlan`.
///
/// <p>Deletes the specified plan.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteProvisionedProductPlanFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::delete_provisioned_product_plan::builders::DeleteProvisionedProductPlanInputBuilder,
}
impl DeleteProvisionedProductPlanFluentBuilder {
    /// Creates a new `DeleteProvisionedProductPlan`.
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
            crate::operation::delete_provisioned_product_plan::DeleteProvisionedProductPlan,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_provisioned_product_plan::DeleteProvisionedProductPlanError,
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
        crate::operation::delete_provisioned_product_plan::DeleteProvisionedProductPlanOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_provisioned_product_plan::DeleteProvisionedProductPlanError,
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
        crate::operation::delete_provisioned_product_plan::DeleteProvisionedProductPlanOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_provisioned_product_plan::DeleteProvisionedProductPlanError,
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
            crate::operation::delete_provisioned_product_plan::DeleteProvisionedProductPlan,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_provisioned_product_plan::DeleteProvisionedProductPlanError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn accept_language(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.accept_language(input.into());
        self
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn set_accept_language(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_accept_language(input);
        self
    }
    /// <p>The plan identifier.</p>
    pub fn plan_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.plan_id(input.into());
        self
    }
    /// <p>The plan identifier.</p>
    pub fn set_plan_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_plan_id(input);
        self
    }
    /// <p>If set to true, Service Catalog stops managing the specified provisioned product even if it cannot delete the underlying resources.</p>
    pub fn ignore_errors(mut self, input: bool) -> Self {
        self.inner = self.inner.ignore_errors(input);
        self
    }
    /// <p>If set to true, Service Catalog stops managing the specified provisioned product even if it cannot delete the underlying resources.</p>
    pub fn set_ignore_errors(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_ignore_errors(input);
        self
    }
}
