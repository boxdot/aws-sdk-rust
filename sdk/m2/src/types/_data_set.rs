// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Defines a data set.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DataSet {
    /// <p>The storage type of the data set: database or file system. For Micro Focus, database corresponds to datastore and file system corresponds to EFS/FSX. For Blu Age, there is no support of file system and database corresponds to Blusam. </p>
    #[doc(hidden)]
    pub storage_type: ::std::option::Option<::std::string::String>,
    /// <p>The logical identifier for a specific data set (in mainframe format).</p>
    #[doc(hidden)]
    pub dataset_name: ::std::option::Option<::std::string::String>,
    /// <p>The type of dataset. The only supported value is VSAM.</p>
    #[doc(hidden)]
    pub dataset_org: ::std::option::Option<crate::types::DatasetOrgAttributes>,
    /// <p>The relative location of the data set in the database or file system. </p>
    #[doc(hidden)]
    pub relative_path: ::std::option::Option<::std::string::String>,
    /// <p>The length of a record.</p>
    #[doc(hidden)]
    pub record_length: ::std::option::Option<crate::types::RecordLength>,
}
impl DataSet {
    /// <p>The storage type of the data set: database or file system. For Micro Focus, database corresponds to datastore and file system corresponds to EFS/FSX. For Blu Age, there is no support of file system and database corresponds to Blusam. </p>
    pub fn storage_type(&self) -> ::std::option::Option<&str> {
        self.storage_type.as_deref()
    }
    /// <p>The logical identifier for a specific data set (in mainframe format).</p>
    pub fn dataset_name(&self) -> ::std::option::Option<&str> {
        self.dataset_name.as_deref()
    }
    /// <p>The type of dataset. The only supported value is VSAM.</p>
    pub fn dataset_org(&self) -> ::std::option::Option<&crate::types::DatasetOrgAttributes> {
        self.dataset_org.as_ref()
    }
    /// <p>The relative location of the data set in the database or file system. </p>
    pub fn relative_path(&self) -> ::std::option::Option<&str> {
        self.relative_path.as_deref()
    }
    /// <p>The length of a record.</p>
    pub fn record_length(&self) -> ::std::option::Option<&crate::types::RecordLength> {
        self.record_length.as_ref()
    }
}
impl DataSet {
    /// Creates a new builder-style object to manufacture [`DataSet`](crate::types::DataSet).
    pub fn builder() -> crate::types::builders::DataSetBuilder {
        crate::types::builders::DataSetBuilder::default()
    }
}

/// A builder for [`DataSet`](crate::types::DataSet).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DataSetBuilder {
    pub(crate) storage_type: ::std::option::Option<::std::string::String>,
    pub(crate) dataset_name: ::std::option::Option<::std::string::String>,
    pub(crate) dataset_org: ::std::option::Option<crate::types::DatasetOrgAttributes>,
    pub(crate) relative_path: ::std::option::Option<::std::string::String>,
    pub(crate) record_length: ::std::option::Option<crate::types::RecordLength>,
}
impl DataSetBuilder {
    /// <p>The storage type of the data set: database or file system. For Micro Focus, database corresponds to datastore and file system corresponds to EFS/FSX. For Blu Age, there is no support of file system and database corresponds to Blusam. </p>
    pub fn storage_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.storage_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The storage type of the data set: database or file system. For Micro Focus, database corresponds to datastore and file system corresponds to EFS/FSX. For Blu Age, there is no support of file system and database corresponds to Blusam. </p>
    pub fn set_storage_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.storage_type = input;
        self
    }
    /// <p>The logical identifier for a specific data set (in mainframe format).</p>
    pub fn dataset_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dataset_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The logical identifier for a specific data set (in mainframe format).</p>
    pub fn set_dataset_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dataset_name = input;
        self
    }
    /// <p>The type of dataset. The only supported value is VSAM.</p>
    pub fn dataset_org(mut self, input: crate::types::DatasetOrgAttributes) -> Self {
        self.dataset_org = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of dataset. The only supported value is VSAM.</p>
    pub fn set_dataset_org(
        mut self,
        input: ::std::option::Option<crate::types::DatasetOrgAttributes>,
    ) -> Self {
        self.dataset_org = input;
        self
    }
    /// <p>The relative location of the data set in the database or file system. </p>
    pub fn relative_path(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.relative_path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The relative location of the data set in the database or file system. </p>
    pub fn set_relative_path(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.relative_path = input;
        self
    }
    /// <p>The length of a record.</p>
    pub fn record_length(mut self, input: crate::types::RecordLength) -> Self {
        self.record_length = ::std::option::Option::Some(input);
        self
    }
    /// <p>The length of a record.</p>
    pub fn set_record_length(
        mut self,
        input: ::std::option::Option<crate::types::RecordLength>,
    ) -> Self {
        self.record_length = input;
        self
    }
    /// Consumes the builder and constructs a [`DataSet`](crate::types::DataSet).
    pub fn build(self) -> crate::types::DataSet {
        crate::types::DataSet {
            storage_type: self.storage_type,
            dataset_name: self.dataset_name,
            dataset_org: self.dataset_org,
            relative_path: self.relative_path,
            record_length: self.record_length,
        }
    }
}
