#![feature(duration_millis_float)]

mod quell;
mod render;
mod levels;

use std::env;
use std::time::Instant;

use quell::cells::Cell;
use render::render::{preview, render};
use quell::codes::import;

use serenity::all::{ActivityData, CommandOptionType, CreateAttachment, CreateCommand, CreateCommandOption, CreateInteractionResponseFollowup, Interaction};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

const RENDER_OUTPUT: &str = "render.mp4";
const PREVIEW_OUTPUT: &str = "preview.jpg";

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let command = interaction.into_command().unwrap();
        if command.data.name == "preview" {
            if let Err(why) = command.defer(&ctx.http).await {
                println!("Cannot defer interaction: {}", why);
            };

            let mut level = command.data.options
                .get(0)
                .expect("Expected level option")
                .value
                .as_str()
                .expect("Idk");

            let ticks_option = command.data.options.iter().find(|&opt| opt.name == "ticks");
            let ticks = ticks_option
                .and_then(|opt| opt.value.as_i64())
                .map(|value| value as u64)
                .unwrap_or(0);

            let tps_option = command.data.options.iter().find(|&opt| opt.name == "tps");
            let tps = tps_option
                .and_then(|opt| opt.value.as_i64())
                .map(|value| value as u64)
                .unwrap_or(5);

            let gpu_option = command.data.options.iter().find(|&opt| opt.name == "gpu");
            let gpu = gpu_option
                .and_then(|opt| opt.value.as_bool())
                .map(|value| value as bool)
                .unwrap_or(false);

            let now = Instant::now();

            if let Some(l) = levels::default().get(level) {
                level = l;
            }
            let mut grid = import(level).unwrap();
            
            let output = if ticks == 0 { PREVIEW_OUTPUT } else { RENDER_OUTPUT };

            let size: u64;
            let mut response = CreateInteractionResponseFollowup::new();

            // Preview else render
            if ticks == 0 {
                let times = preview(&mut grid, output);
                let elapsed = now.elapsed().as_millis();
                size = std::fs::metadata(output).unwrap().len();
                response = response.content(format!("Time: {}ms\nPreview ({}) {}μs\nSaving {}ms{}",
                    elapsed,
                    if times.2 { "minimal" } else { "normal" }, times.0,
                    times.1,
                    if size < 25000000 { "".to_string() } else { format!("\n\nPreview is too big. {} bytes", size) }
                ));
            } else {
                let times = render(&mut grid, ticks, tps, output, gpu);
                let elapsed = now.elapsed().as_millis();
                size = std::fs::metadata(output).unwrap().len();
                response = response.content(format!("Time: {}ms\nUpdate min/max/avg: {}μs {}μs {}μs\nRender ({}) min/max/avg: {}μs {}μs {}μs\nWrite min/max/avg: {}ms {}ms {}ms{}",
                    elapsed,
                    times.0.iter().min().unwrap(), times.0.iter().max().unwrap(), times.0.iter().sum::<u128>() / times.0.len() as u128,
                    if times.3 { "minimal" } else { "normal" }, times.1.iter().min().unwrap(), times.1.iter().max().unwrap(), times.1.iter().sum::<u128>() / times.1.len() as u128,
                    times.2.iter().min().unwrap(), times.2.iter().max().unwrap(), times.2.iter().sum::<u128>() / times.2.len() as u128,
                    if size < 25000000 { "".to_string() } else { format!("\n\nPreview is too big. {} bytes", size) }
                ));
            }

            if size < 25000000 {
                response = response.add_file(CreateAttachment::path(output).await.unwrap());
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

        let render_command = CreateCommand::new("preview")
            .description("Preview a level (V3 only)")
            .add_option(CreateCommandOption::new(CommandOptionType::String, "level", "The level code").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "ticks", "How many ticks to render").required(false).min_int_value(1).max_int_value(2_u64.pow(32)))
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "tps", "Ticks per second the render should play as").min_int_value(1).max_int_value(1000))
            .add_option(CreateCommandOption::new(CommandOptionType::Boolean, "gpu", "Use gpu for rendering (experimental)"));
        
        let commands = guild_id.get_commands(&ctx.http).await.unwrap();
        let famoke_commands = famoke.get_commands(&ctx.http).await.unwrap();

        for command in commands {
            guild_id.delete_command(&ctx.http, command.id).await.unwrap();
        }
        for command in famoke_commands {
            famoke.delete_command(&ctx.http, command.id).await.unwrap();
        }

        guild_id.create_command(&ctx.http, render_command.clone()).await.unwrap();
        famoke.create_command(&ctx.http, render_command).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    println!("Cell is {} bytes", std::mem::size_of::<Cell>());

    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in env");

    let now = Instant::now();
    
    let mut grid = import(levels::default().get("lorux").unwrap()).unwrap();
    let times = render(&mut grid, 11, 5, RENDER_OUTPUT, true);

    println!("Time: {}ms\nUpdate min/max/avg: {}μs {}μs {}μs\nRender min/max/avg: {}μs {}μs {}μs\nWrite min/max/avg: {}ms {}ms {}ms",
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
