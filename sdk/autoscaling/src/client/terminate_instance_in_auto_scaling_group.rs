// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TerminateInstanceInAutoScalingGroup`](crate::operation::terminate_instance_in_auto_scaling_group::builders::TerminateInstanceInAutoScalingGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::terminate_instance_in_auto_scaling_group::builders::TerminateInstanceInAutoScalingGroupFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::terminate_instance_in_auto_scaling_group::builders::TerminateInstanceInAutoScalingGroupFluentBuilder::set_instance_id): <p>The ID of the instance.</p>
    ///   - [`should_decrement_desired_capacity(bool)`](crate::operation::terminate_instance_in_auto_scaling_group::builders::TerminateInstanceInAutoScalingGroupFluentBuilder::should_decrement_desired_capacity) / [`set_should_decrement_desired_capacity(Option<bool>)`](crate::operation::terminate_instance_in_auto_scaling_group::builders::TerminateInstanceInAutoScalingGroupFluentBuilder::set_should_decrement_desired_capacity): <p>Indicates whether terminating the instance also decrements the size of the Auto Scaling group.</p>
    /// - On success, responds with [`TerminateInstanceInAutoScalingGroupOutput`](crate::operation::terminate_instance_in_auto_scaling_group::TerminateInstanceInAutoScalingGroupOutput) with field(s):
    ///   - [`activity(Option<Activity>)`](crate::operation::terminate_instance_in_auto_scaling_group::TerminateInstanceInAutoScalingGroupOutput::activity): <p>A scaling activity.</p>
    /// - On failure, responds with [`SdkError<TerminateInstanceInAutoScalingGroupError>`](crate::operation::terminate_instance_in_auto_scaling_group::TerminateInstanceInAutoScalingGroupError)
    pub fn terminate_instance_in_auto_scaling_group(&self) -> crate::operation::terminate_instance_in_auto_scaling_group::builders::TerminateInstanceInAutoScalingGroupFluentBuilder{
        crate::operation::terminate_instance_in_auto_scaling_group::builders::TerminateInstanceInAutoScalingGroupFluentBuilder::new(self.handle.clone())
    }
}
