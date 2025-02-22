// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeVolumesModificationsOutput {
    /// <p>Information about the volume modifications.</p>
    #[doc(hidden)]
    pub volumes_modifications:
        ::std::option::Option<::std::vec::Vec<crate::types::VolumeModification>>,
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> if there are no more items to return.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeVolumesModificationsOutput {
    /// <p>Information about the volume modifications.</p>
    pub fn volumes_modifications(
        &self,
    ) -> ::std::option::Option<&[crate::types::VolumeModification]> {
        self.volumes_modifications.as_deref()
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> if there are no more items to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeVolumesModificationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeVolumesModificationsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeVolumesModificationsOutput`](crate::operation::describe_volumes_modifications::DescribeVolumesModificationsOutput).
    pub fn builder() -> crate::operation::describe_volumes_modifications::builders::DescribeVolumesModificationsOutputBuilder{
        crate::operation::describe_volumes_modifications::builders::DescribeVolumesModificationsOutputBuilder::default()
    }
}

/// A builder for [`DescribeVolumesModificationsOutput`](crate::operation::describe_volumes_modifications::DescribeVolumesModificationsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeVolumesModificationsOutputBuilder {
    pub(crate) volumes_modifications:
        ::std::option::Option<::std::vec::Vec<crate::types::VolumeModification>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeVolumesModificationsOutputBuilder {
    /// Appends an item to `volumes_modifications`.
    ///
    /// To override the contents of this collection use [`set_volumes_modifications`](Self::set_volumes_modifications).
    ///
    /// <p>Information about the volume modifications.</p>
    pub fn volumes_modifications(mut self, input: crate::types::VolumeModification) -> Self {
        let mut v = self.volumes_modifications.unwrap_or_default();
        v.push(input);
        self.volumes_modifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the volume modifications.</p>
    pub fn set_volumes_modifications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::VolumeModification>>,
    ) -> Self {
        self.volumes_modifications = input;
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> if there are no more items to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> if there are no more items to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`DescribeVolumesModificationsOutput`](crate::operation::describe_volumes_modifications::DescribeVolumesModificationsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_volumes_modifications::DescribeVolumesModificationsOutput {
        crate::operation::describe_volumes_modifications::DescribeVolumesModificationsOutput {
            volumes_modifications: self.volumes_modifications,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
