// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutObject`](crate::operation::put_object::builders::PutObjectFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`backup_job_id(impl ::std::convert::Into<String>)`](crate::operation::put_object::builders::PutObjectFluentBuilder::backup_job_id) / [`set_backup_job_id(Option<String>)`](crate::operation::put_object::builders::PutObjectFluentBuilder::set_backup_job_id): Backup job Id for the in-progress backup.
    ///   - [`object_name(impl ::std::convert::Into<String>)`](crate::operation::put_object::builders::PutObjectFluentBuilder::object_name) / [`set_object_name(Option<String>)`](crate::operation::put_object::builders::PutObjectFluentBuilder::set_object_name): The name of the Object to be uploaded.
    ///   - [`metadata_string(impl ::std::convert::Into<String>)`](crate::operation::put_object::builders::PutObjectFluentBuilder::metadata_string) / [`set_metadata_string(Option<String>)`](crate::operation::put_object::builders::PutObjectFluentBuilder::set_metadata_string): Store user defined metadata like backup checksum, disk ids, restore metadata etc.
    ///   - [`inline_chunk(ByteStream)`](crate::operation::put_object::builders::PutObjectFluentBuilder::inline_chunk) / [`set_inline_chunk(ByteStream)`](crate::operation::put_object::builders::PutObjectFluentBuilder::set_inline_chunk): Inline chunk data to be uploaded.
    ///   - [`inline_chunk_length(i64)`](crate::operation::put_object::builders::PutObjectFluentBuilder::inline_chunk_length) / [`set_inline_chunk_length(i64)`](crate::operation::put_object::builders::PutObjectFluentBuilder::set_inline_chunk_length): Length of the inline chunk data.
    ///   - [`inline_chunk_checksum(impl ::std::convert::Into<String>)`](crate::operation::put_object::builders::PutObjectFluentBuilder::inline_chunk_checksum) / [`set_inline_chunk_checksum(Option<String>)`](crate::operation::put_object::builders::PutObjectFluentBuilder::set_inline_chunk_checksum): Inline chunk checksum
    ///   - [`inline_chunk_checksum_algorithm(impl ::std::convert::Into<String>)`](crate::operation::put_object::builders::PutObjectFluentBuilder::inline_chunk_checksum_algorithm) / [`set_inline_chunk_checksum_algorithm(Option<String>)`](crate::operation::put_object::builders::PutObjectFluentBuilder::set_inline_chunk_checksum_algorithm): Inline chunk checksum algorithm
    ///   - [`object_checksum(impl ::std::convert::Into<String>)`](crate::operation::put_object::builders::PutObjectFluentBuilder::object_checksum) / [`set_object_checksum(Option<String>)`](crate::operation::put_object::builders::PutObjectFluentBuilder::set_object_checksum): object checksum
    ///   - [`object_checksum_algorithm(SummaryChecksumAlgorithm)`](crate::operation::put_object::builders::PutObjectFluentBuilder::object_checksum_algorithm) / [`set_object_checksum_algorithm(Option<SummaryChecksumAlgorithm>)`](crate::operation::put_object::builders::PutObjectFluentBuilder::set_object_checksum_algorithm): object checksum algorithm
    ///   - [`throw_on_duplicate(bool)`](crate::operation::put_object::builders::PutObjectFluentBuilder::throw_on_duplicate) / [`set_throw_on_duplicate(bool)`](crate::operation::put_object::builders::PutObjectFluentBuilder::set_throw_on_duplicate): Throw an exception if Object name is already exist.
    /// - On success, responds with [`PutObjectOutput`](crate::operation::put_object::PutObjectOutput) with field(s):
    ///   - [`inline_chunk_checksum(Option<String>)`](crate::operation::put_object::PutObjectOutput::inline_chunk_checksum): Inline chunk checksum
    ///   - [`inline_chunk_checksum_algorithm(Option<DataChecksumAlgorithm>)`](crate::operation::put_object::PutObjectOutput::inline_chunk_checksum_algorithm): Inline chunk checksum algorithm
    ///   - [`object_checksum(Option<String>)`](crate::operation::put_object::PutObjectOutput::object_checksum): object checksum
    ///   - [`object_checksum_algorithm(Option<SummaryChecksumAlgorithm>)`](crate::operation::put_object::PutObjectOutput::object_checksum_algorithm): object checksum algorithm
    /// - On failure, responds with [`SdkError<PutObjectError>`](crate::operation::put_object::PutObjectError)
    pub fn put_object(&self) -> crate::operation::put_object::builders::PutObjectFluentBuilder {
        crate::operation::put_object::builders::PutObjectFluentBuilder::new(self.handle.clone())
    }
}
