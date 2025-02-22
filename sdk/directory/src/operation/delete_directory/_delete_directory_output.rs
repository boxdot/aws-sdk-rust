// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the results of the <code>DeleteDirectory</code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteDirectoryOutput {
    /// <p>The directory identifier.</p>
    #[doc(hidden)]
    pub directory_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DeleteDirectoryOutput {
    /// <p>The directory identifier.</p>
    pub fn directory_id(&self) -> ::std::option::Option<&str> {
        self.directory_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DeleteDirectoryOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteDirectoryOutput {
    /// Creates a new builder-style object to manufacture [`DeleteDirectoryOutput`](crate::operation::delete_directory::DeleteDirectoryOutput).
    pub fn builder() -> crate::operation::delete_directory::builders::DeleteDirectoryOutputBuilder {
        crate::operation::delete_directory::builders::DeleteDirectoryOutputBuilder::default()
    }
}

/// A builder for [`DeleteDirectoryOutput`](crate::operation::delete_directory::DeleteDirectoryOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteDirectoryOutputBuilder {
    pub(crate) directory_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DeleteDirectoryOutputBuilder {
    /// <p>The directory identifier.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.directory_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The directory identifier.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.directory_id = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteDirectoryOutput`](crate::operation::delete_directory::DeleteDirectoryOutput).
    pub fn build(self) -> crate::operation::delete_directory::DeleteDirectoryOutput {
        crate::operation::delete_directory::DeleteDirectoryOutput {
            directory_id: self.directory_id,
            _request_id: self._request_id,
        }
    }
}
