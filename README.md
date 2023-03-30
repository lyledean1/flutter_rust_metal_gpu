# Flutter Rust Metal

PoC to use Rust bindings in Flutter via [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) and use the underlying metal shader GPU in iOS to perform a simple calculation

[See code here](https://github.com/lyledean1/flutter_rust_metal_gpu/blob/0446bb38fdb8d2513e1a1b97c9b033e5e996fd73/rs_devices/src/api.rs#L26)

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
