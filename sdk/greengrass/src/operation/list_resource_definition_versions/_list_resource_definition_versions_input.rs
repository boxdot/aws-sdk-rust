// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListResourceDefinitionVersionsInput {
    /// The maximum number of results to be returned per request.
    #[doc(hidden)]
    pub max_results: ::std::option::Option<::std::string::String>,
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// The ID of the resource definition.
    #[doc(hidden)]
    pub resource_definition_id: ::std::option::Option<::std::string::String>,
}
impl ListResourceDefinitionVersionsInput {
    /// The maximum number of results to be returned per request.
    pub fn max_results(&self) -> ::std::option::Option<&str> {
        self.max_results.as_deref()
    }
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// The ID of the resource definition.
    pub fn resource_definition_id(&self) -> ::std::option::Option<&str> {
        self.resource_definition_id.as_deref()
    }
}
impl ListResourceDefinitionVersionsInput {
    /// Creates a new builder-style object to manufacture [`ListResourceDefinitionVersionsInput`](crate::operation::list_resource_definition_versions::ListResourceDefinitionVersionsInput).
    pub fn builder() -> crate::operation::list_resource_definition_versions::builders::ListResourceDefinitionVersionsInputBuilder{
        crate::operation::list_resource_definition_versions::builders::ListResourceDefinitionVersionsInputBuilder::default()
    }
}

/// A builder for [`ListResourceDefinitionVersionsInput`](crate::operation::list_resource_definition_versions::ListResourceDefinitionVersionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListResourceDefinitionVersionsInputBuilder {
    pub(crate) max_results: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) resource_definition_id: ::std::option::Option<::std::string::String>,
}
impl ListResourceDefinitionVersionsInputBuilder {
    /// The maximum number of results to be returned per request.
    pub fn max_results(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.max_results = ::std::option::Option::Some(input.into());
        self
    }
    /// The maximum number of results to be returned per request.
    pub fn set_max_results(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.max_results = input;
        self
    }
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// The ID of the resource definition.
    pub fn resource_definition_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.resource_definition_id = ::std::option::Option::Some(input.into());
        self
    }
    /// The ID of the resource definition.
    pub fn set_resource_definition_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.resource_definition_id = input;
        self
    }
    /// Consumes the builder and constructs a [`ListResourceDefinitionVersionsInput`](crate::operation::list_resource_definition_versions::ListResourceDefinitionVersionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_resource_definition_versions::ListResourceDefinitionVersionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_resource_definition_versions::ListResourceDefinitionVersionsInput {
                max_results: self.max_results
                ,
                next_token: self.next_token
                ,
                resource_definition_id: self.resource_definition_id
                ,
            }
        )
    }
}
