// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains an array of <code>Transition</code> objects specifying how long in days before a recovery point transitions to cold storage or is deleted.</p>
/// <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, on the console, the “retention” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold.</p>
/// <p>Resource types that are able to be transitioned to cold storage are listed in the "Lifecycle to cold storage" section of the <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/whatisbackup.html#features-by-resource"> Feature availability by resource</a> table. Backup ignores this expression for other resource types.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Lifecycle {
    /// <p>Specifies the number of days after creation that a recovery point is moved to cold storage.</p>
    #[doc(hidden)]
    pub move_to_cold_storage_after_days: ::std::option::Option<i64>,
    /// <p>Specifies the number of days after creation that a recovery point is deleted. Must be greater than 90 days plus <code>MoveToColdStorageAfterDays</code>.</p>
    #[doc(hidden)]
    pub delete_after_days: ::std::option::Option<i64>,
}
impl Lifecycle {
    /// <p>Specifies the number of days after creation that a recovery point is moved to cold storage.</p>
    pub fn move_to_cold_storage_after_days(&self) -> ::std::option::Option<i64> {
        self.move_to_cold_storage_after_days
    }
    /// <p>Specifies the number of days after creation that a recovery point is deleted. Must be greater than 90 days plus <code>MoveToColdStorageAfterDays</code>.</p>
    pub fn delete_after_days(&self) -> ::std::option::Option<i64> {
        self.delete_after_days
    }
}
impl Lifecycle {
    /// Creates a new builder-style object to manufacture [`Lifecycle`](crate::types::Lifecycle).
    pub fn builder() -> crate::types::builders::LifecycleBuilder {
        crate::types::builders::LifecycleBuilder::default()
    }
}

/// A builder for [`Lifecycle`](crate::types::Lifecycle).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LifecycleBuilder {
    pub(crate) move_to_cold_storage_after_days: ::std::option::Option<i64>,
    pub(crate) delete_after_days: ::std::option::Option<i64>,
}
impl LifecycleBuilder {
    /// <p>Specifies the number of days after creation that a recovery point is moved to cold storage.</p>
    pub fn move_to_cold_storage_after_days(mut self, input: i64) -> Self {
        self.move_to_cold_storage_after_days = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the number of days after creation that a recovery point is moved to cold storage.</p>
    pub fn set_move_to_cold_storage_after_days(
        mut self,
        input: ::std::option::Option<i64>,
    ) -> Self {
        self.move_to_cold_storage_after_days = input;
        self
    }
    /// <p>Specifies the number of days after creation that a recovery point is deleted. Must be greater than 90 days plus <code>MoveToColdStorageAfterDays</code>.</p>
    pub fn delete_after_days(mut self, input: i64) -> Self {
        self.delete_after_days = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the number of days after creation that a recovery point is deleted. Must be greater than 90 days plus <code>MoveToColdStorageAfterDays</code>.</p>
    pub fn set_delete_after_days(mut self, input: ::std::option::Option<i64>) -> Self {
        self.delete_after_days = input;
        self
    }
    /// Consumes the builder and constructs a [`Lifecycle`](crate::types::Lifecycle).
    pub fn build(self) -> crate::types::Lifecycle {
        crate::types::Lifecycle {
            move_to_cold_storage_after_days: self.move_to_cold_storage_after_days,
            delete_after_days: self.delete_after_days,
        }
    }
}
