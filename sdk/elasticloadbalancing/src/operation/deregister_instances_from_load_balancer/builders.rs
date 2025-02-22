// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deregister_instances_from_load_balancer::_deregister_instances_from_load_balancer_output::DeregisterInstancesFromLoadBalancerOutputBuilder;

pub use crate::operation::deregister_instances_from_load_balancer::_deregister_instances_from_load_balancer_input::DeregisterInstancesFromLoadBalancerInputBuilder;

/// Fluent builder constructing a request to `DeregisterInstancesFromLoadBalancer`.
///
/// <p>Deregisters the specified instances from the specified load balancer. After the instance is deregistered, it no longer receives traffic from the load balancer.</p>
/// <p>You can use <code>DescribeLoadBalancers</code> to verify that the instance is deregistered from the load balancer.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-deregister-register-instances.html">Register or De-Register EC2 Instances</a> in the <i>Classic Load Balancers Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeregisterInstancesFromLoadBalancerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::deregister_instances_from_load_balancer::builders::DeregisterInstancesFromLoadBalancerInputBuilder,
}
impl DeregisterInstancesFromLoadBalancerFluentBuilder {
    /// Creates a new `DeregisterInstancesFromLoadBalancer`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::deregister_instances_from_load_balancer::DeregisterInstancesFromLoadBalancer, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::deregister_instances_from_load_balancer::DeregisterInstancesFromLoadBalancerError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::deregister_instances_from_load_balancer::DeregisterInstancesFromLoadBalancerOutput, ::aws_smithy_http::result::SdkError<crate::operation::deregister_instances_from_load_balancer::DeregisterInstancesFromLoadBalancerError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::deregister_instances_from_load_balancer::DeregisterInstancesFromLoadBalancerOutput, ::aws_smithy_http::result::SdkError<crate::operation::deregister_instances_from_load_balancer::DeregisterInstancesFromLoadBalancerError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::deregister_instances_from_load_balancer::DeregisterInstancesFromLoadBalancer, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::deregister_instances_from_load_balancer::DeregisterInstancesFromLoadBalancerError>
    >{
        self.customize_middleware().await
    }
    /// <p>The name of the load balancer.</p>
    pub fn load_balancer_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.load_balancer_name(input.into());
        self
    }
    /// <p>The name of the load balancer.</p>
    pub fn set_load_balancer_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_load_balancer_name(input);
        self
    }
    /// Appends an item to `Instances`.
    ///
    /// To override the contents of this collection use [`set_instances`](Self::set_instances).
    ///
    /// <p>The IDs of the instances.</p>
    pub fn instances(mut self, input: crate::types::Instance) -> Self {
        self.inner = self.inner.instances(input);
        self
    }
    /// <p>The IDs of the instances.</p>
    pub fn set_instances(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Instance>>,
    ) -> Self {
        self.inner = self.inner.set_instances(input);
        self
    }
}
