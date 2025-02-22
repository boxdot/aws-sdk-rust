// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the input of a <code>ModifyCacheParameterGroup</code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyCacheParameterGroupInput {
    /// <p>The name of the cache parameter group to modify.</p>
    #[doc(hidden)]
    pub cache_parameter_group_name: ::std::option::Option<::std::string::String>,
    /// <p>An array of parameter names and values for the parameter update. You must supply at least one parameter name and value; subsequent arguments are optional. A maximum of 20 parameters may be modified per request.</p>
    #[doc(hidden)]
    pub parameter_name_values:
        ::std::option::Option<::std::vec::Vec<crate::types::ParameterNameValue>>,
}
impl ModifyCacheParameterGroupInput {
    /// <p>The name of the cache parameter group to modify.</p>
    pub fn cache_parameter_group_name(&self) -> ::std::option::Option<&str> {
        self.cache_parameter_group_name.as_deref()
    }
    /// <p>An array of parameter names and values for the parameter update. You must supply at least one parameter name and value; subsequent arguments are optional. A maximum of 20 parameters may be modified per request.</p>
    pub fn parameter_name_values(
        &self,
    ) -> ::std::option::Option<&[crate::types::ParameterNameValue]> {
        self.parameter_name_values.as_deref()
    }
}
impl ModifyCacheParameterGroupInput {
    /// Creates a new builder-style object to manufacture [`ModifyCacheParameterGroupInput`](crate::operation::modify_cache_parameter_group::ModifyCacheParameterGroupInput).
    pub fn builder() -> crate::operation::modify_cache_parameter_group::builders::ModifyCacheParameterGroupInputBuilder{
        crate::operation::modify_cache_parameter_group::builders::ModifyCacheParameterGroupInputBuilder::default()
    }
}

/// A builder for [`ModifyCacheParameterGroupInput`](crate::operation::modify_cache_parameter_group::ModifyCacheParameterGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ModifyCacheParameterGroupInputBuilder {
    pub(crate) cache_parameter_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) parameter_name_values:
        ::std::option::Option<::std::vec::Vec<crate::types::ParameterNameValue>>,
}
impl ModifyCacheParameterGroupInputBuilder {
    /// <p>The name of the cache parameter group to modify.</p>
    pub fn cache_parameter_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.cache_parameter_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the cache parameter group to modify.</p>
    pub fn set_cache_parameter_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.cache_parameter_group_name = input;
        self
    }
    /// Appends an item to `parameter_name_values`.
    ///
    /// To override the contents of this collection use [`set_parameter_name_values`](Self::set_parameter_name_values).
    ///
    /// <p>An array of parameter names and values for the parameter update. You must supply at least one parameter name and value; subsequent arguments are optional. A maximum of 20 parameters may be modified per request.</p>
    pub fn parameter_name_values(mut self, input: crate::types::ParameterNameValue) -> Self {
        let mut v = self.parameter_name_values.unwrap_or_default();
        v.push(input);
        self.parameter_name_values = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of parameter names and values for the parameter update. You must supply at least one parameter name and value; subsequent arguments are optional. A maximum of 20 parameters may be modified per request.</p>
    pub fn set_parameter_name_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ParameterNameValue>>,
    ) -> Self {
        self.parameter_name_values = input;
        self
    }
    /// Consumes the builder and constructs a [`ModifyCacheParameterGroupInput`](crate::operation::modify_cache_parameter_group::ModifyCacheParameterGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_cache_parameter_group::ModifyCacheParameterGroupInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::modify_cache_parameter_group::ModifyCacheParameterGroupInput {
                cache_parameter_group_name: self.cache_parameter_group_name,
                parameter_name_values: self.parameter_name_values,
            },
        )
    }
}
