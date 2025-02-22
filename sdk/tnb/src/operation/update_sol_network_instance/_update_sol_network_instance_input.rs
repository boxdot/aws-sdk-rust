// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct UpdateSolNetworkInstanceInput {
    /// <p>ID of the network instance.</p>
    #[doc(hidden)]
    pub ns_instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The type of update.</p>
    #[doc(hidden)]
    pub update_type: ::std::option::Option<crate::types::UpdateSolNetworkType>,
    /// <p>Identifies the network function information parameters and/or the configurable properties of the network function to be modified.</p>
    #[doc(hidden)]
    pub modify_vnf_info_data: ::std::option::Option<crate::types::UpdateSolNetworkModify>,
    /// <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. When you use this API, the tags are transferred to the network operation that is created. Use tags to search and filter your resources or track your Amazon Web Services costs.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl UpdateSolNetworkInstanceInput {
    /// <p>ID of the network instance.</p>
    pub fn ns_instance_id(&self) -> ::std::option::Option<&str> {
        self.ns_instance_id.as_deref()
    }
    /// <p>The type of update.</p>
    pub fn update_type(&self) -> ::std::option::Option<&crate::types::UpdateSolNetworkType> {
        self.update_type.as_ref()
    }
    /// <p>Identifies the network function information parameters and/or the configurable properties of the network function to be modified.</p>
    pub fn modify_vnf_info_data(
        &self,
    ) -> ::std::option::Option<&crate::types::UpdateSolNetworkModify> {
        self.modify_vnf_info_data.as_ref()
    }
    /// <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. When you use this API, the tags are transferred to the network operation that is created. Use tags to search and filter your resources or track your Amazon Web Services costs.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl ::std::fmt::Debug for UpdateSolNetworkInstanceInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateSolNetworkInstanceInput");
        formatter.field("ns_instance_id", &self.ns_instance_id);
        formatter.field("update_type", &self.update_type);
        formatter.field("modify_vnf_info_data", &self.modify_vnf_info_data);
        formatter.field("tags", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl UpdateSolNetworkInstanceInput {
    /// Creates a new builder-style object to manufacture [`UpdateSolNetworkInstanceInput`](crate::operation::update_sol_network_instance::UpdateSolNetworkInstanceInput).
    pub fn builder(
    ) -> crate::operation::update_sol_network_instance::builders::UpdateSolNetworkInstanceInputBuilder
    {
        crate::operation::update_sol_network_instance::builders::UpdateSolNetworkInstanceInputBuilder::default()
    }
}

/// A builder for [`UpdateSolNetworkInstanceInput`](crate::operation::update_sol_network_instance::UpdateSolNetworkInstanceInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct UpdateSolNetworkInstanceInputBuilder {
    pub(crate) ns_instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) update_type: ::std::option::Option<crate::types::UpdateSolNetworkType>,
    pub(crate) modify_vnf_info_data: ::std::option::Option<crate::types::UpdateSolNetworkModify>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl UpdateSolNetworkInstanceInputBuilder {
    /// <p>ID of the network instance.</p>
    pub fn ns_instance_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.ns_instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ID of the network instance.</p>
    pub fn set_ns_instance_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.ns_instance_id = input;
        self
    }
    /// <p>The type of update.</p>
    pub fn update_type(mut self, input: crate::types::UpdateSolNetworkType) -> Self {
        self.update_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of update.</p>
    pub fn set_update_type(
        mut self,
        input: ::std::option::Option<crate::types::UpdateSolNetworkType>,
    ) -> Self {
        self.update_type = input;
        self
    }
    /// <p>Identifies the network function information parameters and/or the configurable properties of the network function to be modified.</p>
    pub fn modify_vnf_info_data(mut self, input: crate::types::UpdateSolNetworkModify) -> Self {
        self.modify_vnf_info_data = ::std::option::Option::Some(input);
        self
    }
    /// <p>Identifies the network function information parameters and/or the configurable properties of the network function to be modified.</p>
    pub fn set_modify_vnf_info_data(
        mut self,
        input: ::std::option::Option<crate::types::UpdateSolNetworkModify>,
    ) -> Self {
        self.modify_vnf_info_data = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. When you use this API, the tags are transferred to the network operation that is created. Use tags to search and filter your resources or track your Amazon Web Services costs.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. When you use this API, the tags are transferred to the network operation that is created. Use tags to search and filter your resources or track your Amazon Web Services costs.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateSolNetworkInstanceInput`](crate::operation::update_sol_network_instance::UpdateSolNetworkInstanceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_sol_network_instance::UpdateSolNetworkInstanceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_sol_network_instance::UpdateSolNetworkInstanceInput {
                ns_instance_id: self.ns_instance_id,
                update_type: self.update_type,
                modify_vnf_info_data: self.modify_vnf_info_data,
                tags: self.tags,
            },
        )
    }
}
impl ::std::fmt::Debug for UpdateSolNetworkInstanceInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateSolNetworkInstanceInputBuilder");
        formatter.field("ns_instance_id", &self.ns_instance_id);
        formatter.field("update_type", &self.update_type);
        formatter.field("modify_vnf_info_data", &self.modify_vnf_info_data);
        formatter.field("tags", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
