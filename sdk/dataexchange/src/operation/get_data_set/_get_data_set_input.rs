// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetDataSetInput {
    /// <p>The unique identifier for a data set.</p>
    #[doc(hidden)]
    pub data_set_id: ::std::option::Option<::std::string::String>,
}
impl GetDataSetInput {
    /// <p>The unique identifier for a data set.</p>
    pub fn data_set_id(&self) -> ::std::option::Option<&str> {
        self.data_set_id.as_deref()
    }
}
impl GetDataSetInput {
    /// Creates a new builder-style object to manufacture [`GetDataSetInput`](crate::operation::get_data_set::GetDataSetInput).
    pub fn builder() -> crate::operation::get_data_set::builders::GetDataSetInputBuilder {
        crate::operation::get_data_set::builders::GetDataSetInputBuilder::default()
    }
}

/// A builder for [`GetDataSetInput`](crate::operation::get_data_set::GetDataSetInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetDataSetInputBuilder {
    pub(crate) data_set_id: ::std::option::Option<::std::string::String>,
}
impl GetDataSetInputBuilder {
    /// <p>The unique identifier for a data set.</p>
    pub fn data_set_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.data_set_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for a data set.</p>
    pub fn set_data_set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.data_set_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetDataSetInput`](crate::operation::get_data_set::GetDataSetInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_data_set::GetDataSetInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_data_set::GetDataSetInput {
            data_set_id: self.data_set_id,
        })
    }
}
