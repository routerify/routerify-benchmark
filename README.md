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
| [hyper v0.14](https://github.com/hyperium/hyper) | Rust 1.50.0 | 144,583 |
| [routerify v2.0.0-beta-4](https://github.com/routerify/routerify) with [hyper v0.14](https://github.com/hyperium/hyper) | Rust 1.50.0 | 144,621 |
| [actix-web v3](https://github.com/actix/actix-web) | Rust 1.50.0 | 131,292 |
| [warp v0.3](https://github.com/seanmonstar/warp) | Rust 1.50.0 | 145,362 |
| [go-httprouter, branch master](https://github.com/julienschmidt/httprouter) | Go 1.16 | 130,662 |
| [Rocket, branch master](https://github.com/SergioBenitez/Rocket) | Rust 1.50.0 | 130,045 |

## Contributing

Your PRs and suggestions are always welcome.
