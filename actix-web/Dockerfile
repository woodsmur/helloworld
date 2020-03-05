FROM rust:1.41.1-stretch as builder
# Choose a workdir
WORKDIR /mybuild
# Create blank project
RUN USER=root cargo init
# Copy Cargo.toml to get dependencies
COPY Cargo.toml .
# This is a dummy build to get the dependencies cached
RUN cargo build --release
# Copy sources
COPY src src
# Build app (bin will be in /mybuild/target/release/actix-web)
RUN cargo build --release

FROM frolvlad/alpine-glibc:alpine-3.11_glibc-2.30
# Copy bin from builder to this new image
COPY --from=builder /mybuild/target/release/actix-web /bin/

EXPOSE 8080

# Default command, run app
# CMD ["/bin/actix-web"]
ENTRYPOINT [ "actix-web" ]
