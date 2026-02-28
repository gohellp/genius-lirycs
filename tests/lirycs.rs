use regex::Regex;

mod common;

#[tokio::test]
async fn fetch_lyrics() {
    let gac = common::setup();

    let lyrics_result = gac.get_lyrics("https://genius.com/Rick-astley-never-gonna-give-you-up-lyrics".to_string()).await;

    assert!(lyrics_result.is_ok(), "{:?}", lyrics_result.err().unwrap());
    let lyrics = lyrics_result.unwrap();
    let regex = Regex::new("(?mis)^.{1,100}\n\n").unwrap();
    let shorter_lyrics_option = regex.find(&lyrics);
    assert!(shorter_lyrics_option.is_some());
    let shorter_lyrics = shorter_lyrics_option.unwrap().as_str().trim();
    assert_eq!(shorter_lyrics, "[Intro]\nDesert you\nOoh-ooh-ooh-ooh\nHurt you")
}
