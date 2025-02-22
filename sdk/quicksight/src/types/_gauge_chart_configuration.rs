// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration of a <code>GaugeChartVisual</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GaugeChartConfiguration {
    /// <p>The field well configuration of a <code>GaugeChartVisual</code>.</p>
    #[doc(hidden)]
    pub field_wells: ::std::option::Option<crate::types::GaugeChartFieldWells>,
    /// <p>The options that determine the presentation of the <code>GaugeChartVisual</code>.</p>
    #[doc(hidden)]
    pub gauge_chart_options: ::std::option::Option<crate::types::GaugeChartOptions>,
    /// <p>The data label configuration of a <code>GaugeChartVisual</code>.</p>
    #[doc(hidden)]
    pub data_labels: ::std::option::Option<crate::types::DataLabelOptions>,
    /// <p>The tooltip configuration of a <code>GaugeChartVisual</code>.</p>
    #[doc(hidden)]
    pub tooltip_options: ::std::option::Option<crate::types::TooltipOptions>,
    /// <p>The visual palette configuration of a <code>GaugeChartVisual</code>.</p>
    #[doc(hidden)]
    pub visual_palette: ::std::option::Option<crate::types::VisualPalette>,
}
impl GaugeChartConfiguration {
    /// <p>The field well configuration of a <code>GaugeChartVisual</code>.</p>
    pub fn field_wells(&self) -> ::std::option::Option<&crate::types::GaugeChartFieldWells> {
        self.field_wells.as_ref()
    }
    /// <p>The options that determine the presentation of the <code>GaugeChartVisual</code>.</p>
    pub fn gauge_chart_options(&self) -> ::std::option::Option<&crate::types::GaugeChartOptions> {
        self.gauge_chart_options.as_ref()
    }
    /// <p>The data label configuration of a <code>GaugeChartVisual</code>.</p>
    pub fn data_labels(&self) -> ::std::option::Option<&crate::types::DataLabelOptions> {
        self.data_labels.as_ref()
    }
    /// <p>The tooltip configuration of a <code>GaugeChartVisual</code>.</p>
    pub fn tooltip_options(&self) -> ::std::option::Option<&crate::types::TooltipOptions> {
        self.tooltip_options.as_ref()
    }
    /// <p>The visual palette configuration of a <code>GaugeChartVisual</code>.</p>
    pub fn visual_palette(&self) -> ::std::option::Option<&crate::types::VisualPalette> {
        self.visual_palette.as_ref()
    }
}
impl GaugeChartConfiguration {
    /// Creates a new builder-style object to manufacture [`GaugeChartConfiguration`](crate::types::GaugeChartConfiguration).
    pub fn builder() -> crate::types::builders::GaugeChartConfigurationBuilder {
        crate::types::builders::GaugeChartConfigurationBuilder::default()
    }
}

/// A builder for [`GaugeChartConfiguration`](crate::types::GaugeChartConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GaugeChartConfigurationBuilder {
    pub(crate) field_wells: ::std::option::Option<crate::types::GaugeChartFieldWells>,
    pub(crate) gauge_chart_options: ::std::option::Option<crate::types::GaugeChartOptions>,
    pub(crate) data_labels: ::std::option::Option<crate::types::DataLabelOptions>,
    pub(crate) tooltip_options: ::std::option::Option<crate::types::TooltipOptions>,
    pub(crate) visual_palette: ::std::option::Option<crate::types::VisualPalette>,
}
impl GaugeChartConfigurationBuilder {
    /// <p>The field well configuration of a <code>GaugeChartVisual</code>.</p>
    pub fn field_wells(mut self, input: crate::types::GaugeChartFieldWells) -> Self {
        self.field_wells = ::std::option::Option::Some(input);
        self
    }
    /// <p>The field well configuration of a <code>GaugeChartVisual</code>.</p>
    pub fn set_field_wells(
        mut self,
        input: ::std::option::Option<crate::types::GaugeChartFieldWells>,
    ) -> Self {
        self.field_wells = input;
        self
    }
    /// <p>The options that determine the presentation of the <code>GaugeChartVisual</code>.</p>
    pub fn gauge_chart_options(mut self, input: crate::types::GaugeChartOptions) -> Self {
        self.gauge_chart_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The options that determine the presentation of the <code>GaugeChartVisual</code>.</p>
    pub fn set_gauge_chart_options(
        mut self,
        input: ::std::option::Option<crate::types::GaugeChartOptions>,
    ) -> Self {
        self.gauge_chart_options = input;
        self
    }
    /// <p>The data label configuration of a <code>GaugeChartVisual</code>.</p>
    pub fn data_labels(mut self, input: crate::types::DataLabelOptions) -> Self {
        self.data_labels = ::std::option::Option::Some(input);
        self
    }
    /// <p>The data label configuration of a <code>GaugeChartVisual</code>.</p>
    pub fn set_data_labels(
        mut self,
        input: ::std::option::Option<crate::types::DataLabelOptions>,
    ) -> Self {
        self.data_labels = input;
        self
    }
    /// <p>The tooltip configuration of a <code>GaugeChartVisual</code>.</p>
    pub fn tooltip_options(mut self, input: crate::types::TooltipOptions) -> Self {
        self.tooltip_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The tooltip configuration of a <code>GaugeChartVisual</code>.</p>
    pub fn set_tooltip_options(
        mut self,
        input: ::std::option::Option<crate::types::TooltipOptions>,
    ) -> Self {
        self.tooltip_options = input;
        self
    }
    /// <p>The visual palette configuration of a <code>GaugeChartVisual</code>.</p>
    pub fn visual_palette(mut self, input: crate::types::VisualPalette) -> Self {
        self.visual_palette = ::std::option::Option::Some(input);
        self
    }
    /// <p>The visual palette configuration of a <code>GaugeChartVisual</code>.</p>
    pub fn set_visual_palette(
        mut self,
        input: ::std::option::Option<crate::types::VisualPalette>,
    ) -> Self {
        self.visual_palette = input;
        self
    }
    /// Consumes the builder and constructs a [`GaugeChartConfiguration`](crate::types::GaugeChartConfiguration).
    pub fn build(self) -> crate::types::GaugeChartConfiguration {
        crate::types::GaugeChartConfiguration {
            field_wells: self.field_wells,
            gauge_chart_options: self.gauge_chart_options,
            data_labels: self.data_labels,
            tooltip_options: self.tooltip_options,
            visual_palette: self.visual_palette,
        }
    }
}
