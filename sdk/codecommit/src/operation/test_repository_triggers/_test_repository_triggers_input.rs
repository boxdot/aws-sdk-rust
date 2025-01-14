// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the input of a test repository triggers operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TestRepositoryTriggersInput {
    /// <p>The name of the repository in which to test the triggers.</p>
    #[doc(hidden)]
    pub repository_name: ::std::option::Option<::std::string::String>,
    /// <p>The list of triggers to test.</p>
    #[doc(hidden)]
    pub triggers: ::std::option::Option<::std::vec::Vec<crate::types::RepositoryTrigger>>,
}
impl TestRepositoryTriggersInput {
    /// <p>The name of the repository in which to test the triggers.</p>
    pub fn repository_name(&self) -> ::std::option::Option<&str> {
        self.repository_name.as_deref()
    }
    /// <p>The list of triggers to test.</p>
    pub fn triggers(&self) -> ::std::option::Option<&[crate::types::RepositoryTrigger]> {
        self.triggers.as_deref()
    }
}
impl TestRepositoryTriggersInput {
    /// Creates a new builder-style object to manufacture [`TestRepositoryTriggersInput`](crate::operation::test_repository_triggers::TestRepositoryTriggersInput).
    pub fn builder(
    ) -> crate::operation::test_repository_triggers::builders::TestRepositoryTriggersInputBuilder
    {
        crate::operation::test_repository_triggers::builders::TestRepositoryTriggersInputBuilder::default()
    }
}

/// A builder for [`TestRepositoryTriggersInput`](crate::operation::test_repository_triggers::TestRepositoryTriggersInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TestRepositoryTriggersInputBuilder {
    pub(crate) repository_name: ::std::option::Option<::std::string::String>,
    pub(crate) triggers: ::std::option::Option<::std::vec::Vec<crate::types::RepositoryTrigger>>,
}
impl TestRepositoryTriggersInputBuilder {
    /// <p>The name of the repository in which to test the triggers.</p>
    pub fn repository_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.repository_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the repository in which to test the triggers.</p>
    pub fn set_repository_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.repository_name = input;
        self
    }
    /// Appends an item to `triggers`.
    ///
    /// To override the contents of this collection use [`set_triggers`](Self::set_triggers).
    ///
    /// <p>The list of triggers to test.</p>
    pub fn triggers(mut self, input: crate::types::RepositoryTrigger) -> Self {
        let mut v = self.triggers.unwrap_or_default();
        v.push(input);
        self.triggers = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of triggers to test.</p>
    pub fn set_triggers(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RepositoryTrigger>>,
    ) -> Self {
        self.triggers = input;
        self
    }
    /// Consumes the builder and constructs a [`TestRepositoryTriggersInput`](crate::operation::test_repository_triggers::TestRepositoryTriggersInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::test_repository_triggers::TestRepositoryTriggersInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::test_repository_triggers::TestRepositoryTriggersInput {
                repository_name: self.repository_name,
                triggers: self.triggers,
            },
        )
    }
}
