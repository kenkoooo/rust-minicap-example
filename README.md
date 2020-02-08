# minicap example in Rust

## Requirements

* [minicap](https://github.com/openstf/minicap)
* An Android device with USB debugging enabled.

## Running

1. Check that your device is connected and ADB is running with `adb devices`. The following steps may not work properly if you don't.
```
adb devices
```
2. Set up a forward for the server we'll soon have running inside the device. Note that due to laziness the port is currently fixed to 1717.
```
adb forward tcp:1717 localabstract:minicap
```
3. Get information about your display.
```
adb shell wm size
```
4. Start the minicap server. You can see more detail in [openstf/minicap](https://github.com/openstf/minicap).
```
minicap/run.sh -P 720x1280@720x1280/0
```
5. Start the example app.
```
cargo run --example capture_jpg
```
6. `screenshot.jpg` will be created.