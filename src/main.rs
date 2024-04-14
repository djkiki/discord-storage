use std::char::MAX;
use std::env;

use serenity::{
    async_trait,
    client::Context,
    model::{
        channel::{Message, Reaction},
        gateway::Ready,
        id::ChannelId,
    },
    prelude::*,
};
use serenity::builder::CreateMessage;
use serenity::builder::CreateAttachment;
use serenity::builder::CreateEmbed;

use tokio::io::AsyncReadExt;

use std::fs::{self, File};
use std::io::{self, prelude::*};
use hex;

const MAX_FILE_SIZE_BYTES: usize = 25 * 1024 * 1024; // 25 MB

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
		let file_name = "./lorem";
        if msg.content == "!ping" {
	    match wczytywanie_pliku().await{
		Ok(hex_string) => {
			if let Err(why) = send_file_chunks(&ctx,msg.channel_id, hex_string.clone(), file_name).await{
				println!("Error sending a message: {:?}", why);
				}
				for chunk in hex_string.chars().collect::<Vec<char>>().chunks(MAX_FILE_SIZE_BYTES){
					let chunk_str: String = chunk.into_iter().collect();
					if let Err(why) = msg.channel_id.say(&ctx.http, chunk_str).await{
							println!("Error sending message: {:?}", why);
						}
					}			
		},
			Err(e) => {
			if let Err(why) = msg.channel_id.say(&ctx.http, format!("Error: {}", e)).await {
					println!("Error sending a message: {:?}", why);
					}
				}
			}
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

async fn wczytywanie_pliku() -> io::Result<String>{
	let mut file = File::open("fagata.png").expect("Nie mozna otworzyc pliku");
	let mut bytes_buffer = Vec::new();
	file.read_to_end(&mut bytes_buffer).expect("Error przy odczycie");
	let hex_string = hex::encode(&bytes_buffer);
	Ok(hex_string)
}

async fn send_file_chunks(ctx: &Context, channel_id: ChannelId, hex_string: String, file_name: &str) -> Result<(), serenity::Error> {
	let bytes = hex_string.into_bytes();
	let mut offset = 0;
	while offset< bytes.len() {
		let chunk_size = std::cmp::min(MAX_FILE_SIZE_BYTES, bytes.len() - offset);
		let chunk = &bytes[offset..offset + chunk_size];
		send_file_part(ctx,&channel_id, chunk, file_name).await?;
		offset += chunk_size;

	}

	Ok(())
}

async fn send_file_part(ctx: &Context, channel_id: &ChannelId, chunk: &[u8], file_name: &str) -> Result<(), serenity::Error> {
	let temp_file_path = format!("{}_part.txt", file_name);
	let mut temp_file = File::create(&temp_file_path)?;
	temp_file.write_all(chunk)?;
	let mut file= tokio::fs::File::open(&temp_file_path).await?;

	

	println!("{}", temp_file_path);
	

	let embed = CreateAttachment::file(&file, "plik.txt").await?;

	let message = CreateMessage::new()
	.add_file(embed);

	let _ = channel_id.send_message(&ctx.http, message).await?;

	//let mut msg = channel_id.send_message(&ctx.http, CreateMessage::new().add_file(embed));
	
	//if let Err(why) = msg.await {
	//	println!("Error sending message: {:?}", why);
	//}

	//fs::remove_file(&temp_file_path)?;

	//if let Err(why) = fs::remove_file(&temp_file_path) {
	//	println!("Error removing temporary file: {:?}", why);
	//}

	Ok(())

}


