# archify

In dev.

# TODO

- Add option for log level
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