// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeRecoveryPointInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[doc(hidden)]
    pub backup_vault_name: ::std::option::Option<::std::string::String>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[doc(hidden)]
    pub recovery_point_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeRecoveryPointInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    pub fn backup_vault_name(&self) -> ::std::option::Option<&str> {
        self.backup_vault_name.as_deref()
    }
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    pub fn recovery_point_arn(&self) -> ::std::option::Option<&str> {
        self.recovery_point_arn.as_deref()
    }
}
impl DescribeRecoveryPointInput {
    /// Creates a new builder-style object to manufacture [`DescribeRecoveryPointInput`](crate::operation::describe_recovery_point::DescribeRecoveryPointInput).
    pub fn builder(
    ) -> crate::operation::describe_recovery_point::builders::DescribeRecoveryPointInputBuilder
    {
        crate::operation::describe_recovery_point::builders::DescribeRecoveryPointInputBuilder::default()
    }
}

/// A builder for [`DescribeRecoveryPointInput`](crate::operation::describe_recovery_point::DescribeRecoveryPointInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeRecoveryPointInputBuilder {
    pub(crate) backup_vault_name: ::std::option::Option<::std::string::String>,
    pub(crate) recovery_point_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeRecoveryPointInputBuilder {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    pub fn backup_vault_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.backup_vault_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    pub fn set_backup_vault_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.backup_vault_name = input;
        self
    }
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    pub fn recovery_point_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.recovery_point_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    pub fn set_recovery_point_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.recovery_point_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeRecoveryPointInput`](crate::operation::describe_recovery_point::DescribeRecoveryPointInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_recovery_point::DescribeRecoveryPointInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_recovery_point::DescribeRecoveryPointInput {
                backup_vault_name: self.backup_vault_name,
                recovery_point_arn: self.recovery_point_arn,
            },
        )
    }
}
