// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Summary information about the content.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ContentSummary {
    /// <p>The Amazon Resource Name (ARN) of the content.</p>
    #[doc(hidden)]
    pub content_arn: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the content.</p>
    #[doc(hidden)]
    pub content_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the knowledge base.</p>
    #[doc(hidden)]
    pub knowledge_base_arn: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the knowledge base.</p>
    #[doc(hidden)]
    pub knowledge_base_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the content.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the revision of the content.</p>
    #[doc(hidden)]
    pub revision_id: ::std::option::Option<::std::string::String>,
    /// <p>The title of the content.</p>
    #[doc(hidden)]
    pub title: ::std::option::Option<::std::string::String>,
    /// <p>The media type of the content.</p>
    #[doc(hidden)]
    pub content_type: ::std::option::Option<::std::string::String>,
    /// <p>The status of the content.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::ContentStatus>,
    /// <p>A key/value map to store attributes without affecting tagging or recommendations. For example, when synchronizing data between an external system and Wisdom, you can store an external version identifier as metadata to utilize for determining drift.</p>
    #[doc(hidden)]
    pub metadata: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl ContentSummary {
    /// <p>The Amazon Resource Name (ARN) of the content.</p>
    pub fn content_arn(&self) -> ::std::option::Option<&str> {
        self.content_arn.as_deref()
    }
    /// <p>The identifier of the content.</p>
    pub fn content_id(&self) -> ::std::option::Option<&str> {
        self.content_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the knowledge base.</p>
    pub fn knowledge_base_arn(&self) -> ::std::option::Option<&str> {
        self.knowledge_base_arn.as_deref()
    }
    /// <p>The identifier of the knowledge base.</p>
    pub fn knowledge_base_id(&self) -> ::std::option::Option<&str> {
        self.knowledge_base_id.as_deref()
    }
    /// <p>The name of the content.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The identifier of the revision of the content.</p>
    pub fn revision_id(&self) -> ::std::option::Option<&str> {
        self.revision_id.as_deref()
    }
    /// <p>The title of the content.</p>
    pub fn title(&self) -> ::std::option::Option<&str> {
        self.title.as_deref()
    }
    /// <p>The media type of the content.</p>
    pub fn content_type(&self) -> ::std::option::Option<&str> {
        self.content_type.as_deref()
    }
    /// <p>The status of the content.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ContentStatus> {
        self.status.as_ref()
    }
    /// <p>A key/value map to store attributes without affecting tagging or recommendations. For example, when synchronizing data between an external system and Wisdom, you can store an external version identifier as metadata to utilize for determining drift.</p>
    pub fn metadata(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.metadata.as_ref()
    }
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl ContentSummary {
    /// Creates a new builder-style object to manufacture [`ContentSummary`](crate::types::ContentSummary).
    pub fn builder() -> crate::types::builders::ContentSummaryBuilder {
        crate::types::builders::ContentSummaryBuilder::default()
    }
}

/// A builder for [`ContentSummary`](crate::types::ContentSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ContentSummaryBuilder {
    pub(crate) content_arn: ::std::option::Option<::std::string::String>,
    pub(crate) content_id: ::std::option::Option<::std::string::String>,
    pub(crate) knowledge_base_arn: ::std::option::Option<::std::string::String>,
    pub(crate) knowledge_base_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) revision_id: ::std::option::Option<::std::string::String>,
    pub(crate) title: ::std::option::Option<::std::string::String>,
    pub(crate) content_type: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ContentStatus>,
    pub(crate) metadata: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl ContentSummaryBuilder {
    /// <p>The Amazon Resource Name (ARN) of the content.</p>
    pub fn content_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the content.</p>
    pub fn set_content_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content_arn = input;
        self
    }
    /// <p>The identifier of the content.</p>
    pub fn content_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the content.</p>
    pub fn set_content_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the knowledge base.</p>
    pub fn knowledge_base_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.knowledge_base_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the knowledge base.</p>
    pub fn set_knowledge_base_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.knowledge_base_arn = input;
        self
    }
    /// <p>The identifier of the knowledge base.</p>
    pub fn knowledge_base_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.knowledge_base_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the knowledge base.</p>
    pub fn set_knowledge_base_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.knowledge_base_id = input;
        self
    }
    /// <p>The name of the content.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the content.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The identifier of the revision of the content.</p>
    pub fn revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.revision_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the revision of the content.</p>
    pub fn set_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.revision_id = input;
        self
    }
    /// <p>The title of the content.</p>
    pub fn title(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.title = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The title of the content.</p>
    pub fn set_title(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.title = input;
        self
    }
    /// <p>The media type of the content.</p>
    pub fn content_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The media type of the content.</p>
    pub fn set_content_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content_type = input;
        self
    }
    /// <p>The status of the content.</p>
    pub fn status(mut self, input: crate::types::ContentStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the content.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ContentStatus>) -> Self {
        self.status = input;
        self
    }
    /// Adds a key-value pair to `metadata`.
    ///
    /// To override the contents of this collection use [`set_metadata`](Self::set_metadata).
    ///
    /// <p>A key/value map to store attributes without affecting tagging or recommendations. For example, when synchronizing data between an external system and Wisdom, you can store an external version identifier as metadata to utilize for determining drift.</p>
    pub fn metadata(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.metadata.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.metadata = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A key/value map to store attributes without affecting tagging or recommendations. For example, when synchronizing data between an external system and Wisdom, you can store an external version identifier as metadata to utilize for determining drift.</p>
    pub fn set_metadata(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.metadata = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags used to organize, track, or control access for this resource.</p>
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
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`ContentSummary`](crate::types::ContentSummary).
    pub fn build(self) -> crate::types::ContentSummary {
        crate::types::ContentSummary {
            content_arn: self.content_arn,
            content_id: self.content_id,
            knowledge_base_arn: self.knowledge_base_arn,
            knowledge_base_id: self.knowledge_base_id,
            name: self.name,
            revision_id: self.revision_id,
            title: self.title,
            content_type: self.content_type,
            status: self.status,
            metadata: self.metadata,
            tags: self.tags,
        }
    }
}
