// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Lifecycle Cutover finalized</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LifeCycleLastCutoverFinalized {
    /// <p>Lifecycle Cutover finalized date and time.</p>
    #[doc(hidden)]
    pub api_call_date_time: ::std::option::Option<::std::string::String>,
}
impl LifeCycleLastCutoverFinalized {
    /// <p>Lifecycle Cutover finalized date and time.</p>
    pub fn api_call_date_time(&self) -> ::std::option::Option<&str> {
        self.api_call_date_time.as_deref()
    }
}
impl LifeCycleLastCutoverFinalized {
    /// Creates a new builder-style object to manufacture [`LifeCycleLastCutoverFinalized`](crate::types::LifeCycleLastCutoverFinalized).
    pub fn builder() -> crate::types::builders::LifeCycleLastCutoverFinalizedBuilder {
        crate::types::builders::LifeCycleLastCutoverFinalizedBuilder::default()
    }
}

/// A builder for [`LifeCycleLastCutoverFinalized`](crate::types::LifeCycleLastCutoverFinalized).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LifeCycleLastCutoverFinalizedBuilder {
    pub(crate) api_call_date_time: ::std::option::Option<::std::string::String>,
}
impl LifeCycleLastCutoverFinalizedBuilder {
    /// <p>Lifecycle Cutover finalized date and time.</p>
    pub fn api_call_date_time(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.api_call_date_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Lifecycle Cutover finalized date and time.</p>
    pub fn set_api_call_date_time(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.api_call_date_time = input;
        self
    }
    /// Consumes the builder and constructs a [`LifeCycleLastCutoverFinalized`](crate::types::LifeCycleLastCutoverFinalized).
    pub fn build(self) -> crate::types::LifeCycleLastCutoverFinalized {
        crate::types::LifeCycleLastCutoverFinalized {
            api_call_date_time: self.api_call_date_time,
        }
    }
}
