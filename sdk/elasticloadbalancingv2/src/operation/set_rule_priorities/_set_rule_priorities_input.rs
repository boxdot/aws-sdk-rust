// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SetRulePrioritiesInput {
    /// <p>The rule priorities.</p>
    #[doc(hidden)]
    pub rule_priorities: ::std::option::Option<::std::vec::Vec<crate::types::RulePriorityPair>>,
}
impl SetRulePrioritiesInput {
    /// <p>The rule priorities.</p>
    pub fn rule_priorities(&self) -> ::std::option::Option<&[crate::types::RulePriorityPair]> {
        self.rule_priorities.as_deref()
    }
}
impl SetRulePrioritiesInput {
    /// Creates a new builder-style object to manufacture [`SetRulePrioritiesInput`](crate::operation::set_rule_priorities::SetRulePrioritiesInput).
    pub fn builder(
    ) -> crate::operation::set_rule_priorities::builders::SetRulePrioritiesInputBuilder {
        crate::operation::set_rule_priorities::builders::SetRulePrioritiesInputBuilder::default()
    }
}

/// A builder for [`SetRulePrioritiesInput`](crate::operation::set_rule_priorities::SetRulePrioritiesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SetRulePrioritiesInputBuilder {
    pub(crate) rule_priorities:
        ::std::option::Option<::std::vec::Vec<crate::types::RulePriorityPair>>,
}
impl SetRulePrioritiesInputBuilder {
    /// Appends an item to `rule_priorities`.
    ///
    /// To override the contents of this collection use [`set_rule_priorities`](Self::set_rule_priorities).
    ///
    /// <p>The rule priorities.</p>
    pub fn rule_priorities(mut self, input: crate::types::RulePriorityPair) -> Self {
        let mut v = self.rule_priorities.unwrap_or_default();
        v.push(input);
        self.rule_priorities = ::std::option::Option::Some(v);
        self
    }
    /// <p>The rule priorities.</p>
    pub fn set_rule_priorities(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RulePriorityPair>>,
    ) -> Self {
        self.rule_priorities = input;
        self
    }
    /// Consumes the builder and constructs a [`SetRulePrioritiesInput`](crate::operation::set_rule_priorities::SetRulePrioritiesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::set_rule_priorities::SetRulePrioritiesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::set_rule_priorities::SetRulePrioritiesInput {
                rule_priorities: self.rule_priorities,
            },
        )
    }
}
