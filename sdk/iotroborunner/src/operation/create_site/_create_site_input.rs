// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateSiteInput {
    /// Token used for detecting replayed requests. Replayed requests will not be performed multiple times.
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// Human friendly name of the resource.
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// A valid ISO 3166-1 alpha-2 code for the country in which the site resides. e.g., US.
    #[doc(hidden)]
    pub country_code: ::std::option::Option<::std::string::String>,
    /// A high-level description of the site.
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
}
impl CreateSiteInput {
    /// Token used for detecting replayed requests. Replayed requests will not be performed multiple times.
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// Human friendly name of the resource.
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// A valid ISO 3166-1 alpha-2 code for the country in which the site resides. e.g., US.
    pub fn country_code(&self) -> ::std::option::Option<&str> {
        self.country_code.as_deref()
    }
    /// A high-level description of the site.
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl CreateSiteInput {
    /// Creates a new builder-style object to manufacture [`CreateSiteInput`](crate::operation::create_site::CreateSiteInput).
    pub fn builder() -> crate::operation::create_site::builders::CreateSiteInputBuilder {
        crate::operation::create_site::builders::CreateSiteInputBuilder::default()
    }
}

/// A builder for [`CreateSiteInput`](crate::operation::create_site::CreateSiteInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateSiteInputBuilder {
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) country_code: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
}
impl CreateSiteInputBuilder {
    /// Token used for detecting replayed requests. Replayed requests will not be performed multiple times.
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// Token used for detecting replayed requests. Replayed requests will not be performed multiple times.
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Human friendly name of the resource.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// Human friendly name of the resource.
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// A valid ISO 3166-1 alpha-2 code for the country in which the site resides. e.g., US.
    pub fn country_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.country_code = ::std::option::Option::Some(input.into());
        self
    }
    /// A valid ISO 3166-1 alpha-2 code for the country in which the site resides. e.g., US.
    pub fn set_country_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.country_code = input;
        self
    }
    /// A high-level description of the site.
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// A high-level description of the site.
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateSiteInput`](crate::operation::create_site::CreateSiteInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_site::CreateSiteInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_site::CreateSiteInput {
            client_token: self.client_token,
            name: self.name,
            country_code: self.country_code,
            description: self.description,
        })
    }
}
