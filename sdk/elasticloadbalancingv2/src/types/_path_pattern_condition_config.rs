// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a path pattern condition.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PathPatternConditionConfig {
    /// <p>The path patterns to compare against the request URL. The maximum size of each string is 128 characters. The comparison is case sensitive. The following wildcard characters are supported: * (matches 0 or more characters) and ? (matches exactly 1 character).</p>
    /// <p>If you specify multiple strings, the condition is satisfied if one of them matches the request URL. The path pattern is compared only to the path of the URL, not to its query string. To compare against the query string, use <code>QueryStringConditionConfig</code>.</p>
    #[doc(hidden)]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl PathPatternConditionConfig {
    /// <p>The path patterns to compare against the request URL. The maximum size of each string is 128 characters. The comparison is case sensitive. The following wildcard characters are supported: * (matches 0 or more characters) and ? (matches exactly 1 character).</p>
    /// <p>If you specify multiple strings, the condition is satisfied if one of them matches the request URL. The path pattern is compared only to the path of the URL, not to its query string. To compare against the query string, use <code>QueryStringConditionConfig</code>.</p>
    pub fn values(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.values.as_deref()
    }
}
impl PathPatternConditionConfig {
    /// Creates a new builder-style object to manufacture [`PathPatternConditionConfig`](crate::types::PathPatternConditionConfig).
    pub fn builder() -> crate::types::builders::PathPatternConditionConfigBuilder {
        crate::types::builders::PathPatternConditionConfigBuilder::default()
    }
}

/// A builder for [`PathPatternConditionConfig`](crate::types::PathPatternConditionConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PathPatternConditionConfigBuilder {
    pub(crate) values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl PathPatternConditionConfigBuilder {
    /// Appends an item to `values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p>The path patterns to compare against the request URL. The maximum size of each string is 128 characters. The comparison is case sensitive. The following wildcard characters are supported: * (matches 0 or more characters) and ? (matches exactly 1 character).</p>
    /// <p>If you specify multiple strings, the condition is satisfied if one of them matches the request URL. The path pattern is compared only to the path of the URL, not to its query string. To compare against the query string, use <code>QueryStringConditionConfig</code>.</p>
    pub fn values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.values.unwrap_or_default();
        v.push(input.into());
        self.values = ::std::option::Option::Some(v);
        self
    }
    /// <p>The path patterns to compare against the request URL. The maximum size of each string is 128 characters. The comparison is case sensitive. The following wildcard characters are supported: * (matches 0 or more characters) and ? (matches exactly 1 character).</p>
    /// <p>If you specify multiple strings, the condition is satisfied if one of them matches the request URL. The path pattern is compared only to the path of the URL, not to its query string. To compare against the query string, use <code>QueryStringConditionConfig</code>.</p>
    pub fn set_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.values = input;
        self
    }
    /// Consumes the builder and constructs a [`PathPatternConditionConfig`](crate::types::PathPatternConditionConfig).
    pub fn build(self) -> crate::types::PathPatternConditionConfig {
        crate::types::PathPatternConditionConfig {
            values: self.values,
        }
    }
}
