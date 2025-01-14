// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Configuration settings for how to perform the auto-merging of profiles.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AutoMerging {
    /// <p>The flag that enables the auto-merging of duplicate profiles.</p>
    #[doc(hidden)]
    pub enabled: ::std::option::Option<bool>,
    /// <p>A list of matching attributes that represent matching criteria. If two profiles meet at least one of the requirements in the matching attributes list, they will be merged.</p>
    #[doc(hidden)]
    pub consolidation: ::std::option::Option<crate::types::Consolidation>,
    /// <p>How the auto-merging process should resolve conflicts between different profiles. For example, if Profile A and Profile B have the same <code>FirstName</code> and <code>LastName</code> (and that is the matching criteria), which <code>EmailAddress</code> should be used? </p>
    #[doc(hidden)]
    pub conflict_resolution: ::std::option::Option<crate::types::ConflictResolution>,
    /// <p>A number between 0 and 1 that represents the minimum confidence score required for profiles within a matching group to be merged during the auto-merge process. A higher score means higher similarity required to merge profiles. </p>
    #[doc(hidden)]
    pub min_allowed_confidence_score_for_merging: ::std::option::Option<f64>,
}
impl AutoMerging {
    /// <p>The flag that enables the auto-merging of duplicate profiles.</p>
    pub fn enabled(&self) -> ::std::option::Option<bool> {
        self.enabled
    }
    /// <p>A list of matching attributes that represent matching criteria. If two profiles meet at least one of the requirements in the matching attributes list, they will be merged.</p>
    pub fn consolidation(&self) -> ::std::option::Option<&crate::types::Consolidation> {
        self.consolidation.as_ref()
    }
    /// <p>How the auto-merging process should resolve conflicts between different profiles. For example, if Profile A and Profile B have the same <code>FirstName</code> and <code>LastName</code> (and that is the matching criteria), which <code>EmailAddress</code> should be used? </p>
    pub fn conflict_resolution(&self) -> ::std::option::Option<&crate::types::ConflictResolution> {
        self.conflict_resolution.as_ref()
    }
    /// <p>A number between 0 and 1 that represents the minimum confidence score required for profiles within a matching group to be merged during the auto-merge process. A higher score means higher similarity required to merge profiles. </p>
    pub fn min_allowed_confidence_score_for_merging(&self) -> ::std::option::Option<f64> {
        self.min_allowed_confidence_score_for_merging
    }
}
impl AutoMerging {
    /// Creates a new builder-style object to manufacture [`AutoMerging`](crate::types::AutoMerging).
    pub fn builder() -> crate::types::builders::AutoMergingBuilder {
        crate::types::builders::AutoMergingBuilder::default()
    }
}

/// A builder for [`AutoMerging`](crate::types::AutoMerging).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AutoMergingBuilder {
    pub(crate) enabled: ::std::option::Option<bool>,
    pub(crate) consolidation: ::std::option::Option<crate::types::Consolidation>,
    pub(crate) conflict_resolution: ::std::option::Option<crate::types::ConflictResolution>,
    pub(crate) min_allowed_confidence_score_for_merging: ::std::option::Option<f64>,
}
impl AutoMergingBuilder {
    /// <p>The flag that enables the auto-merging of duplicate profiles.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>The flag that enables the auto-merging of duplicate profiles.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>A list of matching attributes that represent matching criteria. If two profiles meet at least one of the requirements in the matching attributes list, they will be merged.</p>
    pub fn consolidation(mut self, input: crate::types::Consolidation) -> Self {
        self.consolidation = ::std::option::Option::Some(input);
        self
    }
    /// <p>A list of matching attributes that represent matching criteria. If two profiles meet at least one of the requirements in the matching attributes list, they will be merged.</p>
    pub fn set_consolidation(
        mut self,
        input: ::std::option::Option<crate::types::Consolidation>,
    ) -> Self {
        self.consolidation = input;
        self
    }
    /// <p>How the auto-merging process should resolve conflicts between different profiles. For example, if Profile A and Profile B have the same <code>FirstName</code> and <code>LastName</code> (and that is the matching criteria), which <code>EmailAddress</code> should be used? </p>
    pub fn conflict_resolution(mut self, input: crate::types::ConflictResolution) -> Self {
        self.conflict_resolution = ::std::option::Option::Some(input);
        self
    }
    /// <p>How the auto-merging process should resolve conflicts between different profiles. For example, if Profile A and Profile B have the same <code>FirstName</code> and <code>LastName</code> (and that is the matching criteria), which <code>EmailAddress</code> should be used? </p>
    pub fn set_conflict_resolution(
        mut self,
        input: ::std::option::Option<crate::types::ConflictResolution>,
    ) -> Self {
        self.conflict_resolution = input;
        self
    }
    /// <p>A number between 0 and 1 that represents the minimum confidence score required for profiles within a matching group to be merged during the auto-merge process. A higher score means higher similarity required to merge profiles. </p>
    pub fn min_allowed_confidence_score_for_merging(mut self, input: f64) -> Self {
        self.min_allowed_confidence_score_for_merging = ::std::option::Option::Some(input);
        self
    }
    /// <p>A number between 0 and 1 that represents the minimum confidence score required for profiles within a matching group to be merged during the auto-merge process. A higher score means higher similarity required to merge profiles. </p>
    pub fn set_min_allowed_confidence_score_for_merging(
        mut self,
        input: ::std::option::Option<f64>,
    ) -> Self {
        self.min_allowed_confidence_score_for_merging = input;
        self
    }
    /// Consumes the builder and constructs a [`AutoMerging`](crate::types::AutoMerging).
    pub fn build(self) -> crate::types::AutoMerging {
        crate::types::AutoMerging {
            enabled: self.enabled,
            consolidation: self.consolidation,
            conflict_resolution: self.conflict_resolution,
            min_allowed_confidence_score_for_merging: self.min_allowed_confidence_score_for_merging,
        }
    }
}
