// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the capacity status for a fleet.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ComputeCapacityStatus {
    /// <p>The desired number of streaming instances.</p>
    #[doc(hidden)]
    pub desired: ::std::option::Option<i32>,
    /// <p>The total number of simultaneous streaming instances that are running.</p>
    #[doc(hidden)]
    pub running: ::std::option::Option<i32>,
    /// <p>The number of instances in use for streaming.</p>
    #[doc(hidden)]
    pub in_use: ::std::option::Option<i32>,
    /// <p>The number of currently available instances that can be used to stream sessions.</p>
    #[doc(hidden)]
    pub available: ::std::option::Option<i32>,
}
impl ComputeCapacityStatus {
    /// <p>The desired number of streaming instances.</p>
    pub fn desired(&self) -> ::std::option::Option<i32> {
        self.desired
    }
    /// <p>The total number of simultaneous streaming instances that are running.</p>
    pub fn running(&self) -> ::std::option::Option<i32> {
        self.running
    }
    /// <p>The number of instances in use for streaming.</p>
    pub fn in_use(&self) -> ::std::option::Option<i32> {
        self.in_use
    }
    /// <p>The number of currently available instances that can be used to stream sessions.</p>
    pub fn available(&self) -> ::std::option::Option<i32> {
        self.available
    }
}
impl ComputeCapacityStatus {
    /// Creates a new builder-style object to manufacture [`ComputeCapacityStatus`](crate::types::ComputeCapacityStatus).
    pub fn builder() -> crate::types::builders::ComputeCapacityStatusBuilder {
        crate::types::builders::ComputeCapacityStatusBuilder::default()
    }
}

/// A builder for [`ComputeCapacityStatus`](crate::types::ComputeCapacityStatus).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ComputeCapacityStatusBuilder {
    pub(crate) desired: ::std::option::Option<i32>,
    pub(crate) running: ::std::option::Option<i32>,
    pub(crate) in_use: ::std::option::Option<i32>,
    pub(crate) available: ::std::option::Option<i32>,
}
impl ComputeCapacityStatusBuilder {
    /// <p>The desired number of streaming instances.</p>
    pub fn desired(mut self, input: i32) -> Self {
        self.desired = ::std::option::Option::Some(input);
        self
    }
    /// <p>The desired number of streaming instances.</p>
    pub fn set_desired(mut self, input: ::std::option::Option<i32>) -> Self {
        self.desired = input;
        self
    }
    /// <p>The total number of simultaneous streaming instances that are running.</p>
    pub fn running(mut self, input: i32) -> Self {
        self.running = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total number of simultaneous streaming instances that are running.</p>
    pub fn set_running(mut self, input: ::std::option::Option<i32>) -> Self {
        self.running = input;
        self
    }
    /// <p>The number of instances in use for streaming.</p>
    pub fn in_use(mut self, input: i32) -> Self {
        self.in_use = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of instances in use for streaming.</p>
    pub fn set_in_use(mut self, input: ::std::option::Option<i32>) -> Self {
        self.in_use = input;
        self
    }
    /// <p>The number of currently available instances that can be used to stream sessions.</p>
    pub fn available(mut self, input: i32) -> Self {
        self.available = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of currently available instances that can be used to stream sessions.</p>
    pub fn set_available(mut self, input: ::std::option::Option<i32>) -> Self {
        self.available = input;
        self
    }
    /// Consumes the builder and constructs a [`ComputeCapacityStatus`](crate::types::ComputeCapacityStatus).
    pub fn build(self) -> crate::types::ComputeCapacityStatus {
        crate::types::ComputeCapacityStatus {
            desired: self.desired,
            running: self.running,
            in_use: self.in_use,
            available: self.available,
        }
    }
}
