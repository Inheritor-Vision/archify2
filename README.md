# archify

In dev.

# TODO

- Use rspotify everywhere!
- Add option for log level
- Log change format
- For now only X first song of a playlist are recoved, check API limitation and do multiple requests
- Do not store the JSON output, but a siplified one (like only an array of ids)
- Change how secrets are handled (check GITHUB secrets)
- Store playlist name
- Add an option to list all playlists monitored
- See how it works for radios

## Debug

```Rust
   env_logger::init();
    error!("{}", "And error occured");
    warn!("{:#?}", "This is important");
    info!("{:?}", "Take note");
    debug!("Something weird occured: {}", "Error");
```