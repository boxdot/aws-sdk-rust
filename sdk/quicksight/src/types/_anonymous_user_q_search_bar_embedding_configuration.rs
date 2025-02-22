// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The settings that you want to use with the Q search bar.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AnonymousUserQSearchBarEmbeddingConfiguration {
    /// <p>The QuickSight Q topic ID of the topic that you want the anonymous user to see first. This ID is included in the output URL. When the URL in response is accessed, Amazon QuickSight renders the Q search bar with this topic pre-selected.</p>
    /// <p>The Amazon Resource Name (ARN) of this Q topic must be included in the <code>AuthorizedResourceArns</code> parameter. Otherwise, the request will fail with <code>InvalidParameterValueException</code>.</p>
    #[doc(hidden)]
    pub initial_topic_id: ::std::option::Option<::std::string::String>,
}
impl AnonymousUserQSearchBarEmbeddingConfiguration {
    /// <p>The QuickSight Q topic ID of the topic that you want the anonymous user to see first. This ID is included in the output URL. When the URL in response is accessed, Amazon QuickSight renders the Q search bar with this topic pre-selected.</p>
    /// <p>The Amazon Resource Name (ARN) of this Q topic must be included in the <code>AuthorizedResourceArns</code> parameter. Otherwise, the request will fail with <code>InvalidParameterValueException</code>.</p>
    pub fn initial_topic_id(&self) -> ::std::option::Option<&str> {
        self.initial_topic_id.as_deref()
    }
}
impl AnonymousUserQSearchBarEmbeddingConfiguration {
    /// Creates a new builder-style object to manufacture [`AnonymousUserQSearchBarEmbeddingConfiguration`](crate::types::AnonymousUserQSearchBarEmbeddingConfiguration).
    pub fn builder() -> crate::types::builders::AnonymousUserQSearchBarEmbeddingConfigurationBuilder
    {
        crate::types::builders::AnonymousUserQSearchBarEmbeddingConfigurationBuilder::default()
    }
}

/// A builder for [`AnonymousUserQSearchBarEmbeddingConfiguration`](crate::types::AnonymousUserQSearchBarEmbeddingConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AnonymousUserQSearchBarEmbeddingConfigurationBuilder {
    pub(crate) initial_topic_id: ::std::option::Option<::std::string::String>,
}
impl AnonymousUserQSearchBarEmbeddingConfigurationBuilder {
    /// <p>The QuickSight Q topic ID of the topic that you want the anonymous user to see first. This ID is included in the output URL. When the URL in response is accessed, Amazon QuickSight renders the Q search bar with this topic pre-selected.</p>
    /// <p>The Amazon Resource Name (ARN) of this Q topic must be included in the <code>AuthorizedResourceArns</code> parameter. Otherwise, the request will fail with <code>InvalidParameterValueException</code>.</p>
    pub fn initial_topic_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.initial_topic_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The QuickSight Q topic ID of the topic that you want the anonymous user to see first. This ID is included in the output URL. When the URL in response is accessed, Amazon QuickSight renders the Q search bar with this topic pre-selected.</p>
    /// <p>The Amazon Resource Name (ARN) of this Q topic must be included in the <code>AuthorizedResourceArns</code> parameter. Otherwise, the request will fail with <code>InvalidParameterValueException</code>.</p>
    pub fn set_initial_topic_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.initial_topic_id = input;
        self
    }
    /// Consumes the builder and constructs a [`AnonymousUserQSearchBarEmbeddingConfiguration`](crate::types::AnonymousUserQSearchBarEmbeddingConfiguration).
    pub fn build(self) -> crate::types::AnonymousUserQSearchBarEmbeddingConfiguration {
        crate::types::AnonymousUserQSearchBarEmbeddingConfiguration {
            initial_topic_id: self.initial_topic_id,
        }
    }
}
