// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteLoadBalancerPolicy`](crate::operation::delete_load_balancer_policy::builders::DeleteLoadBalancerPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`load_balancer_name(impl ::std::convert::Into<String>)`](crate::operation::delete_load_balancer_policy::builders::DeleteLoadBalancerPolicyFluentBuilder::load_balancer_name) / [`set_load_balancer_name(Option<String>)`](crate::operation::delete_load_balancer_policy::builders::DeleteLoadBalancerPolicyFluentBuilder::set_load_balancer_name): <p>The name of the load balancer.</p>
    ///   - [`policy_name(impl ::std::convert::Into<String>)`](crate::operation::delete_load_balancer_policy::builders::DeleteLoadBalancerPolicyFluentBuilder::policy_name) / [`set_policy_name(Option<String>)`](crate::operation::delete_load_balancer_policy::builders::DeleteLoadBalancerPolicyFluentBuilder::set_policy_name): <p>The name of the policy.</p>
    /// - On success, responds with [`DeleteLoadBalancerPolicyOutput`](crate::operation::delete_load_balancer_policy::DeleteLoadBalancerPolicyOutput)
    /// - On failure, responds with [`SdkError<DeleteLoadBalancerPolicyError>`](crate::operation::delete_load_balancer_policy::DeleteLoadBalancerPolicyError)
    pub fn delete_load_balancer_policy(&self) -> crate::operation::delete_load_balancer_policy::builders::DeleteLoadBalancerPolicyFluentBuilder{
        crate::operation::delete_load_balancer_policy::builders::DeleteLoadBalancerPolicyFluentBuilder::new(self.handle.clone())
    }
}
