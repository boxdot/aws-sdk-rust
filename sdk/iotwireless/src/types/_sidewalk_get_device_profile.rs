// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Gets information about a Sidewalk device profile.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct SidewalkGetDeviceProfile {
    /// <p>The Sidewalk application server public key.</p>
    #[doc(hidden)]
    pub application_server_public_key: ::std::option::Option<::std::string::String>,
    /// <p>Gets information about the certification status of a Sidewalk device profile.</p>
    #[doc(hidden)]
    pub qualification_status: ::std::option::Option<bool>,
    /// <p>The DAK certificate information of the Sidewalk device profile.</p>
    #[doc(hidden)]
    pub dak_certificate_metadata:
        ::std::option::Option<::std::vec::Vec<crate::types::DakCertificateMetadata>>,
}
impl SidewalkGetDeviceProfile {
    /// <p>The Sidewalk application server public key.</p>
    pub fn application_server_public_key(&self) -> ::std::option::Option<&str> {
        self.application_server_public_key.as_deref()
    }
    /// <p>Gets information about the certification status of a Sidewalk device profile.</p>
    pub fn qualification_status(&self) -> ::std::option::Option<bool> {
        self.qualification_status
    }
    /// <p>The DAK certificate information of the Sidewalk device profile.</p>
    pub fn dak_certificate_metadata(
        &self,
    ) -> ::std::option::Option<&[crate::types::DakCertificateMetadata]> {
        self.dak_certificate_metadata.as_deref()
    }
}
impl ::std::fmt::Debug for SidewalkGetDeviceProfile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SidewalkGetDeviceProfile");
        formatter.field(
            "application_server_public_key",
            &"*** Sensitive Data Redacted ***",
        );
        formatter.field("qualification_status", &self.qualification_status);
        formatter.field("dak_certificate_metadata", &self.dak_certificate_metadata);
        formatter.finish()
    }
}
impl SidewalkGetDeviceProfile {
    /// Creates a new builder-style object to manufacture [`SidewalkGetDeviceProfile`](crate::types::SidewalkGetDeviceProfile).
    pub fn builder() -> crate::types::builders::SidewalkGetDeviceProfileBuilder {
        crate::types::builders::SidewalkGetDeviceProfileBuilder::default()
    }
}

/// A builder for [`SidewalkGetDeviceProfile`](crate::types::SidewalkGetDeviceProfile).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct SidewalkGetDeviceProfileBuilder {
    pub(crate) application_server_public_key: ::std::option::Option<::std::string::String>,
    pub(crate) qualification_status: ::std::option::Option<bool>,
    pub(crate) dak_certificate_metadata:
        ::std::option::Option<::std::vec::Vec<crate::types::DakCertificateMetadata>>,
}
impl SidewalkGetDeviceProfileBuilder {
    /// <p>The Sidewalk application server public key.</p>
    pub fn application_server_public_key(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_server_public_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Sidewalk application server public key.</p>
    pub fn set_application_server_public_key(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_server_public_key = input;
        self
    }
    /// <p>Gets information about the certification status of a Sidewalk device profile.</p>
    pub fn qualification_status(mut self, input: bool) -> Self {
        self.qualification_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Gets information about the certification status of a Sidewalk device profile.</p>
    pub fn set_qualification_status(mut self, input: ::std::option::Option<bool>) -> Self {
        self.qualification_status = input;
        self
    }
    /// Appends an item to `dak_certificate_metadata`.
    ///
    /// To override the contents of this collection use [`set_dak_certificate_metadata`](Self::set_dak_certificate_metadata).
    ///
    /// <p>The DAK certificate information of the Sidewalk device profile.</p>
    pub fn dak_certificate_metadata(mut self, input: crate::types::DakCertificateMetadata) -> Self {
        let mut v = self.dak_certificate_metadata.unwrap_or_default();
        v.push(input);
        self.dak_certificate_metadata = ::std::option::Option::Some(v);
        self
    }
    /// <p>The DAK certificate information of the Sidewalk device profile.</p>
    pub fn set_dak_certificate_metadata(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DakCertificateMetadata>>,
    ) -> Self {
        self.dak_certificate_metadata = input;
        self
    }
    /// Consumes the builder and constructs a [`SidewalkGetDeviceProfile`](crate::types::SidewalkGetDeviceProfile).
    pub fn build(self) -> crate::types::SidewalkGetDeviceProfile {
        crate::types::SidewalkGetDeviceProfile {
            application_server_public_key: self.application_server_public_key,
            qualification_status: self.qualification_status,
            dak_certificate_metadata: self.dak_certificate_metadata,
        }
    }
}
impl ::std::fmt::Debug for SidewalkGetDeviceProfileBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SidewalkGetDeviceProfileBuilder");
        formatter.field(
            "application_server_public_key",
            &"*** Sensitive Data Redacted ***",
        );
        formatter.field("qualification_status", &self.qualification_status);
        formatter.field("dak_certificate_metadata", &self.dak_certificate_metadata);
        formatter.finish()
    }
}
