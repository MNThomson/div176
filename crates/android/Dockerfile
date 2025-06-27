FROM ubuntu:24.04

ARG USER=ubuntu
WORKDIR /home/$USER

RUN apt update && apt install -y \
        git neovim wget curl unzip sudo \
        qemu-system qemu-kvm libvirt-daemon-system libvirt-clients xvfb \
        libwebkit2gtk-4.1-dev build-essential file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev && \
    apt clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN adduser $USER libvirt && \
    adduser $USER kvm && \
    echo "$USER:$USER" | chpasswd && \
    echo "${USER} ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/$USER && \
    usermod -aG sudo $USER && \
    usermod -aG plugdev $USER

USER $USER

ARG ANDROID_STUDIO_VERSION="2024.2.1.12"
RUN wget "https://redirector.gvt1.com/edgedl/android/studio/ide-zips/${ANDROID_STUDIO_VERSION}/android-studio-${ANDROID_STUDIO_VERSION}-linux.tar.gz" -O android-studio.tar.gz && \
        tar xzvf android-studio.tar.gz && \
        rm android-studio.tar.gz

ENV ANDROID_HOME /home/ubuntu/Android/Sdk
ENV ANDROID_SDK_ROOT /home/ubuntu/Android/Sdk
ENV JAVA_HOME /home/ubuntu/android-studio/jbr

RUN wget "https://dl.google.com/android/repository/commandlinetools-linux-11076708_latest.zip" -O cmdline-tools.zip && \
    unzip cmdline-tools.zip && \
    yes | cmdline-tools/bin/sdkmanager --sdk_root="$ANDROID_SDK_ROOT" --install "platform-tools" "platforms;android-34" "platforms;android-35" "build-tools;34.0.0" "build-tools;35.0.0" "sources;android-34" "cmdline-tools;17.0" "ndk;28.0.12674087" "emulator" "system-images;android-35;default;x86_64" && \
    rm -rf cmdline-tools*
ENV NDK_HOME /home/ubuntu/Android/Sdk/ndk/28.0.12674087

RUN echo no | Android/Sdk/cmdline-tools/17.0/bin/avdmanager create avd --name "Pixel9" --package "system-images;android-35;default;x86_64" --device "pixel_9_pro"
ENV ANDROID_EMULATOR_USE_SYSTEM_LIBS 1

RUN echo 'export PATH="~/.cargo/bin:$PATH"' >> /home/ubuntu/.bashrc
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y && \
    ~/.cargo/bin/rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
RUN ~/.cargo/bin/cargo install tauri-cli --version '^2.0.0' --locked && \
    ~/.cargo/bin/cargo install create-tauri-app --locked
