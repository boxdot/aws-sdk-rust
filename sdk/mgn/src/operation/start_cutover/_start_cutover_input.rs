// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct StartCutoverInput {
    /// <p>Start Cutover by Source Server IDs.</p>
    #[doc(hidden)]
    pub source_server_i_ds: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Start Cutover by Tags.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl StartCutoverInput {
    /// <p>Start Cutover by Source Server IDs.</p>
    pub fn source_server_i_ds(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.source_server_i_ds.as_deref()
    }
    /// <p>Start Cutover by Tags.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl ::std::fmt::Debug for StartCutoverInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("StartCutoverInput");
        formatter.field("source_server_i_ds", &self.source_server_i_ds);
        formatter.field("tags", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl StartCutoverInput {
    /// Creates a new builder-style object to manufacture [`StartCutoverInput`](crate::operation::start_cutover::StartCutoverInput).
    pub fn builder() -> crate::operation::start_cutover::builders::StartCutoverInputBuilder {
        crate::operation::start_cutover::builders::StartCutoverInputBuilder::default()
    }
}

/// A builder for [`StartCutoverInput`](crate::operation::start_cutover::StartCutoverInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct StartCutoverInputBuilder {
    pub(crate) source_server_i_ds: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl StartCutoverInputBuilder {
    /// Appends an item to `source_server_i_ds`.
    ///
    /// To override the contents of this collection use [`set_source_server_i_ds`](Self::set_source_server_i_ds).
    ///
    /// <p>Start Cutover by Source Server IDs.</p>
    pub fn source_server_i_ds(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.source_server_i_ds.unwrap_or_default();
        v.push(input.into());
        self.source_server_i_ds = ::std::option::Option::Some(v);
        self
    }
    /// <p>Start Cutover by Source Server IDs.</p>
    pub fn set_source_server_i_ds(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.source_server_i_ds = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Start Cutover by Tags.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Start Cutover by Tags.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`StartCutoverInput`](crate::operation::start_cutover::StartCutoverInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_cutover::StartCutoverInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::start_cutover::StartCutoverInput {
            source_server_i_ds: self.source_server_i_ds,
            tags: self.tags,
        })
    }
}
impl ::std::fmt::Debug for StartCutoverInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("StartCutoverInputBuilder");
        formatter.field("source_server_i_ds", &self.source_server_i_ds);
        formatter.field("tags", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
