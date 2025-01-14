// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteModelPackageGroupPolicyInput {
    /// <p>The name of the model group for which to delete the policy.</p>
    #[doc(hidden)]
    pub model_package_group_name: ::std::option::Option<::std::string::String>,
}
impl DeleteModelPackageGroupPolicyInput {
    /// <p>The name of the model group for which to delete the policy.</p>
    pub fn model_package_group_name(&self) -> ::std::option::Option<&str> {
        self.model_package_group_name.as_deref()
    }
}
impl DeleteModelPackageGroupPolicyInput {
    /// Creates a new builder-style object to manufacture [`DeleteModelPackageGroupPolicyInput`](crate::operation::delete_model_package_group_policy::DeleteModelPackageGroupPolicyInput).
    pub fn builder() -> crate::operation::delete_model_package_group_policy::builders::DeleteModelPackageGroupPolicyInputBuilder{
        crate::operation::delete_model_package_group_policy::builders::DeleteModelPackageGroupPolicyInputBuilder::default()
    }
}

/// A builder for [`DeleteModelPackageGroupPolicyInput`](crate::operation::delete_model_package_group_policy::DeleteModelPackageGroupPolicyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteModelPackageGroupPolicyInputBuilder {
    pub(crate) model_package_group_name: ::std::option::Option<::std::string::String>,
}
impl DeleteModelPackageGroupPolicyInputBuilder {
    /// <p>The name of the model group for which to delete the policy.</p>
    pub fn model_package_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.model_package_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the model group for which to delete the policy.</p>
    pub fn set_model_package_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.model_package_group_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteModelPackageGroupPolicyInput`](crate::operation::delete_model_package_group_policy::DeleteModelPackageGroupPolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_model_package_group_policy::DeleteModelPackageGroupPolicyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_model_package_group_policy::DeleteModelPackageGroupPolicyInput {
                model_package_group_name: self.model_package_group_name
                ,
            }
        )
    }
}
