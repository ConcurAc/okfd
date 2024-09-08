default:
	cargo build -r

linux:
	cargo build -r --target x86_64-unknown-linux-gnu

windows:
	cargo build -r --target x86_64-pc-windows-gnu

mac:
	cargo build -r --target aarch64-apple-darwin

android:
	cargo ndk -t arm64-v8a -o ./android/app/src/main/jniLibs build -r
	android/gradlew build -p android

clean:
	cargo clean
	android/gradlew clean -p android

