// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SetTagsForResourceInput {
    /// <p>The ARN of the assessment template that you want to set tags to.</p>
    #[doc(hidden)]
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p>A collection of key and value pairs that you want to set to the assessment template.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl SetTagsForResourceInput {
    /// <p>The ARN of the assessment template that you want to set tags to.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>A collection of key and value pairs that you want to set to the assessment template.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl SetTagsForResourceInput {
    /// Creates a new builder-style object to manufacture [`SetTagsForResourceInput`](crate::operation::set_tags_for_resource::SetTagsForResourceInput).
    pub fn builder(
    ) -> crate::operation::set_tags_for_resource::builders::SetTagsForResourceInputBuilder {
        crate::operation::set_tags_for_resource::builders::SetTagsForResourceInputBuilder::default()
    }
}

/// A builder for [`SetTagsForResourceInput`](crate::operation::set_tags_for_resource::SetTagsForResourceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SetTagsForResourceInputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl SetTagsForResourceInputBuilder {
    /// <p>The ARN of the assessment template that you want to set tags to.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the assessment template that you want to set tags to.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A collection of key and value pairs that you want to set to the assessment template.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>A collection of key and value pairs that you want to set to the assessment template.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`SetTagsForResourceInput`](crate::operation::set_tags_for_resource::SetTagsForResourceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::set_tags_for_resource::SetTagsForResourceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::set_tags_for_resource::SetTagsForResourceInput {
                resource_arn: self.resource_arn,
                tags: self.tags,
            },
        )
    }
}
