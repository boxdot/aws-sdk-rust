// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CreateUserInput {
    /// <p>The identifier of the organization for which the user is created.</p>
    #[doc(hidden)]
    pub organization_id: ::std::option::Option<::std::string::String>,
    /// <p>The name for the new user. WorkMail directory user names have a maximum length of 64. All others have a maximum length of 20.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The display name for the new user.</p>
    #[doc(hidden)]
    pub display_name: ::std::option::Option<::std::string::String>,
    /// <p>The password for the new user.</p>
    #[doc(hidden)]
    pub password: ::std::option::Option<::std::string::String>,
}
impl CreateUserInput {
    /// <p>The identifier of the organization for which the user is created.</p>
    pub fn organization_id(&self) -> ::std::option::Option<&str> {
        self.organization_id.as_deref()
    }
    /// <p>The name for the new user. WorkMail directory user names have a maximum length of 64. All others have a maximum length of 20.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The display name for the new user.</p>
    pub fn display_name(&self) -> ::std::option::Option<&str> {
        self.display_name.as_deref()
    }
    /// <p>The password for the new user.</p>
    pub fn password(&self) -> ::std::option::Option<&str> {
        self.password.as_deref()
    }
}
impl ::std::fmt::Debug for CreateUserInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateUserInput");
        formatter.field("organization_id", &self.organization_id);
        formatter.field("name", &self.name);
        formatter.field("display_name", &self.display_name);
        formatter.field("password", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl CreateUserInput {
    /// Creates a new builder-style object to manufacture [`CreateUserInput`](crate::operation::create_user::CreateUserInput).
    pub fn builder() -> crate::operation::create_user::builders::CreateUserInputBuilder {
        crate::operation::create_user::builders::CreateUserInputBuilder::default()
    }
}

/// A builder for [`CreateUserInput`](crate::operation::create_user::CreateUserInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct CreateUserInputBuilder {
    pub(crate) organization_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) display_name: ::std::option::Option<::std::string::String>,
    pub(crate) password: ::std::option::Option<::std::string::String>,
}
impl CreateUserInputBuilder {
    /// <p>The identifier of the organization for which the user is created.</p>
    pub fn organization_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.organization_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the organization for which the user is created.</p>
    pub fn set_organization_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.organization_id = input;
        self
    }
    /// <p>The name for the new user. WorkMail directory user names have a maximum length of 64. All others have a maximum length of 20.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name for the new user. WorkMail directory user names have a maximum length of 64. All others have a maximum length of 20.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The display name for the new user.</p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.display_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The display name for the new user.</p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.display_name = input;
        self
    }
    /// <p>The password for the new user.</p>
    pub fn password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.password = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The password for the new user.</p>
    pub fn set_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.password = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateUserInput`](crate::operation::create_user::CreateUserInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_user::CreateUserInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_user::CreateUserInput {
            organization_id: self.organization_id,
            name: self.name,
            display_name: self.display_name,
            password: self.password,
        })
    }
}
impl ::std::fmt::Debug for CreateUserInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateUserInputBuilder");
        formatter.field("organization_id", &self.organization_id);
        formatter.field("name", &self.name);
        formatter.field("display_name", &self.display_name);
        formatter.field("password", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
