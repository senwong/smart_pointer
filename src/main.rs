

struct Song {
    name: String,
    author: String,
}

async fn learn_song() -> Song {
    Song {
        name: String::from("菊花台"),
        author: String::from("周杰伦"),
    }
}

async fn sing_song(song: &Song){
    println!("{}唱了{}", song.author, song.name);
}

async fn dance() {
    println!("dance dance");
}

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use futures::executor::block_on;

async fn hello_cat() {
    println!("hello cat!");
}

async fn do_something() {
    hello_cat().await;
    println!("gogogogo");
}

fn main() {
    let song = block_on(learn_song());
    block_on(sing_song(&song));
    block_on(dance());
}
