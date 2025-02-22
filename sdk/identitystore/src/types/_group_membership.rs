// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the identifiers for a group, a group member, and a <code>GroupMembership</code> object in the identity store.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GroupMembership {
    /// <p>The globally unique identifier for the identity store.</p>
    #[doc(hidden)]
    pub identity_store_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier for a <code>GroupMembership</code> object in an identity store.</p>
    #[doc(hidden)]
    pub membership_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier for a group in the identity store.</p>
    #[doc(hidden)]
    pub group_id: ::std::option::Option<::std::string::String>,
    /// <p>An object that contains the identifier of a group member. Setting the <code>UserID</code> field to the specific identifier for a user indicates that the user is a member of the group.</p>
    #[doc(hidden)]
    pub member_id: ::std::option::Option<crate::types::MemberId>,
}
impl GroupMembership {
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn identity_store_id(&self) -> ::std::option::Option<&str> {
        self.identity_store_id.as_deref()
    }
    /// <p>The identifier for a <code>GroupMembership</code> object in an identity store.</p>
    pub fn membership_id(&self) -> ::std::option::Option<&str> {
        self.membership_id.as_deref()
    }
    /// <p>The identifier for a group in the identity store.</p>
    pub fn group_id(&self) -> ::std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>An object that contains the identifier of a group member. Setting the <code>UserID</code> field to the specific identifier for a user indicates that the user is a member of the group.</p>
    pub fn member_id(&self) -> ::std::option::Option<&crate::types::MemberId> {
        self.member_id.as_ref()
    }
}
impl GroupMembership {
    /// Creates a new builder-style object to manufacture [`GroupMembership`](crate::types::GroupMembership).
    pub fn builder() -> crate::types::builders::GroupMembershipBuilder {
        crate::types::builders::GroupMembershipBuilder::default()
    }
}

/// A builder for [`GroupMembership`](crate::types::GroupMembership).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GroupMembershipBuilder {
    pub(crate) identity_store_id: ::std::option::Option<::std::string::String>,
    pub(crate) membership_id: ::std::option::Option<::std::string::String>,
    pub(crate) group_id: ::std::option::Option<::std::string::String>,
    pub(crate) member_id: ::std::option::Option<crate::types::MemberId>,
}
impl GroupMembershipBuilder {
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn identity_store_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.identity_store_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn set_identity_store_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.identity_store_id = input;
        self
    }
    /// <p>The identifier for a <code>GroupMembership</code> object in an identity store.</p>
    pub fn membership_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.membership_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for a <code>GroupMembership</code> object in an identity store.</p>
    pub fn set_membership_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.membership_id = input;
        self
    }
    /// <p>The identifier for a group in the identity store.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for a group in the identity store.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_id = input;
        self
    }
    /// <p>An object that contains the identifier of a group member. Setting the <code>UserID</code> field to the specific identifier for a user indicates that the user is a member of the group.</p>
    pub fn member_id(mut self, input: crate::types::MemberId) -> Self {
        self.member_id = ::std::option::Option::Some(input);
        self
    }
    /// <p>An object that contains the identifier of a group member. Setting the <code>UserID</code> field to the specific identifier for a user indicates that the user is a member of the group.</p>
    pub fn set_member_id(mut self, input: ::std::option::Option<crate::types::MemberId>) -> Self {
        self.member_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GroupMembership`](crate::types::GroupMembership).
    pub fn build(self) -> crate::types::GroupMembership {
        crate::types::GroupMembership {
            identity_store_id: self.identity_store_id,
            membership_id: self.membership_id,
            group_id: self.group_id,
            member_id: self.member_id,
        }
    }
}
