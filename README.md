# reliquary

a library to parse network packets from a certain turn based anime game!

| module     | purpose                                          |
|------------|--------------------------------------------------|
| `sniffer`  | parse bytes into network packets                 |
| `resource` | look up network packet resources ids             |
| `export`   | transform data to different known output formats |

## use in your projects

not published to crates.io out of caution. add following to your Cargo.toml to use

```toml
[dependencies]
reliquary = { git = "https://github.com/IceDynamix/reliquary" }
```

## codegen

types are outdated? check out [reliquary-codegen](https://github.com/IceDynamix/reliquary-codegen)