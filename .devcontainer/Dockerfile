FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
	build-essential \
	pkg-config \
	libx11-dev \
	libxkbcommon-dev \
	libwayland-dev \
	libegl1-mesa-dev \
	libgl1-mesa-dev \
	libasound2-dev \
	vulkan-tools \
	libvulkan-dev \
	curl \
	git \
	lldb \
	&& rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup component add rustfmt clippy

WORKDIR /workspaces/${LOCAL_WORKSPACE_FOLDER_BASENAME}
