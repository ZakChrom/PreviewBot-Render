mod quell;
mod render;
mod levels;

use std::env;
use std::time::Instant;

use render::render::render;
use quell::codes::import;

use serenity::all::{ActivityData, CommandOptionType, CreateAttachment, CreateCommand, CreateCommandOption, CreateInteractionResponseFollowup, Interaction};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

const OUTPUT: &str = "render.mp4";

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let command = interaction.into_command().unwrap();
        if command.data.name.as_str() == "preview" {
            if let Err(why) = command.defer(&ctx.http).await {
                println!("Cannot defer interaction: {}", why);
            };

            let mut level = command.data.options
                .get(0)
                .expect("Expected level option")
                .value
                .as_str()
                .expect("Idk");

            let ticks = command.data.options
                .get(1)
                .expect("Expected ticks option")
                .value
                .as_i64()
                .expect("Idk") as u64;

            let tps_option = command.data.options.iter().find(|&opt| opt.name == "tps");

            let tps = tps_option
                .and_then(|opt| opt.value.as_i64())
                .map(|value| value as u64)
                .unwrap_or(5);

            let now = Instant::now();

            if let Some(l) = levels::default().get(level) {
                level = l;
            }

            let mut grid = import(level).unwrap();
            
            let times = render(&mut grid, ticks, tps, OUTPUT);

            let size = std::fs::metadata(OUTPUT).unwrap().len();
            
            let mut response = CreateInteractionResponseFollowup::new()
                .content(format!("Time: {}ms\nUpdate min/max/avg: {}μs {}μs {}μs\nRender min/max/avg: {}ms {}ms {}ms\nWrite min/max/avg: {}ms {}ms {}ms{}",
                    now.elapsed().as_millis(),
                    times.0.iter().min().unwrap(), times.0.iter().max().unwrap(), times.0.iter().sum::<u128>() / times.0.len() as u128,
                    times.1.iter().min().unwrap(), times.1.iter().max().unwrap(), times.1.iter().sum::<u128>() / times.1.len() as u128,
                    times.2.iter().min().unwrap(), times.2.iter().max().unwrap(), times.2.iter().sum::<u128>() / times.2.len() as u128,
                    if size < 25000000 { "".to_owned() } else { format!("\n\nPreview is too big. {} bytes", size) }
                ));

            if size < 25000000 {
                response = response.add_file(CreateAttachment::path(OUTPUT).await.unwrap());
            }
            
            if let Err(why) = command.create_followup(&ctx.http, response).await {
                println!("Cannot response to slash command: {}", why);
            }
            /*.embed(|embed| {
                embed
                    .title("Render")
                    .description("")
                    .color(0x43FF19)
                    .image("attachment://render.mp4")
            })*/
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in env")
                .parse()
                .expect("GUILD_ID must be an integer")
        );

        let famoke = GuildId::new(989451474008432700);

        let command = CreateCommand::new("preview")
            .description("Preview a level code (V3 only)")
            .add_option(CreateCommandOption::new(CommandOptionType::String, "level", "The level code").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "ticks", "How many ticks to render").required(true).min_int_value(1).max_int_value(1000))
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "tps", "Ticks per second the render should play as").min_int_value(1).max_int_value(1000));

        guild_id.create_command(&ctx.http, command.clone()).await.unwrap();
        famoke.create_command(ctx.http, command).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in env");

    let now = Instant::now();
    
    let mut grid = import(levels::default().get("ao").unwrap()).unwrap();
    let times = render(&mut grid, 11, 5, OUTPUT);

    println!("Time: {}ms\nUpdate min/max/avg: {}μs {}μs {}μs\nRender min/max/avg: {}ms {}ms {}ms\nWrite min/max/avg: {}ms {}ms {}ms",
        now.elapsed().as_millis(),
        times.0.iter().min().unwrap(), times.0.iter().max().unwrap(), times.0.iter().sum::<u128>() / times.0.len() as u128,
        times.1.iter().min().unwrap(), times.1.iter().max().unwrap(), times.1.iter().sum::<u128>() / times.1.len() as u128,
        times.2.iter().min().unwrap(), times.2.iter().max().unwrap(), times.2.iter().sum::<u128>() / times.2.len() as u128,
    );

    let mut client = Client::builder(token, GatewayIntents::empty())
        .activity(ActivityData::competing("fastest preview bot with Cell Machine Simulator"))
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
