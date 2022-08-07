use super::CadencyCommand;
use crate::error::CadencyError;
use crate::utils;
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::Command, interaction::application_command::ApplicationCommandInteraction,
    },
};

pub struct Ping;

#[async_trait]
impl CadencyCommand for Ping {
    /// Construct the slash command that will be submited to the discord api
    async fn register(ctx: &Context) -> Result<Command, serenity::Error> {
        Ok(
            Command::create_global_application_command(&ctx.http, |command| {
                command.name("ping").description("Play Ping-Pong")
            })
            .await?,
        )
    }

    async fn execute<'a>(
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        debug!("Execute ping command");
        utils::create_response(ctx, command, "Pong!").await?;
        Ok(())
    }
}