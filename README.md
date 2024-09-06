# openvr-driver-sys

Contains function definitions for the [OpenVR](https://github.com/ValveSoftware/openvr) driver sdk.

## Instructions for updating OpenVR

1. `git submodule update --init --recursive` (initial checkout only)
1. `git submodule foreach git pull origin master` to update the submodule
1. `cargo build --features "buildtime_bindgen"` to update the bindings
