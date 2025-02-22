// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a contact evaluation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Evaluation {
    /// <p>A unique identifier for the contact evaluation.</p>
    #[doc(hidden)]
    pub evaluation_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) for the contact evaluation resource.</p>
    #[doc(hidden)]
    pub evaluation_arn: ::std::option::Option<::std::string::String>,
    /// <p>Metadata about the contact evaluation.</p>
    #[doc(hidden)]
    pub metadata: ::std::option::Option<crate::types::EvaluationMetadata>,
    /// <p>A map of question identifiers to answer value.</p>
    #[doc(hidden)]
    pub answers: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::EvaluationAnswerOutput>,
    >,
    /// <p>A map of question identifiers to note value.</p>
    #[doc(hidden)]
    pub notes: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::EvaluationNote>,
    >,
    /// <p>The status of the contact evaluation.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::EvaluationStatus>,
    /// <p>A map of item (section or question) identifiers to score value.</p>
    #[doc(hidden)]
    pub scores: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::EvaluationScore>,
    >,
    /// <p>The timestamp for when the evaluation was created.</p>
    #[doc(hidden)]
    pub created_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The timestamp for when the evaluation was last updated.</p>
    #[doc(hidden)]
    pub last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl Evaluation {
    /// <p>A unique identifier for the contact evaluation.</p>
    pub fn evaluation_id(&self) -> ::std::option::Option<&str> {
        self.evaluation_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) for the contact evaluation resource.</p>
    pub fn evaluation_arn(&self) -> ::std::option::Option<&str> {
        self.evaluation_arn.as_deref()
    }
    /// <p>Metadata about the contact evaluation.</p>
    pub fn metadata(&self) -> ::std::option::Option<&crate::types::EvaluationMetadata> {
        self.metadata.as_ref()
    }
    /// <p>A map of question identifiers to answer value.</p>
    pub fn answers(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, crate::types::EvaluationAnswerOutput>,
    > {
        self.answers.as_ref()
    }
    /// <p>A map of question identifiers to note value.</p>
    pub fn notes(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, crate::types::EvaluationNote>,
    > {
        self.notes.as_ref()
    }
    /// <p>The status of the contact evaluation.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::EvaluationStatus> {
        self.status.as_ref()
    }
    /// <p>A map of item (section or question) identifiers to score value.</p>
    pub fn scores(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, crate::types::EvaluationScore>,
    > {
        self.scores.as_ref()
    }
    /// <p>The timestamp for when the evaluation was created.</p>
    pub fn created_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_time.as_ref()
    }
    /// <p>The timestamp for when the evaluation was last updated.</p>
    pub fn last_modified_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified_time.as_ref()
    }
    /// <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl Evaluation {
    /// Creates a new builder-style object to manufacture [`Evaluation`](crate::types::Evaluation).
    pub fn builder() -> crate::types::builders::EvaluationBuilder {
        crate::types::builders::EvaluationBuilder::default()
    }
}

/// A builder for [`Evaluation`](crate::types::Evaluation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EvaluationBuilder {
    pub(crate) evaluation_id: ::std::option::Option<::std::string::String>,
    pub(crate) evaluation_arn: ::std::option::Option<::std::string::String>,
    pub(crate) metadata: ::std::option::Option<crate::types::EvaluationMetadata>,
    pub(crate) answers: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::EvaluationAnswerOutput>,
    >,
    pub(crate) notes: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::EvaluationNote>,
    >,
    pub(crate) status: ::std::option::Option<crate::types::EvaluationStatus>,
    pub(crate) scores: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::EvaluationScore>,
    >,
    pub(crate) created_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl EvaluationBuilder {
    /// <p>A unique identifier for the contact evaluation.</p>
    pub fn evaluation_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.evaluation_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the contact evaluation.</p>
    pub fn set_evaluation_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.evaluation_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the contact evaluation resource.</p>
    pub fn evaluation_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.evaluation_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the contact evaluation resource.</p>
    pub fn set_evaluation_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.evaluation_arn = input;
        self
    }
    /// <p>Metadata about the contact evaluation.</p>
    pub fn metadata(mut self, input: crate::types::EvaluationMetadata) -> Self {
        self.metadata = ::std::option::Option::Some(input);
        self
    }
    /// <p>Metadata about the contact evaluation.</p>
    pub fn set_metadata(
        mut self,
        input: ::std::option::Option<crate::types::EvaluationMetadata>,
    ) -> Self {
        self.metadata = input;
        self
    }
    /// Adds a key-value pair to `answers`.
    ///
    /// To override the contents of this collection use [`set_answers`](Self::set_answers).
    ///
    /// <p>A map of question identifiers to answer value.</p>
    pub fn answers(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::EvaluationAnswerOutput,
    ) -> Self {
        let mut hash_map = self.answers.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.answers = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A map of question identifiers to answer value.</p>
    pub fn set_answers(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<
                ::std::string::String,
                crate::types::EvaluationAnswerOutput,
            >,
        >,
    ) -> Self {
        self.answers = input;
        self
    }
    /// Adds a key-value pair to `notes`.
    ///
    /// To override the contents of this collection use [`set_notes`](Self::set_notes).
    ///
    /// <p>A map of question identifiers to note value.</p>
    pub fn notes(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::EvaluationNote,
    ) -> Self {
        let mut hash_map = self.notes.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.notes = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A map of question identifiers to note value.</p>
    pub fn set_notes(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::EvaluationNote>,
        >,
    ) -> Self {
        self.notes = input;
        self
    }
    /// <p>The status of the contact evaluation.</p>
    pub fn status(mut self, input: crate::types::EvaluationStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the contact evaluation.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::EvaluationStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// Adds a key-value pair to `scores`.
    ///
    /// To override the contents of this collection use [`set_scores`](Self::set_scores).
    ///
    /// <p>A map of item (section or question) identifiers to score value.</p>
    pub fn scores(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::EvaluationScore,
    ) -> Self {
        let mut hash_map = self.scores.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.scores = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A map of item (section or question) identifiers to score value.</p>
    pub fn set_scores(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::EvaluationScore>,
        >,
    ) -> Self {
        self.scores = input;
        self
    }
    /// <p>The timestamp for when the evaluation was created.</p>
    pub fn created_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp for when the evaluation was created.</p>
    pub fn set_created_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_time = input;
        self
    }
    /// <p>The timestamp for when the evaluation was last updated.</p>
    pub fn last_modified_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp for when the evaluation was last updated.</p>
    pub fn set_last_modified_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_modified_time = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p>
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
    /// <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`Evaluation`](crate::types::Evaluation).
    pub fn build(self) -> crate::types::Evaluation {
        crate::types::Evaluation {
            evaluation_id: self.evaluation_id,
            evaluation_arn: self.evaluation_arn,
            metadata: self.metadata,
            answers: self.answers,
            notes: self.notes,
            status: self.status,
            scores: self.scores,
            created_time: self.created_time,
            last_modified_time: self.last_modified_time,
            tags: self.tags,
        }
    }
}
