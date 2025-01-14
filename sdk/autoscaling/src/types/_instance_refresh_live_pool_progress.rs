// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Reports progress on replacing instances that are in the Auto Scaling group.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InstanceRefreshLivePoolProgress {
    /// <p>The percentage of instances in the Auto Scaling group that have been replaced. For each instance replacement, Amazon EC2 Auto Scaling tracks the instance's health status and warm-up time. When the instance's health status changes to healthy and the specified warm-up time passes, the instance is considered updated and is added to the percentage complete.</p>
    #[doc(hidden)]
    pub percentage_complete: ::std::option::Option<i32>,
    /// <p>The number of instances remaining to update.</p>
    #[doc(hidden)]
    pub instances_to_update: ::std::option::Option<i32>,
}
impl InstanceRefreshLivePoolProgress {
    /// <p>The percentage of instances in the Auto Scaling group that have been replaced. For each instance replacement, Amazon EC2 Auto Scaling tracks the instance's health status and warm-up time. When the instance's health status changes to healthy and the specified warm-up time passes, the instance is considered updated and is added to the percentage complete.</p>
    pub fn percentage_complete(&self) -> ::std::option::Option<i32> {
        self.percentage_complete
    }
    /// <p>The number of instances remaining to update.</p>
    pub fn instances_to_update(&self) -> ::std::option::Option<i32> {
        self.instances_to_update
    }
}
impl InstanceRefreshLivePoolProgress {
    /// Creates a new builder-style object to manufacture [`InstanceRefreshLivePoolProgress`](crate::types::InstanceRefreshLivePoolProgress).
    pub fn builder() -> crate::types::builders::InstanceRefreshLivePoolProgressBuilder {
        crate::types::builders::InstanceRefreshLivePoolProgressBuilder::default()
    }
}

/// A builder for [`InstanceRefreshLivePoolProgress`](crate::types::InstanceRefreshLivePoolProgress).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InstanceRefreshLivePoolProgressBuilder {
    pub(crate) percentage_complete: ::std::option::Option<i32>,
    pub(crate) instances_to_update: ::std::option::Option<i32>,
}
impl InstanceRefreshLivePoolProgressBuilder {
    /// <p>The percentage of instances in the Auto Scaling group that have been replaced. For each instance replacement, Amazon EC2 Auto Scaling tracks the instance's health status and warm-up time. When the instance's health status changes to healthy and the specified warm-up time passes, the instance is considered updated and is added to the percentage complete.</p>
    pub fn percentage_complete(mut self, input: i32) -> Self {
        self.percentage_complete = ::std::option::Option::Some(input);
        self
    }
    /// <p>The percentage of instances in the Auto Scaling group that have been replaced. For each instance replacement, Amazon EC2 Auto Scaling tracks the instance's health status and warm-up time. When the instance's health status changes to healthy and the specified warm-up time passes, the instance is considered updated and is added to the percentage complete.</p>
    pub fn set_percentage_complete(mut self, input: ::std::option::Option<i32>) -> Self {
        self.percentage_complete = input;
        self
    }
    /// <p>The number of instances remaining to update.</p>
    pub fn instances_to_update(mut self, input: i32) -> Self {
        self.instances_to_update = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of instances remaining to update.</p>
    pub fn set_instances_to_update(mut self, input: ::std::option::Option<i32>) -> Self {
        self.instances_to_update = input;
        self
    }
    /// Consumes the builder and constructs a [`InstanceRefreshLivePoolProgress`](crate::types::InstanceRefreshLivePoolProgress).
    pub fn build(self) -> crate::types::InstanceRefreshLivePoolProgress {
        crate::types::InstanceRefreshLivePoolProgress {
            percentage_complete: self.percentage_complete,
            instances_to_update: self.instances_to_update,
        }
    }
}
