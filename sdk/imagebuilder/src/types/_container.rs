// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container encapsulates the runtime environment for an application.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Container {
    /// <p>Containers and container images are Region-specific. This is the Region context for the container.</p>
    #[doc(hidden)]
    pub region: ::std::option::Option<::std::string::String>,
    /// <p>A list of URIs for containers created in the context Region.</p>
    #[doc(hidden)]
    pub image_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl Container {
    /// <p>Containers and container images are Region-specific. This is the Region context for the container.</p>
    pub fn region(&self) -> ::std::option::Option<&str> {
        self.region.as_deref()
    }
    /// <p>A list of URIs for containers created in the context Region.</p>
    pub fn image_uris(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.image_uris.as_deref()
    }
}
impl Container {
    /// Creates a new builder-style object to manufacture [`Container`](crate::types::Container).
    pub fn builder() -> crate::types::builders::ContainerBuilder {
        crate::types::builders::ContainerBuilder::default()
    }
}

/// A builder for [`Container`](crate::types::Container).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ContainerBuilder {
    pub(crate) region: ::std::option::Option<::std::string::String>,
    pub(crate) image_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ContainerBuilder {
    /// <p>Containers and container images are Region-specific. This is the Region context for the container.</p>
    pub fn region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Containers and container images are Region-specific. This is the Region context for the container.</p>
    pub fn set_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.region = input;
        self
    }
    /// Appends an item to `image_uris`.
    ///
    /// To override the contents of this collection use [`set_image_uris`](Self::set_image_uris).
    ///
    /// <p>A list of URIs for containers created in the context Region.</p>
    pub fn image_uris(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.image_uris.unwrap_or_default();
        v.push(input.into());
        self.image_uris = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of URIs for containers created in the context Region.</p>
    pub fn set_image_uris(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.image_uris = input;
        self
    }
    /// Consumes the builder and constructs a [`Container`](crate::types::Container).
    pub fn build(self) -> crate::types::Container {
        crate::types::Container {
            region: self.region,
            image_uris: self.image_uris,
        }
    }
}
