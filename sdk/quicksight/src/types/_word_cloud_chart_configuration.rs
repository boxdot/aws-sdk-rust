// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration of a word cloud visual.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct WordCloudChartConfiguration {
    /// <p>The field wells of the visual.</p>
    #[doc(hidden)]
    pub field_wells: ::std::option::Option<crate::types::WordCloudFieldWells>,
    /// <p>The sort configuration of a word cloud visual.</p>
    #[doc(hidden)]
    pub sort_configuration: ::std::option::Option<crate::types::WordCloudSortConfiguration>,
    /// <p>The label options (label text, label visibility, and sort icon visibility) for the word cloud category.</p>
    #[doc(hidden)]
    pub category_label_options: ::std::option::Option<crate::types::ChartAxisLabelOptions>,
    /// <p>The options for a word cloud visual.</p>
    #[doc(hidden)]
    pub word_cloud_options: ::std::option::Option<crate::types::WordCloudOptions>,
}
impl WordCloudChartConfiguration {
    /// <p>The field wells of the visual.</p>
    pub fn field_wells(&self) -> ::std::option::Option<&crate::types::WordCloudFieldWells> {
        self.field_wells.as_ref()
    }
    /// <p>The sort configuration of a word cloud visual.</p>
    pub fn sort_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::WordCloudSortConfiguration> {
        self.sort_configuration.as_ref()
    }
    /// <p>The label options (label text, label visibility, and sort icon visibility) for the word cloud category.</p>
    pub fn category_label_options(
        &self,
    ) -> ::std::option::Option<&crate::types::ChartAxisLabelOptions> {
        self.category_label_options.as_ref()
    }
    /// <p>The options for a word cloud visual.</p>
    pub fn word_cloud_options(&self) -> ::std::option::Option<&crate::types::WordCloudOptions> {
        self.word_cloud_options.as_ref()
    }
}
impl WordCloudChartConfiguration {
    /// Creates a new builder-style object to manufacture [`WordCloudChartConfiguration`](crate::types::WordCloudChartConfiguration).
    pub fn builder() -> crate::types::builders::WordCloudChartConfigurationBuilder {
        crate::types::builders::WordCloudChartConfigurationBuilder::default()
    }
}

/// A builder for [`WordCloudChartConfiguration`](crate::types::WordCloudChartConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct WordCloudChartConfigurationBuilder {
    pub(crate) field_wells: ::std::option::Option<crate::types::WordCloudFieldWells>,
    pub(crate) sort_configuration: ::std::option::Option<crate::types::WordCloudSortConfiguration>,
    pub(crate) category_label_options: ::std::option::Option<crate::types::ChartAxisLabelOptions>,
    pub(crate) word_cloud_options: ::std::option::Option<crate::types::WordCloudOptions>,
}
impl WordCloudChartConfigurationBuilder {
    /// <p>The field wells of the visual.</p>
    pub fn field_wells(mut self, input: crate::types::WordCloudFieldWells) -> Self {
        self.field_wells = ::std::option::Option::Some(input);
        self
    }
    /// <p>The field wells of the visual.</p>
    pub fn set_field_wells(
        mut self,
        input: ::std::option::Option<crate::types::WordCloudFieldWells>,
    ) -> Self {
        self.field_wells = input;
        self
    }
    /// <p>The sort configuration of a word cloud visual.</p>
    pub fn sort_configuration(mut self, input: crate::types::WordCloudSortConfiguration) -> Self {
        self.sort_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The sort configuration of a word cloud visual.</p>
    pub fn set_sort_configuration(
        mut self,
        input: ::std::option::Option<crate::types::WordCloudSortConfiguration>,
    ) -> Self {
        self.sort_configuration = input;
        self
    }
    /// <p>The label options (label text, label visibility, and sort icon visibility) for the word cloud category.</p>
    pub fn category_label_options(mut self, input: crate::types::ChartAxisLabelOptions) -> Self {
        self.category_label_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The label options (label text, label visibility, and sort icon visibility) for the word cloud category.</p>
    pub fn set_category_label_options(
        mut self,
        input: ::std::option::Option<crate::types::ChartAxisLabelOptions>,
    ) -> Self {
        self.category_label_options = input;
        self
    }
    /// <p>The options for a word cloud visual.</p>
    pub fn word_cloud_options(mut self, input: crate::types::WordCloudOptions) -> Self {
        self.word_cloud_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The options for a word cloud visual.</p>
    pub fn set_word_cloud_options(
        mut self,
        input: ::std::option::Option<crate::types::WordCloudOptions>,
    ) -> Self {
        self.word_cloud_options = input;
        self
    }
    /// Consumes the builder and constructs a [`WordCloudChartConfiguration`](crate::types::WordCloudChartConfiguration).
    pub fn build(self) -> crate::types::WordCloudChartConfiguration {
        crate::types::WordCloudChartConfiguration {
            field_wells: self.field_wells,
            sort_configuration: self.sort_configuration,
            category_label_options: self.category_label_options,
            word_cloud_options: self.word_cloud_options,
        }
    }
}
