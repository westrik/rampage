FROM rust:1.36 as build
COPY ./ ./
RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/rampage /build-out/

FROM debian:buster
COPY --from=build /build-out/rampage /
CMD /rampage
