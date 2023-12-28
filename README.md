# Parsley - Container manifest parser

This crate provides a convenient way to interact with the specifications defined by the
[Moby](https://github.com/moby/moby) project.

- [Image Format Specification](https://github.com/moby/moby/blob/master/image/spec/spec.md)

```toml
[dependencies]
parsley = { git = "https://github.com/placintaalexandru/parsley", version = "0.1.0" }
```
*Compiler support: requires rustc 1.56.0+*.

## Examples

### Image spec

#### Load a manifest from the filesystem

```rust
use parsley::docker::image::ImageManifest;

let image_manifest = ImageManifest::from_file("manifest.json").expect("Could not deserialize from file");
```

#### Create a manifest using the builder

```rust
use parsley::docker::image::ImageManifest;
use parsley::docker::image::ManifestItemBuilder;

let manifest = ImageManifest(vec![ManifestItemBuilder::default()
    .config(
        "ee56d70bcdf1aeca472a9899de653eb4d72f4a3ac31d9b0b95e677488ce766f3.json".to_owned(),
    )
    .repo_tags(vec!["postgres:15.4".to_owned()])
    .layers(vec![
        "3b05311756d94678c1ea8e45bf7665a4e29f850c31c6f58d6c28403c6fdc0cdc/layer.tar"
            .to_owned(),
        "454d82adf13f02e53baeae05d06b595b34bbab2836977c6b679488ec038449c3/layer.tar"
            .to_owned(),
        "c039956656e1c9cd1e2d72dba02179b8d9008e0c0771af344944e218c7dc3351/layer.tar"
            .to_owned(),
    ])
    .build()
    .expect("Manifest Build Item 1")]
);
```

#### Load a config from the filesystem

```rust
use parsley::docker::image::ImageConfiguration;

let image_config = ImageConfiguration::from_file("1bc9978a2dd04fb656d9055670b5beee1c948ca3b65cade7783c2d3bab306141.json").unwrap();
```

#### Create a config using the builder

```rust
use std::collections::HashMap;
use std::time::Duration;
use oci_spec::image;
use parsley::docker::image::ConfigExtensionBuilder;
use parsley::docker::image::HealthcheckConfigBuilder;
use parsley::docker::image::ImageConfigurationBuilder;
use parsley::docker::image::ImageConfigurationExtensionBuilder;

let docker_oci_extension = ImageConfigurationExtensionBuilder::default()
    .config(
        ConfigExtensionBuilder::default()
        .memory(2048_u64)
        .memory_swap(4096_u64)
        .cpu_shares(8_u16)
        .args_escaped(false)
        .shell(vec![
            "/bin/bash".to_owned(),
            "-o".to_owned(),
            "pipefail".to_owned(),
            "-c".to_owned(),
        ])
        .on_build(vec!["a".to_owned(), "b".to_owned()])
        .health_check(
            HealthcheckConfigBuilder::default()
            .test(vec![
                "CMD-SHELL".to_owned(),
                "/usr/bin/check-health localhost".to_owned(),
            ])
            .interval(Duration::from_nanos(30000000000))
            .timeout(Duration::from_nanos(10000000000))
            .start_interval(Duration::from_nanos(3000000000))
            .retries(3_u32)
            .build()
            .expect("Build Docker OCI Extension: Healthcheck"),
        )
        .build()
        .expect("Build Docker Config Extension"),
    )
    .build()
    .expect("Docker OCI Image Extension");
let oci_spec = image::ImageConfigurationBuilder::default()
    .created("2023-08-16T06:40:57.929475525Z")
    .author("author".to_owned())
    .architecture(image::Arch::ARM64)
    .os(image::Os::Linux)
    .config(
        image::ConfigBuilder::default()
        .user("1001".to_owned())
        .exposed_ports(vec!["5432/tcp".to_owned()])
        .env(vec![
            "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/lib/postgresql/15/bin".to_owned(),
            "GOSU_VERSION=1.16".to_owned(),
            "LANG=en_US.utf8".to_owned(),
            "PG_MAJOR=15".to_owned(),
            "PG_VERSION=15.4-1.pgdg120+1".to_owned(),
            "PGDATA=/var/lib/postgresql/data".to_owned()
        ])
        .entrypoint(vec!["docker-entrypoint.sh".to_owned()])
        .labels(HashMap::from_iter([("maintainer".to_owned(),"someone".to_owned())]))
        .stop_signal("SIGINT".to_owned())
        .cmd(vec!["postgres".to_owned()])
        .volumes(vec!["/var/lib/postgresql/data".to_owned()])
        .working_dir("/postgres".to_owned())
        .build()
        .expect("Build Config")
    )
    .rootfs(image::RootFsBuilder::default()
    .typ("layers")
    .diff_ids(vec![
        "sha256:1c3daa06574284614db07a23682ab6d1c344f09f8093ee10e5de4152a51677a1".to_owned(),
        "sha256:310729fcb068da6941441d9627a3d8979e7dbd015c220324331e34af28b7e20c".to_owned(),
        "sha256:6cc6868915f4c4d399ec0026fd321acfd0b92e84cd2a51076e89041b3e3118b6".to_owned()
    ]).build().expect("Rootfs"))
    .history(vec![
        image::HistoryBuilder::default()
            .created("2023-08-15T23:39:57.178505081Z".to_owned())
            .created_by("/bin/sh -c #(nop) ADD file:bc58956fa3d1aff2efb0264655d039fedfff28dc4ff19a65a235e82754ee1cfa in / ".to_owned())
            .build()
            .expect("Build History 1"),
        image::HistoryBuilder::default()
            .created("2023-08-15T23:39:57.574431303Z".to_owned())
            .created_by("/bin/sh -c #(nop)  CMD [\"bash\"]".to_owned())
            .empty_layer(true)
            .build()
            .expect("Build History 2"),
        image::HistoryBuilder::default()
            .created("2023-08-16T06:38:58.796057889Z".to_owned())
            .created_by("/bin/sh -c set -eux; \tgroupadd -r postgres --gid=999; \tuseradd -r -g postgres --uid=999 --home-dir=/var/lib/postgresql --shell=/bin/bash postgres; \tmkdir -p /var/lib/postgresql; \tchown -R postgres:postgres /var/lib/postgresql".to_owned())
            .build()
            .expect("Build History 3")
    ])
    .variant("v8".to_owned())
    .build()
    .expect("OCI Config Spec");

ImageConfigurationBuilder::default()
    .docker_oci_extension(docker_oci_extension)
    .oci_spec(oci_spec)
    .build()
    .expect("Image Config");
```

### Distribution spec

#### Load from the filesystem

```rust
use parsley::docker::distribution::Repositories;

let repositories = Repositories::from_file("repositories").unwrap();
```

# Contributing
This project welcomes your PRs and issues. Should you wish to work on an issue, please claim it first by commenting on the
issue that you want to work on it. This is to prevent duplicated efforts from contributers on the same issue.
