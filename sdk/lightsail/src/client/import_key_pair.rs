// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ImportKeyPair`](crate::operation::import_key_pair::builders::ImportKeyPairFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_pair_name(impl ::std::convert::Into<String>)`](crate::operation::import_key_pair::builders::ImportKeyPairFluentBuilder::key_pair_name) / [`set_key_pair_name(Option<String>)`](crate::operation::import_key_pair::builders::ImportKeyPairFluentBuilder::set_key_pair_name): <p>The name of the key pair for which you want to import the public key.</p>
    ///   - [`public_key_base64(impl ::std::convert::Into<String>)`](crate::operation::import_key_pair::builders::ImportKeyPairFluentBuilder::public_key_base64) / [`set_public_key_base64(Option<String>)`](crate::operation::import_key_pair::builders::ImportKeyPairFluentBuilder::set_public_key_base64): <p>A base64-encoded public key of the <code>ssh-rsa</code> type.</p>
    /// - On success, responds with [`ImportKeyPairOutput`](crate::operation::import_key_pair::ImportKeyPairOutput) with field(s):
    ///   - [`operation(Option<Operation>)`](crate::operation::import_key_pair::ImportKeyPairOutput::operation): <p>An array of objects that describe the result of the action, such as the status of the request, the timestamp of the request, and the resources affected by the request.</p>
    /// - On failure, responds with [`SdkError<ImportKeyPairError>`](crate::operation::import_key_pair::ImportKeyPairError)
    pub fn import_key_pair(
        &self,
    ) -> crate::operation::import_key_pair::builders::ImportKeyPairFluentBuilder {
        crate::operation::import_key_pair::builders::ImportKeyPairFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
