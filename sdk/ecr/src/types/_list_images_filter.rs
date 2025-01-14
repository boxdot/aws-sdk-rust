// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object representing a filter on a <code>ListImages</code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListImagesFilter {
    /// <p>The tag status with which to filter your <code>ListImages</code> results. You can filter results based on whether they are <code>TAGGED</code> or <code>UNTAGGED</code>.</p>
    #[doc(hidden)]
    pub tag_status: ::std::option::Option<crate::types::TagStatus>,
}
impl ListImagesFilter {
    /// <p>The tag status with which to filter your <code>ListImages</code> results. You can filter results based on whether they are <code>TAGGED</code> or <code>UNTAGGED</code>.</p>
    pub fn tag_status(&self) -> ::std::option::Option<&crate::types::TagStatus> {
        self.tag_status.as_ref()
    }
}
impl ListImagesFilter {
    /// Creates a new builder-style object to manufacture [`ListImagesFilter`](crate::types::ListImagesFilter).
    pub fn builder() -> crate::types::builders::ListImagesFilterBuilder {
        crate::types::builders::ListImagesFilterBuilder::default()
    }
}

/// A builder for [`ListImagesFilter`](crate::types::ListImagesFilter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListImagesFilterBuilder {
    pub(crate) tag_status: ::std::option::Option<crate::types::TagStatus>,
}
impl ListImagesFilterBuilder {
    /// <p>The tag status with which to filter your <code>ListImages</code> results. You can filter results based on whether they are <code>TAGGED</code> or <code>UNTAGGED</code>.</p>
    pub fn tag_status(mut self, input: crate::types::TagStatus) -> Self {
        self.tag_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The tag status with which to filter your <code>ListImages</code> results. You can filter results based on whether they are <code>TAGGED</code> or <code>UNTAGGED</code>.</p>
    pub fn set_tag_status(mut self, input: ::std::option::Option<crate::types::TagStatus>) -> Self {
        self.tag_status = input;
        self
    }
    /// Consumes the builder and constructs a [`ListImagesFilter`](crate::types::ListImagesFilter).
    pub fn build(self) -> crate::types::ListImagesFilter {
        crate::types::ListImagesFilter {
            tag_status: self.tag_status,
        }
    }
}
