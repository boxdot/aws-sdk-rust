// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateGameServer`](crate::operation::update_game_server::builders::UpdateGameServerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`game_server_group_name(impl ::std::convert::Into<String>)`](crate::operation::update_game_server::builders::UpdateGameServerFluentBuilder::game_server_group_name) / [`set_game_server_group_name(Option<String>)`](crate::operation::update_game_server::builders::UpdateGameServerFluentBuilder::set_game_server_group_name): <p>A unique identifier for the game server group where the game server is running.</p>
    ///   - [`game_server_id(impl ::std::convert::Into<String>)`](crate::operation::update_game_server::builders::UpdateGameServerFluentBuilder::game_server_id) / [`set_game_server_id(Option<String>)`](crate::operation::update_game_server::builders::UpdateGameServerFluentBuilder::set_game_server_id): <p>A custom string that uniquely identifies the game server to update.</p>
    ///   - [`game_server_data(impl ::std::convert::Into<String>)`](crate::operation::update_game_server::builders::UpdateGameServerFluentBuilder::game_server_data) / [`set_game_server_data(Option<String>)`](crate::operation::update_game_server::builders::UpdateGameServerFluentBuilder::set_game_server_data): <p>A set of custom game server properties, formatted as a single string value. This data is passed to a game client or service when it requests information on game servers. </p>
    ///   - [`utilization_status(GameServerUtilizationStatus)`](crate::operation::update_game_server::builders::UpdateGameServerFluentBuilder::utilization_status) / [`set_utilization_status(Option<GameServerUtilizationStatus>)`](crate::operation::update_game_server::builders::UpdateGameServerFluentBuilder::set_utilization_status): <p>Indicates whether the game server is available or is currently hosting gameplay.</p>
    ///   - [`health_check(GameServerHealthCheck)`](crate::operation::update_game_server::builders::UpdateGameServerFluentBuilder::health_check) / [`set_health_check(Option<GameServerHealthCheck>)`](crate::operation::update_game_server::builders::UpdateGameServerFluentBuilder::set_health_check): <p>Indicates health status of the game server. A request that includes this parameter updates the game server's <i>LastHealthCheckTime</i> timestamp. </p>
    /// - On success, responds with [`UpdateGameServerOutput`](crate::operation::update_game_server::UpdateGameServerOutput) with field(s):
    ///   - [`game_server(Option<GameServer>)`](crate::operation::update_game_server::UpdateGameServerOutput::game_server): <p>Object that describes the newly updated game server.</p>
    /// - On failure, responds with [`SdkError<UpdateGameServerError>`](crate::operation::update_game_server::UpdateGameServerError)
    pub fn update_game_server(
        &self,
    ) -> crate::operation::update_game_server::builders::UpdateGameServerFluentBuilder {
        crate::operation::update_game_server::builders::UpdateGameServerFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
