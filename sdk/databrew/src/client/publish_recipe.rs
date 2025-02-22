// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PublishRecipe`](crate::operation::publish_recipe::builders::PublishRecipeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::publish_recipe::builders::PublishRecipeFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::publish_recipe::builders::PublishRecipeFluentBuilder::set_description): <p>A description of the recipe to be published, for this version of the recipe.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::publish_recipe::builders::PublishRecipeFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::publish_recipe::builders::PublishRecipeFluentBuilder::set_name): <p>The name of the recipe to be published.</p>
    /// - On success, responds with [`PublishRecipeOutput`](crate::operation::publish_recipe::PublishRecipeOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::operation::publish_recipe::PublishRecipeOutput::name): <p>The name of the recipe that you published.</p>
    /// - On failure, responds with [`SdkError<PublishRecipeError>`](crate::operation::publish_recipe::PublishRecipeError)
    pub fn publish_recipe(
        &self,
    ) -> crate::operation::publish_recipe::builders::PublishRecipeFluentBuilder {
        crate::operation::publish_recipe::builders::PublishRecipeFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
