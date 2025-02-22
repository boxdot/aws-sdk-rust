// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateParameterGroupInput {
    /// <p>The name of the parameter group.</p>
    #[doc(hidden)]
    pub parameter_group_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the parameter group family that the parameter group can be used with.</p>
    #[doc(hidden)]
    pub family: ::std::option::Option<::std::string::String>,
    /// <p>An optional description of the parameter group.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value, although null is accepted.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateParameterGroupInput {
    /// <p>The name of the parameter group.</p>
    pub fn parameter_group_name(&self) -> ::std::option::Option<&str> {
        self.parameter_group_name.as_deref()
    }
    /// <p>The name of the parameter group family that the parameter group can be used with.</p>
    pub fn family(&self) -> ::std::option::Option<&str> {
        self.family.as_deref()
    }
    /// <p>An optional description of the parameter group.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value, although null is accepted.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl CreateParameterGroupInput {
    /// Creates a new builder-style object to manufacture [`CreateParameterGroupInput`](crate::operation::create_parameter_group::CreateParameterGroupInput).
    pub fn builder(
    ) -> crate::operation::create_parameter_group::builders::CreateParameterGroupInputBuilder {
        crate::operation::create_parameter_group::builders::CreateParameterGroupInputBuilder::default()
    }
}

/// A builder for [`CreateParameterGroupInput`](crate::operation::create_parameter_group::CreateParameterGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateParameterGroupInputBuilder {
    pub(crate) parameter_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) family: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateParameterGroupInputBuilder {
    /// <p>The name of the parameter group.</p>
    pub fn parameter_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.parameter_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the parameter group.</p>
    pub fn set_parameter_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.parameter_group_name = input;
        self
    }
    /// <p>The name of the parameter group family that the parameter group can be used with.</p>
    pub fn family(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.family = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the parameter group family that the parameter group can be used with.</p>
    pub fn set_family(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.family = input;
        self
    }
    /// <p>An optional description of the parameter group.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An optional description of the parameter group.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value, although null is accepted.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value, although null is accepted.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateParameterGroupInput`](crate::operation::create_parameter_group::CreateParameterGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_parameter_group::CreateParameterGroupInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_parameter_group::CreateParameterGroupInput {
                parameter_group_name: self.parameter_group_name,
                family: self.family,
                description: self.description,
                tags: self.tags,
            },
        )
    }
}
