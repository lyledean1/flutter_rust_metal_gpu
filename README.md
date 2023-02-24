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

## Getting Started

This project is a starting point for a Flutter application.

A few resources to get you started if this is your first Flutter project:

- [Lab: Write your first Flutter app](https://docs.flutter.dev/get-started/codelab)
- [Cookbook: Useful Flutter samples](https://docs.flutter.dev/cookbook)

For help getting started with Flutter development, view the
[online documentation](https://docs.flutter.dev/), which offers tutorials,
samples, guidance on mobile development, and a full API reference.
