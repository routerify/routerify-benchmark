# routerify-benchmark

A simple benchmarking between other web frameworks and router implementation libraries.

## Benchmarking Tool

I used [wrk](https://github.com/wg/wrk) to test these http servers:

```sh
$ wrk --latency -t4 -c200 -d8s http://127.0.0.1:8081
```

## System

I ran all those tests on:

```txt
MacBook Pro, 2.2 GHz Intel Core i7 processor with 4 cores, 16GB of RAM
```

## Comparisons

| Framework      | Language    | Requests/sec |
|----------------|-------------|--------------|
| [hyper v0.13](https://github.com/hyperium/hyper) | Rust 1.43.0 | 112,557 |
| [routerify v1.0](https://github.com/routerify/routerify) with [hyper v0.13](https://github.com/hyperium/hyper) | Rust 1.43.0 | 112,320 |
| [gotham v0.4.0](https://github.com/gotham-rs/gotham) | Rust 1.43.0 | 100,097 |
| [actix-web v2](https://github.com/actix/actix-web) | Rust 1.43.0 | 96,397 |
| [warp v0.2](https://github.com/seanmonstar/warp) | Rust 1.43.0 | 81,912 |
| [go-httprouter, branch master](https://github.com/julienschmidt/httprouter) | Go 1.13.7 | 74,958 |
| [Rocket, branch async](https://github.com/SergioBenitez/Rocket) | Rust 1.43.0 | 2,041 ? |

## Contributing

Your PRs and suggestions are always welcome.
