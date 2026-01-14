# Use a Rust base image with Cargo installed
FROM rust:1.80.0-slim-bullseye AS build

# Set the working directory for build
WORKDIR /psp

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Now copy the source code
COPY ./src ./src

# Build your application
RUN cargo build --release

# Create a new stage for running the application
FROM python:3.14-slim AS final

# Install psp dependencies
RUN apt-get -y update && apt-get install -y --no-install-recommends git curl

# Set the working directory for final
WORKDIR /psp

# Copy psp executable from the "build" stage.
COPY --from=build /psp/target/release/psp /bin/

# Command to run the application
ENTRYPOINT ["/bin/psp"]
