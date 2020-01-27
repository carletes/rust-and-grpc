# Experiments in Rust with gRPC

Expose a [CRI endpoint](https://github.com/kubernetes/cri-api/blob/master/pkg/apis/runtime/v1alpha2/api.proto).

Rust gRPC implementations (early 2020):

* [Tonic](https://github.com/hyperium/tonic). A native gRPC client & server
  implementation with async/await support. The next generation of `tower-grpc`".
  Uses [rustls](https://github.com/ctz/rustls), so might be easier to use in
  statically-linked projects.
* [Tower gRPC](https://github.com/tower-rs/tower-grpc). A gRPC client and server
  implementation. Superceeded by Tonic, it seems.
* [gRPC-rs](https://github.com/tikv/grpc-rs). A Rust wrapper of [gRPC
  Core](https://github.com/grpc/grpc). Might be difficult to use in
  statically-linked projects.

Related Rust crates:

* [PROST!](https://github.com/danburkert/prost). Protocol Buffers implementation
  for the Rust Language.
