mod common;

#[tokio::test]
async fn fetch_song_by_options_with_auth_header() {
    let gac = common::setup();

    let song_result = gac.get_song(
        "Rick Astley",
        "Never Gonna Give You Up",
        None
    ).await;

    assert!(song_result.is_ok(), "{:?}", song_result.err().unwrap());
    let song = song_result.unwrap();
    assert_eq!(song.primary_artist.name, "Rick Astley");
    assert_eq!(song.title, "Never Gonna Give You Up");
}

#[tokio::test]
async fn fetch_song_by_options_with_auth_argument() {
    let gac = common::setup();

    let song_result = gac.get_song(
        "Rick Astley",
        "Never Gonna Give You Up",
        Some(true)
    ).await;

    assert!(song_result.is_ok(), "{:?}", song_result.err().unwrap());
    let song = song_result.unwrap();
    assert_eq!(song.primary_artist.name, "Rick Astley");
    assert_eq!(song.title, "Never Gonna Give You Up");
}

#[tokio::test]
async fn fetch_song_by_id() {
    let gac = common::setup();

    let song_result = gac.get_song_by_id(84851).await;

    assert!(song_result.is_ok(), "{:?}", song_result.err().unwrap());
    let song = song_result.unwrap();
    assert_eq!(song.primary_artist.name, "Rick Astley");
    assert_eq!(song.title, "Never Gonna Give You Up");
}
