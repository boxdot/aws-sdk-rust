// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Object that describes the frequency.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Frequency {
    /// <p>Frequency value. Valid values are between 2200 to 2300 MHz and 7750 to 8400 MHz for downlink and 2025 to 2120 MHz for uplink.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<f64>,
    /// <p>Frequency units.</p>
    #[doc(hidden)]
    pub units: ::std::option::Option<crate::types::FrequencyUnits>,
}
impl Frequency {
    /// <p>Frequency value. Valid values are between 2200 to 2300 MHz and 7750 to 8400 MHz for downlink and 2025 to 2120 MHz for uplink.</p>
    pub fn value(&self) -> ::std::option::Option<f64> {
        self.value
    }
    /// <p>Frequency units.</p>
    pub fn units(&self) -> ::std::option::Option<&crate::types::FrequencyUnits> {
        self.units.as_ref()
    }
}
impl Frequency {
    /// Creates a new builder-style object to manufacture [`Frequency`](crate::types::Frequency).
    pub fn builder() -> crate::types::builders::FrequencyBuilder {
        crate::types::builders::FrequencyBuilder::default()
    }
}

/// A builder for [`Frequency`](crate::types::Frequency).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FrequencyBuilder {
    pub(crate) value: ::std::option::Option<f64>,
    pub(crate) units: ::std::option::Option<crate::types::FrequencyUnits>,
}
impl FrequencyBuilder {
    /// <p>Frequency value. Valid values are between 2200 to 2300 MHz and 7750 to 8400 MHz for downlink and 2025 to 2120 MHz for uplink.</p>
    pub fn value(mut self, input: f64) -> Self {
        self.value = ::std::option::Option::Some(input);
        self
    }
    /// <p>Frequency value. Valid values are between 2200 to 2300 MHz and 7750 to 8400 MHz for downlink and 2025 to 2120 MHz for uplink.</p>
    pub fn set_value(mut self, input: ::std::option::Option<f64>) -> Self {
        self.value = input;
        self
    }
    /// <p>Frequency units.</p>
    pub fn units(mut self, input: crate::types::FrequencyUnits) -> Self {
        self.units = ::std::option::Option::Some(input);
        self
    }
    /// <p>Frequency units.</p>
    pub fn set_units(mut self, input: ::std::option::Option<crate::types::FrequencyUnits>) -> Self {
        self.units = input;
        self
    }
    /// Consumes the builder and constructs a [`Frequency`](crate::types::Frequency).
    pub fn build(self) -> crate::types::Frequency {
        crate::types::Frequency {
            value: self.value,
            units: self.units,
        }
    }
}
