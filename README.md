# Flutter Rust Metal

Simple PoC to use Rust bindings in Flutter via [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) and use the underlying metal shader GPU in iOS to perform a simple calculation

## Generating Metal Shaders Library

In the rs_devices/src folder run 
```
xcrun -sdk iphoneos metal -c shaders.metal -o shaders.air
```
then 
```
xcrun -sdk iphoneos metallib shaders.air -o shaders.metallib
```

Make sure this file is included in the iOS asset folder in XCode
