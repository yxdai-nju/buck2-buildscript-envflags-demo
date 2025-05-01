# buildscript `cargo:rustc-env` demo

This is a demo repository related to facebook/buck2#929 and facebookincubator/reindeer#74.

Created for verifying whether the support for build scripts' `cargo:rustc-env` outputs works correctly.

## Usage

**1. Clone this repository**

```bash
git clone https://github.com/yxdai-nju/buck2-buildscript-envflags-demo.git
```

**2. Open the folder with VS Code Dev Containers**

Open the `buck2-buildscript-envflags-demo` folder with VS Code, then `"Reopen in Container"`.

With `.devcontainer/Dockerfile`, the following components are installed:

- Buck2 from PR facebook/buck2#929
- Reindeer from PR facebookincubator/reindeer#74
- NativeLink for remote execution

With `.buckconfig` and `platforms/{BUCK,defs.bzl}`, Buck2 is configured is to use NativeLink for RE.

NativeLink will start automatically as soon as the Dev Container is up.

**3. Run this command**

```bash
./run_reindeer.sh && buck2 run //project:my_bin
```

**4. Check the results:**

First, check the command-line output. It should build and run successfully:

```console
[rust@xxxxxxxxxxxx workspace]$ buck2 run //project:my_bin
...
BUILD SUCCEEDED - starting your binary
SHA-256 hash of "Hello, Ring!": c9fd0e7810009909ca8a251dc646527718c240f2ca7ce7d220d2a80675db1397
MIME type of 'example.txt': text/plain
```

Second, examine the content of `project/BUCK`, crates that have `buildscript.run = true` fixups should have a `env_flags = ...` line in their `cargo.rust_library` target definitions, like:

```starlark
cargo.rust_library(
    name = "mime_guess-2.0.5",
    # ...
    env_flags = ["@$(location :mime_guess-2.0.5-build-script-run[env_flags])"],
    # ...
)
```

Third, run `cat $(buck2 build //project:mime_guess-2.0.5-build-script-run[env_flags] --show-simple-output)`. The desired outcome is:

```
--path-env=MIME_TYPES_GENERATED_PATH=buck-out/v2/gen/root/[hash]/project/__mime_guess-2.0.5-build-script-run__/OUT_DIR/mime_types_generated.rs
```

Furthermore, you can run `cat $(buck2 build //project:ring-0.17.8-build-script-run[env_flags] --show-simple-output)`. The desired outcome is:

```
--env=RING_CORE_PREFIX=ring_core_0_17_8_
```

If you see these outcomes, it indicates that the build script runner is correctly collecting the `cargo:rustc-env` outputs as "--env" or "--path-env" flags.

## Structure

- `.devcontainer` sets up VS Code Dev Container
- `.devcontainer/{devcontainer.json,nativelink_config.json5}` configure NativeLink
- `project` is a Rust workspace root
- `project/crates/my_bin` is a workspace member, providing a binary target that depends on `mime_guess` and `ring`
- `project/fixups/**/fixups.toml` are fixups for crates.io crates
- `project/{Cargo.toml,reindeer.toml}` set up Reindeer

The `my_bin` crate depends on `mime_guess` and `ring`, both have `cargo:rustc-env` outputs during their build script execution. Their successful build depends on the environment variables set by build scripts' `cargo:rustc-env` outputs working correctly, which this repo intends to verify.
