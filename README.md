# genius-lyrics

genius_lyrics is a library that can find information about a track and try to extract the lyrics from the genius website

## Features

- Gets track info
- Search track on Genius
- Gets track's lyrics

## Install a crate

1. Open a command line and switch to the directory that contains your project file.

2. Use the following command to install a NuGet package:

    ```powershell
    cargo add genius_lyrics
    ```

    After the command completes, you can open the project file to see the package reference.

    For example, open the `Cargo.toml` file to see the added `genius_lyrics` package reference:

    ```toml
    [dependencies]
    genius_lyrics = "0.0.1"
    ```

## Usage

```csharp
use genius_lyrics::GeniusApiClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = GeniusApiClient::new("REPLACE_ME_GeniusToken");

    let song = client.get_song_by_id(84851).await?;

    let lyrics = client.get_lyrics(song.url).await?;

    println!("{}", lyrics);

    Ok(())
}
```

You can get your Genius client access token [here](https://genius.com/api-clients).

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Thanks

Thank you very much [farshed](https://github.com/farshed) for writing a similar library for JavaScript. If it wasn't for the [genius-lyrics-api](https://github.com/farshed/genius-lyrics-api), I wouldn't have come up with the idea of rewriting it in [C#](https://www.nuget.org/packages/GeniusLyricsAPI) and Rust :D
