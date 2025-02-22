// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the authentication methods used by a Client VPN endpoint. For more information, see <a href="https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/client-authentication.html">Authentication</a> in the <i>Client VPN Administrator Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ClientVpnAuthentication {
    /// <p>The authentication type used.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::ClientVpnAuthenticationType>,
    /// <p>Information about the Active Directory, if applicable.</p>
    #[doc(hidden)]
    pub active_directory: ::std::option::Option<crate::types::DirectoryServiceAuthentication>,
    /// <p>Information about the authentication certificates, if applicable.</p>
    #[doc(hidden)]
    pub mutual_authentication: ::std::option::Option<crate::types::CertificateAuthentication>,
    /// <p>Information about the IAM SAML identity provider, if applicable.</p>
    #[doc(hidden)]
    pub federated_authentication: ::std::option::Option<crate::types::FederatedAuthentication>,
}
impl ClientVpnAuthentication {
    /// <p>The authentication type used.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::ClientVpnAuthenticationType> {
        self.r#type.as_ref()
    }
    /// <p>Information about the Active Directory, if applicable.</p>
    pub fn active_directory(
        &self,
    ) -> ::std::option::Option<&crate::types::DirectoryServiceAuthentication> {
        self.active_directory.as_ref()
    }
    /// <p>Information about the authentication certificates, if applicable.</p>
    pub fn mutual_authentication(
        &self,
    ) -> ::std::option::Option<&crate::types::CertificateAuthentication> {
        self.mutual_authentication.as_ref()
    }
    /// <p>Information about the IAM SAML identity provider, if applicable.</p>
    pub fn federated_authentication(
        &self,
    ) -> ::std::option::Option<&crate::types::FederatedAuthentication> {
        self.federated_authentication.as_ref()
    }
}
impl ClientVpnAuthentication {
    /// Creates a new builder-style object to manufacture [`ClientVpnAuthentication`](crate::types::ClientVpnAuthentication).
    pub fn builder() -> crate::types::builders::ClientVpnAuthenticationBuilder {
        crate::types::builders::ClientVpnAuthenticationBuilder::default()
    }
}

/// A builder for [`ClientVpnAuthentication`](crate::types::ClientVpnAuthentication).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ClientVpnAuthenticationBuilder {
    pub(crate) r#type: ::std::option::Option<crate::types::ClientVpnAuthenticationType>,
    pub(crate) active_directory:
        ::std::option::Option<crate::types::DirectoryServiceAuthentication>,
    pub(crate) mutual_authentication:
        ::std::option::Option<crate::types::CertificateAuthentication>,
    pub(crate) federated_authentication:
        ::std::option::Option<crate::types::FederatedAuthentication>,
}
impl ClientVpnAuthenticationBuilder {
    /// <p>The authentication type used.</p>
    pub fn r#type(mut self, input: crate::types::ClientVpnAuthenticationType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The authentication type used.</p>
    pub fn set_type(
        mut self,
        input: ::std::option::Option<crate::types::ClientVpnAuthenticationType>,
    ) -> Self {
        self.r#type = input;
        self
    }
    /// <p>Information about the Active Directory, if applicable.</p>
    pub fn active_directory(mut self, input: crate::types::DirectoryServiceAuthentication) -> Self {
        self.active_directory = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the Active Directory, if applicable.</p>
    pub fn set_active_directory(
        mut self,
        input: ::std::option::Option<crate::types::DirectoryServiceAuthentication>,
    ) -> Self {
        self.active_directory = input;
        self
    }
    /// <p>Information about the authentication certificates, if applicable.</p>
    pub fn mutual_authentication(mut self, input: crate::types::CertificateAuthentication) -> Self {
        self.mutual_authentication = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the authentication certificates, if applicable.</p>
    pub fn set_mutual_authentication(
        mut self,
        input: ::std::option::Option<crate::types::CertificateAuthentication>,
    ) -> Self {
        self.mutual_authentication = input;
        self
    }
    /// <p>Information about the IAM SAML identity provider, if applicable.</p>
    pub fn federated_authentication(
        mut self,
        input: crate::types::FederatedAuthentication,
    ) -> Self {
        self.federated_authentication = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the IAM SAML identity provider, if applicable.</p>
    pub fn set_federated_authentication(
        mut self,
        input: ::std::option::Option<crate::types::FederatedAuthentication>,
    ) -> Self {
        self.federated_authentication = input;
        self
    }
    /// Consumes the builder and constructs a [`ClientVpnAuthentication`](crate::types::ClientVpnAuthentication).
    pub fn build(self) -> crate::types::ClientVpnAuthentication {
        crate::types::ClientVpnAuthentication {
            r#type: self.r#type,
            active_directory: self.active_directory,
            mutual_authentication: self.mutual_authentication,
            federated_authentication: self.federated_authentication,
        }
    }
}
