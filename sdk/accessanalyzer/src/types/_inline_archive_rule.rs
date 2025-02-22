// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An criterion statement in an archive rule. Each archive rule may have multiple criteria.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InlineArchiveRule {
    /// <p>The name of the rule.</p>
    #[doc(hidden)]
    pub rule_name: ::std::option::Option<::std::string::String>,
    /// <p>The condition and values for a criterion.</p>
    #[doc(hidden)]
    pub filter: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::Criterion>,
    >,
}
impl InlineArchiveRule {
    /// <p>The name of the rule.</p>
    pub fn rule_name(&self) -> ::std::option::Option<&str> {
        self.rule_name.as_deref()
    }
    /// <p>The condition and values for a criterion.</p>
    pub fn filter(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, crate::types::Criterion>,
    > {
        self.filter.as_ref()
    }
}
impl InlineArchiveRule {
    /// Creates a new builder-style object to manufacture [`InlineArchiveRule`](crate::types::InlineArchiveRule).
    pub fn builder() -> crate::types::builders::InlineArchiveRuleBuilder {
        crate::types::builders::InlineArchiveRuleBuilder::default()
    }
}

/// A builder for [`InlineArchiveRule`](crate::types::InlineArchiveRule).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InlineArchiveRuleBuilder {
    pub(crate) rule_name: ::std::option::Option<::std::string::String>,
    pub(crate) filter: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::Criterion>,
    >,
}
impl InlineArchiveRuleBuilder {
    /// <p>The name of the rule.</p>
    pub fn rule_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.rule_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the rule.</p>
    pub fn set_rule_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.rule_name = input;
        self
    }
    /// Adds a key-value pair to `filter`.
    ///
    /// To override the contents of this collection use [`set_filter`](Self::set_filter).
    ///
    /// <p>The condition and values for a criterion.</p>
    pub fn filter(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::Criterion,
    ) -> Self {
        let mut hash_map = self.filter.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.filter = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The condition and values for a criterion.</p>
    pub fn set_filter(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::Criterion>,
        >,
    ) -> Self {
        self.filter = input;
        self
    }
    /// Consumes the builder and constructs a [`InlineArchiveRule`](crate::types::InlineArchiveRule).
    pub fn build(self) -> crate::types::InlineArchiveRule {
        crate::types::InlineArchiveRule {
            rule_name: self.rule_name,
            filter: self.filter,
        }
    }
}
