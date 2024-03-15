# reliquary

a library to parse network packets from a certain turn based anime game!

| module     | purpose                                      |
|------------|----------------------------------------------|
| `network`  | parse bytes into network packets             |
| `resource` | look up network packet resource ids from SRD |

## use in your projects

not published to crates.io out of caution. add following to your Cargo.toml to use

```toml
[dependencies]
reliquary = { git = "https://github.com/IceDynamix/reliquary" } # optionally add revision
```

for documentation, use `cargo doc`

## codegen

types are outdated? check out [reliquary-codegen](https://github.com/IceDynamix/reliquary-codegen)

## related

- [reliquary-archiver](https://github.com/IceDynamix/reliquary-archiver)