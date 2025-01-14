// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateGame`](crate::operation::update_game::builders::UpdateGameFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`game_name(impl ::std::convert::Into<String>)`](crate::operation::update_game::builders::UpdateGameFluentBuilder::game_name) / [`set_game_name(Option<String>)`](crate::operation::update_game::builders::UpdateGameFluentBuilder::set_game_name): <p>The name of the game.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_game::builders::UpdateGameFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_game::builders::UpdateGameFluentBuilder::set_description): <p>The description of the game.</p>
    /// - On success, responds with [`UpdateGameOutput`](crate::operation::update_game::UpdateGameOutput) with field(s):
    ///   - [`game(Option<GameDetails>)`](crate::operation::update_game::UpdateGameOutput::game): <p>The details of the game.</p>
    /// - On failure, responds with [`SdkError<UpdateGameError>`](crate::operation::update_game::UpdateGameError)
    pub fn update_game(&self) -> crate::operation::update_game::builders::UpdateGameFluentBuilder {
        crate::operation::update_game::builders::UpdateGameFluentBuilder::new(self.handle.clone())
    }
}
