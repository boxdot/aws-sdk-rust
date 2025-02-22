// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateApplicationSnapshotInput {
    /// <p>The name of an existing application</p>
    #[doc(hidden)]
    pub application_name: ::std::option::Option<::std::string::String>,
    /// <p>An identifier for the application snapshot.</p>
    #[doc(hidden)]
    pub snapshot_name: ::std::option::Option<::std::string::String>,
}
impl CreateApplicationSnapshotInput {
    /// <p>The name of an existing application</p>
    pub fn application_name(&self) -> ::std::option::Option<&str> {
        self.application_name.as_deref()
    }
    /// <p>An identifier for the application snapshot.</p>
    pub fn snapshot_name(&self) -> ::std::option::Option<&str> {
        self.snapshot_name.as_deref()
    }
}
impl CreateApplicationSnapshotInput {
    /// Creates a new builder-style object to manufacture [`CreateApplicationSnapshotInput`](crate::operation::create_application_snapshot::CreateApplicationSnapshotInput).
    pub fn builder() -> crate::operation::create_application_snapshot::builders::CreateApplicationSnapshotInputBuilder{
        crate::operation::create_application_snapshot::builders::CreateApplicationSnapshotInputBuilder::default()
    }
}

/// A builder for [`CreateApplicationSnapshotInput`](crate::operation::create_application_snapshot::CreateApplicationSnapshotInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateApplicationSnapshotInputBuilder {
    pub(crate) application_name: ::std::option::Option<::std::string::String>,
    pub(crate) snapshot_name: ::std::option::Option<::std::string::String>,
}
impl CreateApplicationSnapshotInputBuilder {
    /// <p>The name of an existing application</p>
    pub fn application_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of an existing application</p>
    pub fn set_application_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_name = input;
        self
    }
    /// <p>An identifier for the application snapshot.</p>
    pub fn snapshot_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.snapshot_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An identifier for the application snapshot.</p>
    pub fn set_snapshot_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.snapshot_name = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateApplicationSnapshotInput`](crate::operation::create_application_snapshot::CreateApplicationSnapshotInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_application_snapshot::CreateApplicationSnapshotInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_application_snapshot::CreateApplicationSnapshotInput {
                application_name: self.application_name,
                snapshot_name: self.snapshot_name,
            },
        )
    }
}
