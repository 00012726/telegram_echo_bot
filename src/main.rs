use telegram_bot::*;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {

    //bot token
    let token: &str = "API_TOKEN";
    let api: Api = Api::new(token);

    //updating stream
    let mut stream: UpdatesStream = api.stream(); 

    while let Some(update) = stream.next().await 
    {
        let update: Update = update?;

        //checking if the update is a message
        if let UpdateKind::Message(message) = update.kind 
        {
            if let MessageKind::Text {ref data, .. } = message.kind 
            {
                println!("Received message is: {}", data);

                //send message back to user in bot
                api.send(message.text_reply(data)).await?;
            }
        }
    }

    Ok(())
}
