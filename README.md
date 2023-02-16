# archify

In dev.

# TODO

- Use rspotify everywhere!
- rspotify "PlaylistId" might be able to parse URL. Not usre abou it. Doc unclear.
- Certificate has been reimplemented for proxy. Check archify first version. It might be impossible to use it because we don't use reqwest directly, but through rspotify. Check how OAuth & CLient credential struct are created (and  what are there fields).
- Add option for log level
- Log change format
- For now only X first song of a playlist are recoved, check API limitation and do multiple requests
- Do not store the JSON output, but a siplified one (like only an array of ids)
- Change how secrets are handled (check GITHUB secrets)
- Store playlist name
- Add an option to list all playlists monitored
- Add old playlist to user playlists
- See how it works for radios
- Change how proxy works (not a feature but a CLI option, but don't use "from env" of rspotify but set it manually). Same for others rspotify struct, set them manually, not with env.

## Debug

```Rust
   env_logger::init();
    error!("{}", "And error occured");
    warn!("{:#?}", "This is important");
    info!("{:?}", "Take note");
    debug!("Something weird occured: {}", "Error");
```
