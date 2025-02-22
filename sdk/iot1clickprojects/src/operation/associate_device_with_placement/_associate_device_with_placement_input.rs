// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateDeviceWithPlacementInput {
    /// <p>The name of the project containing the placement in which to associate the device.</p>
    #[doc(hidden)]
    pub project_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the placement in which to associate the device.</p>
    #[doc(hidden)]
    pub placement_name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the physical device to be associated with the given placement in the project. Note that a mandatory 4 character prefix is required for all <code>deviceId</code> values.</p>
    #[doc(hidden)]
    pub device_id: ::std::option::Option<::std::string::String>,
    /// <p>The device template name to associate with the device ID.</p>
    #[doc(hidden)]
    pub device_template_name: ::std::option::Option<::std::string::String>,
}
impl AssociateDeviceWithPlacementInput {
    /// <p>The name of the project containing the placement in which to associate the device.</p>
    pub fn project_name(&self) -> ::std::option::Option<&str> {
        self.project_name.as_deref()
    }
    /// <p>The name of the placement in which to associate the device.</p>
    pub fn placement_name(&self) -> ::std::option::Option<&str> {
        self.placement_name.as_deref()
    }
    /// <p>The ID of the physical device to be associated with the given placement in the project. Note that a mandatory 4 character prefix is required for all <code>deviceId</code> values.</p>
    pub fn device_id(&self) -> ::std::option::Option<&str> {
        self.device_id.as_deref()
    }
    /// <p>The device template name to associate with the device ID.</p>
    pub fn device_template_name(&self) -> ::std::option::Option<&str> {
        self.device_template_name.as_deref()
    }
}
impl AssociateDeviceWithPlacementInput {
    /// Creates a new builder-style object to manufacture [`AssociateDeviceWithPlacementInput`](crate::operation::associate_device_with_placement::AssociateDeviceWithPlacementInput).
    pub fn builder() -> crate::operation::associate_device_with_placement::builders::AssociateDeviceWithPlacementInputBuilder{
        crate::operation::associate_device_with_placement::builders::AssociateDeviceWithPlacementInputBuilder::default()
    }
}

/// A builder for [`AssociateDeviceWithPlacementInput`](crate::operation::associate_device_with_placement::AssociateDeviceWithPlacementInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AssociateDeviceWithPlacementInputBuilder {
    pub(crate) project_name: ::std::option::Option<::std::string::String>,
    pub(crate) placement_name: ::std::option::Option<::std::string::String>,
    pub(crate) device_id: ::std::option::Option<::std::string::String>,
    pub(crate) device_template_name: ::std::option::Option<::std::string::String>,
}
impl AssociateDeviceWithPlacementInputBuilder {
    /// <p>The name of the project containing the placement in which to associate the device.</p>
    pub fn project_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the project containing the placement in which to associate the device.</p>
    pub fn set_project_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project_name = input;
        self
    }
    /// <p>The name of the placement in which to associate the device.</p>
    pub fn placement_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.placement_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the placement in which to associate the device.</p>
    pub fn set_placement_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.placement_name = input;
        self
    }
    /// <p>The ID of the physical device to be associated with the given placement in the project. Note that a mandatory 4 character prefix is required for all <code>deviceId</code> values.</p>
    pub fn device_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.device_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the physical device to be associated with the given placement in the project. Note that a mandatory 4 character prefix is required for all <code>deviceId</code> values.</p>
    pub fn set_device_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.device_id = input;
        self
    }
    /// <p>The device template name to associate with the device ID.</p>
    pub fn device_template_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.device_template_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The device template name to associate with the device ID.</p>
    pub fn set_device_template_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.device_template_name = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociateDeviceWithPlacementInput`](crate::operation::associate_device_with_placement::AssociateDeviceWithPlacementInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::associate_device_with_placement::AssociateDeviceWithPlacementInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::associate_device_with_placement::AssociateDeviceWithPlacementInput {
                project_name: self.project_name,
                placement_name: self.placement_name,
                device_id: self.device_id,
                device_template_name: self.device_template_name,
            },
        )
    }
}
