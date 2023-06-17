ARG APP=sofar-mqtt

FROM --platform=$BUILDPLATFORM tonistiigi/xx AS xx

FROM --platform=$BUILDPLATFORM rust:1.70-alpine as builder

# Copy xx scripts to build stage
COPY --from=xx / /

# Install build deps
RUN apk add clang lld
RUN apk add --no-cache musl-dev

# Setup envs
ARG APP

# Set working directory
WORKDIR /usr/src

# Create dummy project
RUN cargo new $APP

COPY Cargo.toml Cargo.lock /usr/src/$APP
COPY .cargo /usr/src/$APP/.cargo

# Set working directory
WORKDIR /usr/src/$APP

ARG TARGETPLATFORM

# Build project to cache dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    xx-cargo build --release

# Copy project files
COPY src /usr/src/$APP/src

# Touch main.rs to prevent cached release build
RUN touch /usr/src/$APP/src/main.rs

# Build real project
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    xx-cargo build --release && \
    xx-verify /usr/src/$APP/target/$(xx-cargo --print-target-triple)/release/$APP

RUN cp /usr/src/$APP/target/$(xx-cargo --print-target-triple)/release/$APP ./target

FROM alpine:3.18 AS runtime
ARG APP
ENV APP ${APP}

# Copy application binary from builder image
COPY --from=builder /usr/src/$APP/target/$APP /usr/local/bin

# Expose app port
EXPOSE 8080

# Run the application
CMD /usr/local/bin/${APP}