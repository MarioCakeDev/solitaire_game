#!/bin/bash

ANDROID_HOME="$HOME/Android/Sdk"
NDK_BASE_DIR="$ANDROID_HOME/ndk"
echo "NDK Base Dir: $NDK_BASE_DIR"
NDK_VERSION=$(ls "$NDK_BASE_DIR" | sort -Vr | fzf --select-1)
#ANDROID_JNILIBS_DIR="mobile/android/app/src/main/jniLibs"

ANDROID_NDK_HOME="$NDK_BASE_DIR/$NDK_VERSION"
#ANDROID_NDK_HOME="$ANDROID_NDK_HOME" cargo ndk -t arm64-v8a tree
#ANDROID_NDK_HOME="$ANDROID_NDK_HOME" cargo ndk -t arm64-v8a -o "$ANDROID_JNILIBS_DIR" build
NDK_DEBUG=1 ANDROID_NDK_HOME="$ANDROID_NDK_HOME" ANDROID_HOME="$ANDROID_HOME" cargo apk run -p mobile
