mod common;

#[tokio::test]
async fn search_song_with_auth_headers() {
    let gac = common::setup();

    let hits_result = gac.search_song(
        "Rick Astley",
        "Never Gonna Give You Up",
        None
    ).await;

    assert!(hits_result.is_ok(), "{:?}", hits_result.err().unwrap());
    let hits = hits_result.unwrap();
    assert!(hits.len() > 0);
    let song = &hits[0].song;
    assert_eq!(song.artist.name, "Rick Astley");
    assert_eq!(song.title, "Never Gonna Give You Up");
}

#[tokio::test]
async fn search_song_with_auth_argument() {
    let gac = common::setup();

    let hits_result = gac.search_song(
        "Rick Astley",
        "Never Gonna Give You Up",
        Some(true)
    ).await;

    assert!(hits_result.is_ok(), "{:?}", hits_result.err().unwrap());
    let hits = hits_result.unwrap();
    assert!(hits.len() > 0);
    let song = &hits[0].song;
    assert_eq!(song.artist.name, "Rick Astley");
    assert_eq!(song.title, "Never Gonna Give You Up");
}
