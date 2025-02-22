// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteWorkspaceBundleInput {
    /// <p>The identifier of the bundle.</p>
    #[doc(hidden)]
    pub bundle_id: ::std::option::Option<::std::string::String>,
}
impl DeleteWorkspaceBundleInput {
    /// <p>The identifier of the bundle.</p>
    pub fn bundle_id(&self) -> ::std::option::Option<&str> {
        self.bundle_id.as_deref()
    }
}
impl DeleteWorkspaceBundleInput {
    /// Creates a new builder-style object to manufacture [`DeleteWorkspaceBundleInput`](crate::operation::delete_workspace_bundle::DeleteWorkspaceBundleInput).
    pub fn builder(
    ) -> crate::operation::delete_workspace_bundle::builders::DeleteWorkspaceBundleInputBuilder
    {
        crate::operation::delete_workspace_bundle::builders::DeleteWorkspaceBundleInputBuilder::default()
    }
}

/// A builder for [`DeleteWorkspaceBundleInput`](crate::operation::delete_workspace_bundle::DeleteWorkspaceBundleInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteWorkspaceBundleInputBuilder {
    pub(crate) bundle_id: ::std::option::Option<::std::string::String>,
}
impl DeleteWorkspaceBundleInputBuilder {
    /// <p>The identifier of the bundle.</p>
    pub fn bundle_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bundle_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the bundle.</p>
    pub fn set_bundle_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bundle_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteWorkspaceBundleInput`](crate::operation::delete_workspace_bundle::DeleteWorkspaceBundleInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_workspace_bundle::DeleteWorkspaceBundleInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_workspace_bundle::DeleteWorkspaceBundleInput {
                bundle_id: self.bundle_id,
            },
        )
    }
}
