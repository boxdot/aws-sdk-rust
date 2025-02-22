// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteNotebookInput {
    /// <p>The ID of the notebook to delete.</p>
    #[doc(hidden)]
    pub notebook_id: ::std::option::Option<::std::string::String>,
}
impl DeleteNotebookInput {
    /// <p>The ID of the notebook to delete.</p>
    pub fn notebook_id(&self) -> ::std::option::Option<&str> {
        self.notebook_id.as_deref()
    }
}
impl DeleteNotebookInput {
    /// Creates a new builder-style object to manufacture [`DeleteNotebookInput`](crate::operation::delete_notebook::DeleteNotebookInput).
    pub fn builder() -> crate::operation::delete_notebook::builders::DeleteNotebookInputBuilder {
        crate::operation::delete_notebook::builders::DeleteNotebookInputBuilder::default()
    }
}

/// A builder for [`DeleteNotebookInput`](crate::operation::delete_notebook::DeleteNotebookInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteNotebookInputBuilder {
    pub(crate) notebook_id: ::std::option::Option<::std::string::String>,
}
impl DeleteNotebookInputBuilder {
    /// <p>The ID of the notebook to delete.</p>
    pub fn notebook_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.notebook_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the notebook to delete.</p>
    pub fn set_notebook_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.notebook_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteNotebookInput`](crate::operation::delete_notebook::DeleteNotebookInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_notebook::DeleteNotebookInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_notebook::DeleteNotebookInput {
            notebook_id: self.notebook_id,
        })
    }
}
