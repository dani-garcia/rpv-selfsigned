mkdir -p app/app/src/main/jniLibs/arm64-v8a

cross build -p rpv-selfsigned --target=aarch64-linux-android
mv target/aarch64-linux-android/debug/librpv_selfsigned.so app/app/src/main/jniLibs/arm64-v8a/
