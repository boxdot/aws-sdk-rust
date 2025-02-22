// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The aggregated field well for the table.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TableAggregatedFieldWells {
    /// <p>The group by field well for a pivot table. Values are grouped by group by fields.</p>
    #[doc(hidden)]
    pub group_by: ::std::option::Option<::std::vec::Vec<crate::types::DimensionField>>,
    /// <p>The values field well for a pivot table. Values are aggregated based on group by fields.</p>
    #[doc(hidden)]
    pub values: ::std::option::Option<::std::vec::Vec<crate::types::MeasureField>>,
}
impl TableAggregatedFieldWells {
    /// <p>The group by field well for a pivot table. Values are grouped by group by fields.</p>
    pub fn group_by(&self) -> ::std::option::Option<&[crate::types::DimensionField]> {
        self.group_by.as_deref()
    }
    /// <p>The values field well for a pivot table. Values are aggregated based on group by fields.</p>
    pub fn values(&self) -> ::std::option::Option<&[crate::types::MeasureField]> {
        self.values.as_deref()
    }
}
impl TableAggregatedFieldWells {
    /// Creates a new builder-style object to manufacture [`TableAggregatedFieldWells`](crate::types::TableAggregatedFieldWells).
    pub fn builder() -> crate::types::builders::TableAggregatedFieldWellsBuilder {
        crate::types::builders::TableAggregatedFieldWellsBuilder::default()
    }
}

/// A builder for [`TableAggregatedFieldWells`](crate::types::TableAggregatedFieldWells).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TableAggregatedFieldWellsBuilder {
    pub(crate) group_by: ::std::option::Option<::std::vec::Vec<crate::types::DimensionField>>,
    pub(crate) values: ::std::option::Option<::std::vec::Vec<crate::types::MeasureField>>,
}
impl TableAggregatedFieldWellsBuilder {
    /// Appends an item to `group_by`.
    ///
    /// To override the contents of this collection use [`set_group_by`](Self::set_group_by).
    ///
    /// <p>The group by field well for a pivot table. Values are grouped by group by fields.</p>
    pub fn group_by(mut self, input: crate::types::DimensionField) -> Self {
        let mut v = self.group_by.unwrap_or_default();
        v.push(input);
        self.group_by = ::std::option::Option::Some(v);
        self
    }
    /// <p>The group by field well for a pivot table. Values are grouped by group by fields.</p>
    pub fn set_group_by(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DimensionField>>,
    ) -> Self {
        self.group_by = input;
        self
    }
    /// Appends an item to `values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p>The values field well for a pivot table. Values are aggregated based on group by fields.</p>
    pub fn values(mut self, input: crate::types::MeasureField) -> Self {
        let mut v = self.values.unwrap_or_default();
        v.push(input);
        self.values = ::std::option::Option::Some(v);
        self
    }
    /// <p>The values field well for a pivot table. Values are aggregated based on group by fields.</p>
    pub fn set_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MeasureField>>,
    ) -> Self {
        self.values = input;
        self
    }
    /// Consumes the builder and constructs a [`TableAggregatedFieldWells`](crate::types::TableAggregatedFieldWells).
    pub fn build(self) -> crate::types::TableAggregatedFieldWells {
        crate::types::TableAggregatedFieldWells {
            group_by: self.group_by,
            values: self.values,
        }
    }
}
