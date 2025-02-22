// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAnalyzer`](crate::operation::delete_analyzer::builders::DeleteAnalyzerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`analyzer_name(impl ::std::convert::Into<String>)`](crate::operation::delete_analyzer::builders::DeleteAnalyzerFluentBuilder::analyzer_name) / [`set_analyzer_name(Option<String>)`](crate::operation::delete_analyzer::builders::DeleteAnalyzerFluentBuilder::set_analyzer_name): <p>The name of the analyzer to delete.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::delete_analyzer::builders::DeleteAnalyzerFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::delete_analyzer::builders::DeleteAnalyzerFluentBuilder::set_client_token): <p>A client token.</p>
    /// - On success, responds with [`DeleteAnalyzerOutput`](crate::operation::delete_analyzer::DeleteAnalyzerOutput)
    /// - On failure, responds with [`SdkError<DeleteAnalyzerError>`](crate::operation::delete_analyzer::DeleteAnalyzerError)
    pub fn delete_analyzer(
        &self,
    ) -> crate::operation::delete_analyzer::builders::DeleteAnalyzerFluentBuilder {
        crate::operation::delete_analyzer::builders::DeleteAnalyzerFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
