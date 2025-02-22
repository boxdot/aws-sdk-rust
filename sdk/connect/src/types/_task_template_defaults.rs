// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes default values for fields on a template.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TaskTemplateDefaults {
    /// <p>Default value for the field.</p>
    #[doc(hidden)]
    pub default_field_values:
        ::std::option::Option<::std::vec::Vec<crate::types::TaskTemplateDefaultFieldValue>>,
}
impl TaskTemplateDefaults {
    /// <p>Default value for the field.</p>
    pub fn default_field_values(
        &self,
    ) -> ::std::option::Option<&[crate::types::TaskTemplateDefaultFieldValue]> {
        self.default_field_values.as_deref()
    }
}
impl TaskTemplateDefaults {
    /// Creates a new builder-style object to manufacture [`TaskTemplateDefaults`](crate::types::TaskTemplateDefaults).
    pub fn builder() -> crate::types::builders::TaskTemplateDefaultsBuilder {
        crate::types::builders::TaskTemplateDefaultsBuilder::default()
    }
}

/// A builder for [`TaskTemplateDefaults`](crate::types::TaskTemplateDefaults).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TaskTemplateDefaultsBuilder {
    pub(crate) default_field_values:
        ::std::option::Option<::std::vec::Vec<crate::types::TaskTemplateDefaultFieldValue>>,
}
impl TaskTemplateDefaultsBuilder {
    /// Appends an item to `default_field_values`.
    ///
    /// To override the contents of this collection use [`set_default_field_values`](Self::set_default_field_values).
    ///
    /// <p>Default value for the field.</p>
    pub fn default_field_values(
        mut self,
        input: crate::types::TaskTemplateDefaultFieldValue,
    ) -> Self {
        let mut v = self.default_field_values.unwrap_or_default();
        v.push(input);
        self.default_field_values = ::std::option::Option::Some(v);
        self
    }
    /// <p>Default value for the field.</p>
    pub fn set_default_field_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TaskTemplateDefaultFieldValue>>,
    ) -> Self {
        self.default_field_values = input;
        self
    }
    /// Consumes the builder and constructs a [`TaskTemplateDefaults`](crate::types::TaskTemplateDefaults).
    pub fn build(self) -> crate::types::TaskTemplateDefaults {
        crate::types::TaskTemplateDefaults {
            default_field_values: self.default_field_values,
        }
    }
}
