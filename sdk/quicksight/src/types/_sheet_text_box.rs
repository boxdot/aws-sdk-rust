// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A text box.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SheetTextBox {
    /// <p>The unique identifier for a text box. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have text boxes that share identifiers.</p>
    #[doc(hidden)]
    pub sheet_text_box_id: ::std::option::Option<::std::string::String>,
    /// <p>The content that is displayed in the text box.</p>
    #[doc(hidden)]
    pub content: ::std::option::Option<::std::string::String>,
}
impl SheetTextBox {
    /// <p>The unique identifier for a text box. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have text boxes that share identifiers.</p>
    pub fn sheet_text_box_id(&self) -> ::std::option::Option<&str> {
        self.sheet_text_box_id.as_deref()
    }
    /// <p>The content that is displayed in the text box.</p>
    pub fn content(&self) -> ::std::option::Option<&str> {
        self.content.as_deref()
    }
}
impl SheetTextBox {
    /// Creates a new builder-style object to manufacture [`SheetTextBox`](crate::types::SheetTextBox).
    pub fn builder() -> crate::types::builders::SheetTextBoxBuilder {
        crate::types::builders::SheetTextBoxBuilder::default()
    }
}

/// A builder for [`SheetTextBox`](crate::types::SheetTextBox).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SheetTextBoxBuilder {
    pub(crate) sheet_text_box_id: ::std::option::Option<::std::string::String>,
    pub(crate) content: ::std::option::Option<::std::string::String>,
}
impl SheetTextBoxBuilder {
    /// <p>The unique identifier for a text box. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have text boxes that share identifiers.</p>
    pub fn sheet_text_box_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.sheet_text_box_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for a text box. This identifier must be unique within the context of a dashboard, template, or analysis. Two dashboards, analyses, or templates can have text boxes that share identifiers.</p>
    pub fn set_sheet_text_box_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.sheet_text_box_id = input;
        self
    }
    /// <p>The content that is displayed in the text box.</p>
    pub fn content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The content that is displayed in the text box.</p>
    pub fn set_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content = input;
        self
    }
    /// Consumes the builder and constructs a [`SheetTextBox`](crate::types::SheetTextBox).
    pub fn build(self) -> crate::types::SheetTextBox {
        crate::types::SheetTextBox {
            sheet_text_box_id: self.sheet_text_box_id,
            content: self.content,
        }
    }
}
