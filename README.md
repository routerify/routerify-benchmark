# routerify-benchmark

A simple benchmark between other web frameworks and router implementation

## Comparisons

### Web Frameworks


| Framework      | Language    | Requests/sec |   |   |
|----------------|-------------|--------------|---|---|
| hyper v0.13    | Rust 1.43.0 | 112557.51    |   |   |
| routerify v1.0 | Rust 1.43.0 | 105598.51    |   |   |
|                |             |              |   |   |

cols: Framework, Language, Requests/sec

### Router Implementation Library

cols: Framework, Language, Requests/sec


results
---------
go-httprouter[master branch, go1.13.7 darwin/amd64]: 
Requests/sec:  74958.44

hyper[v0.13, rustc 1.43.0]:
Requests/sec: 112557.51

routerify[v1.0] with hyper.rs[v0.13, rustc 1.43.0]:
Requests/sec: 105598.51

gotham[v0.4.0, rustc 1.43.0]:
Requests/sec: 100097.73

actix-web[v2, rustc 1.43.0]
Requests/sec:  96397.31

warp[v0.2, rustc 1.43.0]
Requests/sec:  81912.45

Rocket[branch async, rustc 1.43.0]
Requests/sec:   2041.93