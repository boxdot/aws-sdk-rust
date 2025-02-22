// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the suite definition.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SuiteDefinitionInformation {
    /// <p>Suite definition ID of the test suite.</p>
    #[doc(hidden)]
    pub suite_definition_id: ::std::option::Option<::std::string::String>,
    /// <p>Suite name of the test suite.</p>
    #[doc(hidden)]
    pub suite_definition_name: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the devices that are under test for the test suite.</p>
    #[doc(hidden)]
    pub default_devices: ::std::option::Option<::std::vec::Vec<crate::types::DeviceUnderTest>>,
    /// <p>Specifies if the test suite is intended for qualification.</p>
    #[doc(hidden)]
    pub intended_for_qualification: bool,
    /// <p>Verifies if the test suite is a long duration test.</p>
    #[doc(hidden)]
    pub is_long_duration_test: bool,
    /// <p>Gets the MQTT protocol that is configured in the suite definition.</p>
    #[doc(hidden)]
    pub protocol: ::std::option::Option<crate::types::Protocol>,
    /// <p>Date (in Unix epoch time) when the test suite was created.</p>
    #[doc(hidden)]
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl SuiteDefinitionInformation {
    /// <p>Suite definition ID of the test suite.</p>
    pub fn suite_definition_id(&self) -> ::std::option::Option<&str> {
        self.suite_definition_id.as_deref()
    }
    /// <p>Suite name of the test suite.</p>
    pub fn suite_definition_name(&self) -> ::std::option::Option<&str> {
        self.suite_definition_name.as_deref()
    }
    /// <p>Specifies the devices that are under test for the test suite.</p>
    pub fn default_devices(&self) -> ::std::option::Option<&[crate::types::DeviceUnderTest]> {
        self.default_devices.as_deref()
    }
    /// <p>Specifies if the test suite is intended for qualification.</p>
    pub fn intended_for_qualification(&self) -> bool {
        self.intended_for_qualification
    }
    /// <p>Verifies if the test suite is a long duration test.</p>
    pub fn is_long_duration_test(&self) -> bool {
        self.is_long_duration_test
    }
    /// <p>Gets the MQTT protocol that is configured in the suite definition.</p>
    pub fn protocol(&self) -> ::std::option::Option<&crate::types::Protocol> {
        self.protocol.as_ref()
    }
    /// <p>Date (in Unix epoch time) when the test suite was created.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
}
impl SuiteDefinitionInformation {
    /// Creates a new builder-style object to manufacture [`SuiteDefinitionInformation`](crate::types::SuiteDefinitionInformation).
    pub fn builder() -> crate::types::builders::SuiteDefinitionInformationBuilder {
        crate::types::builders::SuiteDefinitionInformationBuilder::default()
    }
}

/// A builder for [`SuiteDefinitionInformation`](crate::types::SuiteDefinitionInformation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SuiteDefinitionInformationBuilder {
    pub(crate) suite_definition_id: ::std::option::Option<::std::string::String>,
    pub(crate) suite_definition_name: ::std::option::Option<::std::string::String>,
    pub(crate) default_devices:
        ::std::option::Option<::std::vec::Vec<crate::types::DeviceUnderTest>>,
    pub(crate) intended_for_qualification: ::std::option::Option<bool>,
    pub(crate) is_long_duration_test: ::std::option::Option<bool>,
    pub(crate) protocol: ::std::option::Option<crate::types::Protocol>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl SuiteDefinitionInformationBuilder {
    /// <p>Suite definition ID of the test suite.</p>
    pub fn suite_definition_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.suite_definition_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Suite definition ID of the test suite.</p>
    pub fn set_suite_definition_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.suite_definition_id = input;
        self
    }
    /// <p>Suite name of the test suite.</p>
    pub fn suite_definition_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.suite_definition_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Suite name of the test suite.</p>
    pub fn set_suite_definition_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.suite_definition_name = input;
        self
    }
    /// Appends an item to `default_devices`.
    ///
    /// To override the contents of this collection use [`set_default_devices`](Self::set_default_devices).
    ///
    /// <p>Specifies the devices that are under test for the test suite.</p>
    pub fn default_devices(mut self, input: crate::types::DeviceUnderTest) -> Self {
        let mut v = self.default_devices.unwrap_or_default();
        v.push(input);
        self.default_devices = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies the devices that are under test for the test suite.</p>
    pub fn set_default_devices(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DeviceUnderTest>>,
    ) -> Self {
        self.default_devices = input;
        self
    }
    /// <p>Specifies if the test suite is intended for qualification.</p>
    pub fn intended_for_qualification(mut self, input: bool) -> Self {
        self.intended_for_qualification = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies if the test suite is intended for qualification.</p>
    pub fn set_intended_for_qualification(mut self, input: ::std::option::Option<bool>) -> Self {
        self.intended_for_qualification = input;
        self
    }
    /// <p>Verifies if the test suite is a long duration test.</p>
    pub fn is_long_duration_test(mut self, input: bool) -> Self {
        self.is_long_duration_test = ::std::option::Option::Some(input);
        self
    }
    /// <p>Verifies if the test suite is a long duration test.</p>
    pub fn set_is_long_duration_test(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_long_duration_test = input;
        self
    }
    /// <p>Gets the MQTT protocol that is configured in the suite definition.</p>
    pub fn protocol(mut self, input: crate::types::Protocol) -> Self {
        self.protocol = ::std::option::Option::Some(input);
        self
    }
    /// <p>Gets the MQTT protocol that is configured in the suite definition.</p>
    pub fn set_protocol(mut self, input: ::std::option::Option<crate::types::Protocol>) -> Self {
        self.protocol = input;
        self
    }
    /// <p>Date (in Unix epoch time) when the test suite was created.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>Date (in Unix epoch time) when the test suite was created.</p>
    pub fn set_created_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_at = input;
        self
    }
    /// Consumes the builder and constructs a [`SuiteDefinitionInformation`](crate::types::SuiteDefinitionInformation).
    pub fn build(self) -> crate::types::SuiteDefinitionInformation {
        crate::types::SuiteDefinitionInformation {
            suite_definition_id: self.suite_definition_id,
            suite_definition_name: self.suite_definition_name,
            default_devices: self.default_devices,
            intended_for_qualification: self.intended_for_qualification.unwrap_or_default(),
            is_long_duration_test: self.is_long_duration_test.unwrap_or_default(),
            protocol: self.protocol,
            created_at: self.created_at,
        }
    }
}
