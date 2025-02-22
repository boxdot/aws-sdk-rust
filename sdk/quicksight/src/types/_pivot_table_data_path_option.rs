// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The data path options for the pivot table field options.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PivotTableDataPathOption {
    /// <p>The list of data path values for the data path options.</p>
    #[doc(hidden)]
    pub data_path_list: ::std::option::Option<::std::vec::Vec<crate::types::DataPathValue>>,
    /// <p>The width of the data path option.</p>
    #[doc(hidden)]
    pub width: ::std::option::Option<::std::string::String>,
}
impl PivotTableDataPathOption {
    /// <p>The list of data path values for the data path options.</p>
    pub fn data_path_list(&self) -> ::std::option::Option<&[crate::types::DataPathValue]> {
        self.data_path_list.as_deref()
    }
    /// <p>The width of the data path option.</p>
    pub fn width(&self) -> ::std::option::Option<&str> {
        self.width.as_deref()
    }
}
impl PivotTableDataPathOption {
    /// Creates a new builder-style object to manufacture [`PivotTableDataPathOption`](crate::types::PivotTableDataPathOption).
    pub fn builder() -> crate::types::builders::PivotTableDataPathOptionBuilder {
        crate::types::builders::PivotTableDataPathOptionBuilder::default()
    }
}

/// A builder for [`PivotTableDataPathOption`](crate::types::PivotTableDataPathOption).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PivotTableDataPathOptionBuilder {
    pub(crate) data_path_list: ::std::option::Option<::std::vec::Vec<crate::types::DataPathValue>>,
    pub(crate) width: ::std::option::Option<::std::string::String>,
}
impl PivotTableDataPathOptionBuilder {
    /// Appends an item to `data_path_list`.
    ///
    /// To override the contents of this collection use [`set_data_path_list`](Self::set_data_path_list).
    ///
    /// <p>The list of data path values for the data path options.</p>
    pub fn data_path_list(mut self, input: crate::types::DataPathValue) -> Self {
        let mut v = self.data_path_list.unwrap_or_default();
        v.push(input);
        self.data_path_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of data path values for the data path options.</p>
    pub fn set_data_path_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DataPathValue>>,
    ) -> Self {
        self.data_path_list = input;
        self
    }
    /// <p>The width of the data path option.</p>
    pub fn width(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.width = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The width of the data path option.</p>
    pub fn set_width(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.width = input;
        self
    }
    /// Consumes the builder and constructs a [`PivotTableDataPathOption`](crate::types::PivotTableDataPathOption).
    pub fn build(self) -> crate::types::PivotTableDataPathOption {
        crate::types::PivotTableDataPathOption {
            data_path_list: self.data_path_list,
            width: self.width,
        }
    }
}
