FROM almalinux:9.5-20250411

# Install tools and dependencies
# buildifier (Dockerfile): golang
# cc (crates.io): clang
# Reindeer (Dockerfile): openssl-devel
RUN dnf update -y \
    && dnf group install -y "Development Tools" \
    && dnf install -y glibc-langpack-en sudo tzdata \
    && dnf install -y clang golang openssl-devel

# Create and switch to user
ARG USERNAME
ARG USER_UID
RUN useradd -m -s /bin/bash -u $USER_UID $USERNAME \
    && mkdir -p /etc/sudoers.d \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME
USER $USERNAME

# Install Rust with nightly-2025-02-16 toolchain (required by Buck2)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly-2025-02-16 -y
ENV PATH="/home/$USERNAME/.cargo/bin:$PATH"

# Install Buck2 (PR facebook/buck2#929)
RUN cargo +nightly-2025-02-16 install --git https://github.com/yxdai-nju/buck2.git --rev 7cda182 buck2

# Install Reindeer (PR facebookincubator/reindeer#74)
RUN cargo +nightly-2025-02-16 install --git https://github.com/yxdai-nju/reindeer.git --rev c3fbe53 reindeer

# Install buildifier
RUN go install github.com/bazelbuild/buildtools/buildifier@latest
ENV PATH="$PATH:/usr/local/go/bin"

# Create and set permissions for workspace directory
USER root
RUN mkdir -p /workspace && chown $USERNAME:$USERNAME /workspace
USER $USERNAME

WORKDIR /workspace

# ENV TZ=Asia/Shanghai
ENV LANG=en_US.UTF-8
ENV LC_ALL=en_US.UTF-8
