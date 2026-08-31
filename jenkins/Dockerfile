FROM jenkins/jenkins:2.579-jdk21

USER root

RUN apt-get update \
    && apt-get install --yes --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        git \
        gnupg \
        libssl-dev \
        pkg-config \
    && install -m 0755 -d /etc/apt/keyrings \
    && curl --fail --silent --show-error \
        https://download.docker.com/linux/debian/gpg \
        --output /etc/apt/keyrings/docker.asc \
    && chmod a+r /etc/apt/keyrings/docker.asc \
    && . /etc/os-release \
    && printf '%s\n' \
        "Types: deb" \
        "URIs: https://download.docker.com/linux/debian" \
        "Suites: ${VERSION_CODENAME}" \
        "Components: stable" \
        "Architectures: $(dpkg --print-architecture)" \
        "Signed-By: /etc/apt/keyrings/docker.asc" \
        > /etc/apt/sources.list.d/docker.sources \
    && apt-get update \
    && apt-get install --yes --no-install-recommends \
        docker-buildx-plugin \
        docker-ce-cli \
        docker-compose-plugin \
    && rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:${PATH}3

RUN curl --proto '=https' --tlsv1.2 --fail --silent --show-error \
        https://sh.rustup.rs \
        --output /tmp/rustup-init.sh \
    && sh /tmp/rustup-init.sh \
        -y \
        --profile minimal \
        --default-toolchain 1.94.1 \
    && rustup component add clippy rustfmt \
    && rm /tmp/rustup-init.sh \
    && chown -R jenkins:jenkins "${RUSTUP_HOME}" "${CARGO_HOME}"

COPY jenkins/plugins.txt /usr/share/jenkins/ref/plugins.txt
RUN jenkins-plugin-cli --plugin-file /usr/share/jenkins/ref/plugins.txt

USER jenkins
