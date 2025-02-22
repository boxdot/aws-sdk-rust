// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the metadata of a device offering.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Offering {
    /// <p>The ID that corresponds to a device offering.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>A string that describes the offering.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The type of offering (for example, <code>RECURRING</code>) for a device.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::OfferingType>,
    /// <p>The platform of the device (for example, <code>ANDROID</code> or <code>IOS</code>).</p>
    #[doc(hidden)]
    pub platform: ::std::option::Option<crate::types::DevicePlatform>,
    /// <p>Specifies whether there are recurring charges for the offering.</p>
    #[doc(hidden)]
    pub recurring_charges: ::std::option::Option<::std::vec::Vec<crate::types::RecurringCharge>>,
}
impl Offering {
    /// <p>The ID that corresponds to a device offering.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>A string that describes the offering.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The type of offering (for example, <code>RECURRING</code>) for a device.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::OfferingType> {
        self.r#type.as_ref()
    }
    /// <p>The platform of the device (for example, <code>ANDROID</code> or <code>IOS</code>).</p>
    pub fn platform(&self) -> ::std::option::Option<&crate::types::DevicePlatform> {
        self.platform.as_ref()
    }
    /// <p>Specifies whether there are recurring charges for the offering.</p>
    pub fn recurring_charges(&self) -> ::std::option::Option<&[crate::types::RecurringCharge]> {
        self.recurring_charges.as_deref()
    }
}
impl Offering {
    /// Creates a new builder-style object to manufacture [`Offering`](crate::types::Offering).
    pub fn builder() -> crate::types::builders::OfferingBuilder {
        crate::types::builders::OfferingBuilder::default()
    }
}

/// A builder for [`Offering`](crate::types::Offering).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OfferingBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::OfferingType>,
    pub(crate) platform: ::std::option::Option<crate::types::DevicePlatform>,
    pub(crate) recurring_charges:
        ::std::option::Option<::std::vec::Vec<crate::types::RecurringCharge>>,
}
impl OfferingBuilder {
    /// <p>The ID that corresponds to a device offering.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID that corresponds to a device offering.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>A string that describes the offering.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string that describes the offering.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The type of offering (for example, <code>RECURRING</code>) for a device.</p>
    pub fn r#type(mut self, input: crate::types::OfferingType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of offering (for example, <code>RECURRING</code>) for a device.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::OfferingType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The platform of the device (for example, <code>ANDROID</code> or <code>IOS</code>).</p>
    pub fn platform(mut self, input: crate::types::DevicePlatform) -> Self {
        self.platform = ::std::option::Option::Some(input);
        self
    }
    /// <p>The platform of the device (for example, <code>ANDROID</code> or <code>IOS</code>).</p>
    pub fn set_platform(
        mut self,
        input: ::std::option::Option<crate::types::DevicePlatform>,
    ) -> Self {
        self.platform = input;
        self
    }
    /// Appends an item to `recurring_charges`.
    ///
    /// To override the contents of this collection use [`set_recurring_charges`](Self::set_recurring_charges).
    ///
    /// <p>Specifies whether there are recurring charges for the offering.</p>
    pub fn recurring_charges(mut self, input: crate::types::RecurringCharge) -> Self {
        let mut v = self.recurring_charges.unwrap_or_default();
        v.push(input);
        self.recurring_charges = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies whether there are recurring charges for the offering.</p>
    pub fn set_recurring_charges(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RecurringCharge>>,
    ) -> Self {
        self.recurring_charges = input;
        self
    }
    /// Consumes the builder and constructs a [`Offering`](crate::types::Offering).
    pub fn build(self) -> crate::types::Offering {
        crate::types::Offering {
            id: self.id,
            description: self.description,
            r#type: self.r#type,
            platform: self.platform,
            recurring_charges: self.recurring_charges,
        }
    }
}
