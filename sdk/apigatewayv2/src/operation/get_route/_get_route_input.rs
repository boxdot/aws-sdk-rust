// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetRouteInput {
    /// <p>The API identifier.</p>
    #[doc(hidden)]
    pub api_id: ::std::option::Option<::std::string::String>,
    /// <p>The route ID.</p>
    #[doc(hidden)]
    pub route_id: ::std::option::Option<::std::string::String>,
}
impl GetRouteInput {
    /// <p>The API identifier.</p>
    pub fn api_id(&self) -> ::std::option::Option<&str> {
        self.api_id.as_deref()
    }
    /// <p>The route ID.</p>
    pub fn route_id(&self) -> ::std::option::Option<&str> {
        self.route_id.as_deref()
    }
}
impl GetRouteInput {
    /// Creates a new builder-style object to manufacture [`GetRouteInput`](crate::operation::get_route::GetRouteInput).
    pub fn builder() -> crate::operation::get_route::builders::GetRouteInputBuilder {
        crate::operation::get_route::builders::GetRouteInputBuilder::default()
    }
}

/// A builder for [`GetRouteInput`](crate::operation::get_route::GetRouteInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetRouteInputBuilder {
    pub(crate) api_id: ::std::option::Option<::std::string::String>,
    pub(crate) route_id: ::std::option::Option<::std::string::String>,
}
impl GetRouteInputBuilder {
    /// <p>The API identifier.</p>
    pub fn api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.api_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The API identifier.</p>
    pub fn set_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.api_id = input;
        self
    }
    /// <p>The route ID.</p>
    pub fn route_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.route_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The route ID.</p>
    pub fn set_route_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.route_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetRouteInput`](crate::operation::get_route::GetRouteInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_route::GetRouteInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_route::GetRouteInput {
            api_id: self.api_id,
            route_id: self.route_id,
        })
    }
}
