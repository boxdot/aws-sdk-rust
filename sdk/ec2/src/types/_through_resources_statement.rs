// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a through resource statement.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ThroughResourcesStatement {
    /// <p>The resource statement.</p>
    #[doc(hidden)]
    pub resource_statement: ::std::option::Option<crate::types::ResourceStatement>,
}
impl ThroughResourcesStatement {
    /// <p>The resource statement.</p>
    pub fn resource_statement(&self) -> ::std::option::Option<&crate::types::ResourceStatement> {
        self.resource_statement.as_ref()
    }
}
impl ThroughResourcesStatement {
    /// Creates a new builder-style object to manufacture [`ThroughResourcesStatement`](crate::types::ThroughResourcesStatement).
    pub fn builder() -> crate::types::builders::ThroughResourcesStatementBuilder {
        crate::types::builders::ThroughResourcesStatementBuilder::default()
    }
}

/// A builder for [`ThroughResourcesStatement`](crate::types::ThroughResourcesStatement).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ThroughResourcesStatementBuilder {
    pub(crate) resource_statement: ::std::option::Option<crate::types::ResourceStatement>,
}
impl ThroughResourcesStatementBuilder {
    /// <p>The resource statement.</p>
    pub fn resource_statement(mut self, input: crate::types::ResourceStatement) -> Self {
        self.resource_statement = ::std::option::Option::Some(input);
        self
    }
    /// <p>The resource statement.</p>
    pub fn set_resource_statement(
        mut self,
        input: ::std::option::Option<crate::types::ResourceStatement>,
    ) -> Self {
        self.resource_statement = input;
        self
    }
    /// Consumes the builder and constructs a [`ThroughResourcesStatement`](crate::types::ThroughResourcesStatement).
    pub fn build(self) -> crate::types::ThroughResourcesStatement {
        crate::types::ThroughResourcesStatement {
            resource_statement: self.resource_statement,
        }
    }
}
