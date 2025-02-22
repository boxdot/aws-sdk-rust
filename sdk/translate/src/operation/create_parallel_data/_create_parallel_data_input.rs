// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateParallelDataInput {
    /// <p>A custom name for the parallel data resource in Amazon Translate. You must assign a name that is unique in the account and region.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>A custom description for the parallel data resource in Amazon Translate.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the format and S3 location of the parallel data input file.</p>
    #[doc(hidden)]
    pub parallel_data_config: ::std::option::Option<crate::types::ParallelDataConfig>,
    /// <p>The encryption key used to encrypt this object.</p>
    #[doc(hidden)]
    pub encryption_key: ::std::option::Option<crate::types::EncryptionKey>,
    /// <p>A unique identifier for the request. This token is automatically generated when you use Amazon Translate through an AWS SDK.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>Tags to be associated with this resource. A tag is a key-value pair that adds metadata to a resource. Each tag key for the resource must be unique. For more information, see <a href="https://docs.aws.amazon.com/translate/latest/dg/tagging.html"> Tagging your resources</a>.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateParallelDataInput {
    /// <p>A custom name for the parallel data resource in Amazon Translate. You must assign a name that is unique in the account and region.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>A custom description for the parallel data resource in Amazon Translate.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Specifies the format and S3 location of the parallel data input file.</p>
    pub fn parallel_data_config(&self) -> ::std::option::Option<&crate::types::ParallelDataConfig> {
        self.parallel_data_config.as_ref()
    }
    /// <p>The encryption key used to encrypt this object.</p>
    pub fn encryption_key(&self) -> ::std::option::Option<&crate::types::EncryptionKey> {
        self.encryption_key.as_ref()
    }
    /// <p>A unique identifier for the request. This token is automatically generated when you use Amazon Translate through an AWS SDK.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>Tags to be associated with this resource. A tag is a key-value pair that adds metadata to a resource. Each tag key for the resource must be unique. For more information, see <a href="https://docs.aws.amazon.com/translate/latest/dg/tagging.html"> Tagging your resources</a>.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl CreateParallelDataInput {
    /// Creates a new builder-style object to manufacture [`CreateParallelDataInput`](crate::operation::create_parallel_data::CreateParallelDataInput).
    pub fn builder(
    ) -> crate::operation::create_parallel_data::builders::CreateParallelDataInputBuilder {
        crate::operation::create_parallel_data::builders::CreateParallelDataInputBuilder::default()
    }
}

/// A builder for [`CreateParallelDataInput`](crate::operation::create_parallel_data::CreateParallelDataInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateParallelDataInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) parallel_data_config: ::std::option::Option<crate::types::ParallelDataConfig>,
    pub(crate) encryption_key: ::std::option::Option<crate::types::EncryptionKey>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateParallelDataInputBuilder {
    /// <p>A custom name for the parallel data resource in Amazon Translate. You must assign a name that is unique in the account and region.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A custom name for the parallel data resource in Amazon Translate. You must assign a name that is unique in the account and region.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A custom description for the parallel data resource in Amazon Translate.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A custom description for the parallel data resource in Amazon Translate.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>Specifies the format and S3 location of the parallel data input file.</p>
    pub fn parallel_data_config(mut self, input: crate::types::ParallelDataConfig) -> Self {
        self.parallel_data_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the format and S3 location of the parallel data input file.</p>
    pub fn set_parallel_data_config(
        mut self,
        input: ::std::option::Option<crate::types::ParallelDataConfig>,
    ) -> Self {
        self.parallel_data_config = input;
        self
    }
    /// <p>The encryption key used to encrypt this object.</p>
    pub fn encryption_key(mut self, input: crate::types::EncryptionKey) -> Self {
        self.encryption_key = ::std::option::Option::Some(input);
        self
    }
    /// <p>The encryption key used to encrypt this object.</p>
    pub fn set_encryption_key(
        mut self,
        input: ::std::option::Option<crate::types::EncryptionKey>,
    ) -> Self {
        self.encryption_key = input;
        self
    }
    /// <p>A unique identifier for the request. This token is automatically generated when you use Amazon Translate through an AWS SDK.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the request. This token is automatically generated when you use Amazon Translate through an AWS SDK.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags to be associated with this resource. A tag is a key-value pair that adds metadata to a resource. Each tag key for the resource must be unique. For more information, see <a href="https://docs.aws.amazon.com/translate/latest/dg/tagging.html"> Tagging your resources</a>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Tags to be associated with this resource. A tag is a key-value pair that adds metadata to a resource. Each tag key for the resource must be unique. For more information, see <a href="https://docs.aws.amazon.com/translate/latest/dg/tagging.html"> Tagging your resources</a>.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateParallelDataInput`](crate::operation::create_parallel_data::CreateParallelDataInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_parallel_data::CreateParallelDataInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_parallel_data::CreateParallelDataInput {
                name: self.name,
                description: self.description,
                parallel_data_config: self.parallel_data_config,
                encryption_key: self.encryption_key,
                client_token: self.client_token,
                tags: self.tags,
            },
        )
    }
}
