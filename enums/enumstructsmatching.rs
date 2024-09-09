enum Message {
    Text(String),
    Photo{ url: String },
    Video { url: String, duration: u32},
}

impl Message {
    fn summary(&self){
        match self {
            Message::Text(text) => println!("Text: {}", text),
            Message::Photo{ url } => println!("Photo: {}", url),
            Message::Video { url, duration } => {println!("Video: {}, Duration: {} seconds", url, duration)},
        }
    }
}

fn main(){
    let text_msg = Message::Text("Hello, World!".to_string());
    text_msg.summary();

    let photo_msg = Message::Photo{ url: String::from ("example.com/photo.jpg")};
    photo_msg.summary();

    let video_msg = Message::Video{ url: String::from("example.com/video.mp4"), duration: 120};
    video_msg.summary();
}