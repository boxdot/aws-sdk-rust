// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRoom`](crate::operation::delete_room::builders::DeleteRoomFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identifier(impl ::std::convert::Into<String>)`](crate::operation::delete_room::builders::DeleteRoomFluentBuilder::identifier) / [`set_identifier(Option<String>)`](crate::operation::delete_room::builders::DeleteRoomFluentBuilder::set_identifier): <p>Identifier of the room to be deleted. Currently this must be an ARN.</p>
    /// - On success, responds with [`DeleteRoomOutput`](crate::operation::delete_room::DeleteRoomOutput)
    /// - On failure, responds with [`SdkError<DeleteRoomError>`](crate::operation::delete_room::DeleteRoomError)
    pub fn delete_room(&self) -> crate::operation::delete_room::builders::DeleteRoomFluentBuilder {
        crate::operation::delete_room::builders::DeleteRoomFluentBuilder::new(self.handle.clone())
    }
}
