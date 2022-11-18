# Sudoku

Wanted a simple game in bevy to see how fast I can create a complete yet simple game.


# Setup

sudo apt-get install lld



## Android Setup Notes

Add targets
```
rustup target add x86_64-linux-android
rustup target add aarch64-linux-android

Add repository for android studio
```
sudo add-apt-repository ppa:maarten-fonville/android-studio
sudo apt-get update
```

Install java jdk
```
sudo apt install openjdk-11-jdk
```

Install Android Studio
```
sudo apt-get install android-studio
```

Start android-studio and install default configuration.

Install cargo-apk
```
cargo install cargo-apk
```

Add paths for cargo-apk, using full path since I had issues, should be cleaner way.
```
export ANDROID_HOME="/home/<USER>/Android/Sdk"
export ANDROID_NDK_ROOT="/home/<USER>/Android/Sdk/ndk/25.1.8937393"
```

Add Platform 30

