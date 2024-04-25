# Juice Project Status

An overview page of all repositories involved in the juice framework:

## Crates

Since the integration into the workspace repo, currently there is only the overarching project status available.

| Project          | Status |
| ---------------- | --- |
| juice (default) | [![Build Status juice (default)](https://ci.fff.rs/api/v1/teams/fff/pipelines/juice/jobs/test-juice-fedora-default/badge)](https://ci.fff.rs/teams/fff/pipelines/juice) |
| juice (native) | [![Build Status juice (native)](https://ci.fff.rs/api/v1/teams/fff/pipelines/juice/jobs/test-juice-fedora-native/badge)](https://ci.fff.rs/teams/fff/pipelines/juice) |
| juice (cuda) | [![Build Status juice (cuda)](https://ci.fff.rs/api/v1/teams/fff/pipelines/juice/jobs/test-juice-fedora-cuda/badge)](https://ci.fff.rs/teams/fff/pipelines/juice) |

## Base

All crates are tested per PR on these base images, which is currently limted to [latest fedora version](https://getfedora.org).

| Container | Status |
| --- | --- |
| fedora-base | [![Build Status Base](https://ci.fff.rs/api/v1/teams/fff/pipelines//juice-containers/jobs/create-oci-fedora-base/badge)](https://ci.fff.rs/teams/fff/pipelines/juice-containers/jobs/create-oci-fedora-base) |
| fedora-native | [![Build Status Native](https://ci.fff.rs/api/v1/teams/fff/pipelines//juice-containers/jobs/create-oci-fedora-native/badge)](https://ci.fff.rs/teams/fff/pipelines/juice-containers/jobs/create-oci-fedora-native) |
| fedora-cuda | [![Build Status Cuda](https://ci.fff.rs/api/v1/teams/fff/pipelines//juice-containers/jobs/create-oci-fedora-cuda/badge)](https://ci.fff.rs/teams/fff/pipelines/juice-containers/jobs/create-oci-fedora-cuda) |
| fedora-default | [![Build Status Default](https://ci.fff.rs/api/v1/teams/fff/pipelines//juice-containers/jobs/create-oci-fedora-default/badge)](https://ci.fff.rs/teams/fff/pipelines/juice-containers/jobs/create-oci-fedora-default) |

The containers are available from the OCI registry [quay.io](https://quay.io/organization/fff) by adding
a prefix of `container-` to the above table.

To regenerate the pipelines after a change to anything in the `templates` dir, make sure to regenerate with `cargo run`, which will spawn `3` files in the root dir of this repository: `juice.yml`, `juice-containers.yml`, and `juice-crashtest.yml`.
