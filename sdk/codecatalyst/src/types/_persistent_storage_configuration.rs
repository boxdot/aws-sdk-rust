// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the configuration of persistent storage for a Dev Environment. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PersistentStorageConfiguration {
    /// <p>The size of the persistent storage in gigabytes (specifically GiB).</p> <note>
    /// <p>Valid values for storage are based on memory sizes in 16GB increments. Valid values are 16, 32, and 64.</p>
    /// </note>
    #[doc(hidden)]
    pub size_in_gi_b: ::std::option::Option<i32>,
}
impl PersistentStorageConfiguration {
    /// <p>The size of the persistent storage in gigabytes (specifically GiB).</p> <note>
    /// <p>Valid values for storage are based on memory sizes in 16GB increments. Valid values are 16, 32, and 64.</p>
    /// </note>
    pub fn size_in_gi_b(&self) -> ::std::option::Option<i32> {
        self.size_in_gi_b
    }
}
impl PersistentStorageConfiguration {
    /// Creates a new builder-style object to manufacture [`PersistentStorageConfiguration`](crate::types::PersistentStorageConfiguration).
    pub fn builder() -> crate::types::builders::PersistentStorageConfigurationBuilder {
        crate::types::builders::PersistentStorageConfigurationBuilder::default()
    }
}

/// A builder for [`PersistentStorageConfiguration`](crate::types::PersistentStorageConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PersistentStorageConfigurationBuilder {
    pub(crate) size_in_gi_b: ::std::option::Option<i32>,
}
impl PersistentStorageConfigurationBuilder {
    /// <p>The size of the persistent storage in gigabytes (specifically GiB).</p> <note>
    /// <p>Valid values for storage are based on memory sizes in 16GB increments. Valid values are 16, 32, and 64.</p>
    /// </note>
    pub fn size_in_gi_b(mut self, input: i32) -> Self {
        self.size_in_gi_b = ::std::option::Option::Some(input);
        self
    }
    /// <p>The size of the persistent storage in gigabytes (specifically GiB).</p> <note>
    /// <p>Valid values for storage are based on memory sizes in 16GB increments. Valid values are 16, 32, and 64.</p>
    /// </note>
    pub fn set_size_in_gi_b(mut self, input: ::std::option::Option<i32>) -> Self {
        self.size_in_gi_b = input;
        self
    }
    /// Consumes the builder and constructs a [`PersistentStorageConfiguration`](crate::types::PersistentStorageConfiguration).
    pub fn build(self) -> crate::types::PersistentStorageConfiguration {
        crate::types::PersistentStorageConfiguration {
            size_in_gi_b: self.size_in_gi_b,
        }
    }
}
