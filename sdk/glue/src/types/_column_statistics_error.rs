// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Encapsulates a <code>ColumnStatistics</code> object that failed and the reason for failure.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ColumnStatisticsError {
    /// <p>The <code>ColumnStatistics</code> of the column.</p>
    #[doc(hidden)]
    pub column_statistics: ::std::option::Option<crate::types::ColumnStatistics>,
    /// <p>An error message with the reason for the failure of an operation.</p>
    #[doc(hidden)]
    pub error: ::std::option::Option<crate::types::ErrorDetail>,
}
impl ColumnStatisticsError {
    /// <p>The <code>ColumnStatistics</code> of the column.</p>
    pub fn column_statistics(&self) -> ::std::option::Option<&crate::types::ColumnStatistics> {
        self.column_statistics.as_ref()
    }
    /// <p>An error message with the reason for the failure of an operation.</p>
    pub fn error(&self) -> ::std::option::Option<&crate::types::ErrorDetail> {
        self.error.as_ref()
    }
}
impl ColumnStatisticsError {
    /// Creates a new builder-style object to manufacture [`ColumnStatisticsError`](crate::types::ColumnStatisticsError).
    pub fn builder() -> crate::types::builders::ColumnStatisticsErrorBuilder {
        crate::types::builders::ColumnStatisticsErrorBuilder::default()
    }
}

/// A builder for [`ColumnStatisticsError`](crate::types::ColumnStatisticsError).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ColumnStatisticsErrorBuilder {
    pub(crate) column_statistics: ::std::option::Option<crate::types::ColumnStatistics>,
    pub(crate) error: ::std::option::Option<crate::types::ErrorDetail>,
}
impl ColumnStatisticsErrorBuilder {
    /// <p>The <code>ColumnStatistics</code> of the column.</p>
    pub fn column_statistics(mut self, input: crate::types::ColumnStatistics) -> Self {
        self.column_statistics = ::std::option::Option::Some(input);
        self
    }
    /// <p>The <code>ColumnStatistics</code> of the column.</p>
    pub fn set_column_statistics(
        mut self,
        input: ::std::option::Option<crate::types::ColumnStatistics>,
    ) -> Self {
        self.column_statistics = input;
        self
    }
    /// <p>An error message with the reason for the failure of an operation.</p>
    pub fn error(mut self, input: crate::types::ErrorDetail) -> Self {
        self.error = ::std::option::Option::Some(input);
        self
    }
    /// <p>An error message with the reason for the failure of an operation.</p>
    pub fn set_error(mut self, input: ::std::option::Option<crate::types::ErrorDetail>) -> Self {
        self.error = input;
        self
    }
    /// Consumes the builder and constructs a [`ColumnStatisticsError`](crate::types::ColumnStatisticsError).
    pub fn build(self) -> crate::types::ColumnStatisticsError {
        crate::types::ColumnStatisticsError {
            column_statistics: self.column_statistics,
            error: self.error,
        }
    }
}
