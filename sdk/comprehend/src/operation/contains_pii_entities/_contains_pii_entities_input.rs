// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ContainsPiiEntitiesInput {
    /// <p>A UTF-8 text string. The maximum string size is 100 KB.</p>
    #[doc(hidden)]
    pub text: ::std::option::Option<::std::string::String>,
    /// <p>The language of the input documents. Currently, English is the only valid language.</p>
    #[doc(hidden)]
    pub language_code: ::std::option::Option<crate::types::LanguageCode>,
}
impl ContainsPiiEntitiesInput {
    /// <p>A UTF-8 text string. The maximum string size is 100 KB.</p>
    pub fn text(&self) -> ::std::option::Option<&str> {
        self.text.as_deref()
    }
    /// <p>The language of the input documents. Currently, English is the only valid language.</p>
    pub fn language_code(&self) -> ::std::option::Option<&crate::types::LanguageCode> {
        self.language_code.as_ref()
    }
}
impl ContainsPiiEntitiesInput {
    /// Creates a new builder-style object to manufacture [`ContainsPiiEntitiesInput`](crate::operation::contains_pii_entities::ContainsPiiEntitiesInput).
    pub fn builder(
    ) -> crate::operation::contains_pii_entities::builders::ContainsPiiEntitiesInputBuilder {
        crate::operation::contains_pii_entities::builders::ContainsPiiEntitiesInputBuilder::default(
        )
    }
}

/// A builder for [`ContainsPiiEntitiesInput`](crate::operation::contains_pii_entities::ContainsPiiEntitiesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ContainsPiiEntitiesInputBuilder {
    pub(crate) text: ::std::option::Option<::std::string::String>,
    pub(crate) language_code: ::std::option::Option<crate::types::LanguageCode>,
}
impl ContainsPiiEntitiesInputBuilder {
    /// <p>A UTF-8 text string. The maximum string size is 100 KB.</p>
    pub fn text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.text = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A UTF-8 text string. The maximum string size is 100 KB.</p>
    pub fn set_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.text = input;
        self
    }
    /// <p>The language of the input documents. Currently, English is the only valid language.</p>
    pub fn language_code(mut self, input: crate::types::LanguageCode) -> Self {
        self.language_code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The language of the input documents. Currently, English is the only valid language.</p>
    pub fn set_language_code(
        mut self,
        input: ::std::option::Option<crate::types::LanguageCode>,
    ) -> Self {
        self.language_code = input;
        self
    }
    /// Consumes the builder and constructs a [`ContainsPiiEntitiesInput`](crate::operation::contains_pii_entities::ContainsPiiEntitiesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::contains_pii_entities::ContainsPiiEntitiesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::contains_pii_entities::ContainsPiiEntitiesInput {
                text: self.text,
                language_code: self.language_code,
            },
        )
    }
}
