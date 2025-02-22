// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutItems`](crate::operation::put_items::builders::PutItemsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dataset_arn(impl ::std::convert::Into<String>)`](crate::operation::put_items::builders::PutItemsFluentBuilder::dataset_arn) / [`set_dataset_arn(Option<String>)`](crate::operation::put_items::builders::PutItemsFluentBuilder::set_dataset_arn): <p>The Amazon Resource Name (ARN) of the Items dataset you are adding the item or items to.</p>
    ///   - [`items(Vec<Item>)`](crate::operation::put_items::builders::PutItemsFluentBuilder::items) / [`set_items(Option<Vec<Item>>)`](crate::operation::put_items::builders::PutItemsFluentBuilder::set_items): <p>A list of item data.</p>
    /// - On success, responds with [`PutItemsOutput`](crate::operation::put_items::PutItemsOutput)
    /// - On failure, responds with [`SdkError<PutItemsError>`](crate::operation::put_items::PutItemsError)
    pub fn put_items(&self) -> crate::operation::put_items::builders::PutItemsFluentBuilder {
        crate::operation::put_items::builders::PutItemsFluentBuilder::new(self.handle.clone())
    }
}
