## Marv Bot

<img src="https://i.pinimg.com/474x/c4/db/8d/c4db8d7643fcd1319b918397c57cfebc.jpg"
 alt="Marv" title="The man himself" align="right" height="420px" />

Your rust Awesome IRC Bot, it's a personal project mainly used to play with Rust Language

### Todo

- [x] modules
- [x] closures
- [x] kafka / produce
- [x] kafka / consume
- [x] database / orm (diesel / sqlx)
- [x] monads
- [x] variables - multi assignements
- [x] write something to the fs
- [x] bot single thread
- [x] dotenv
- [x] singleton (lazy_static?)
- [x] plugin contract should return Result (stop using :expect)
- [x] error handling vs monads (unwrap, Option, Result, ?, etc)
- [x] 'awesome rust' (and produtivity stuff like libs and tools)
- [x] singleton with lazy (once_cell)
- [x] custom types (to :return signature and :parameters)
- [x] threads / mutex - network-io and plugin dispatch
- [x] evented-io
- [x] tests / unit
- [x] kafka: rdkafka: consumer / tokio
- [x] kafka: rdkafka: producer / tokio
- [x] kafka: serialize messages with: cbor, protobuf, messagepack
- [x] housekeeping: remove all unsafe :unwrap and :expect from the code
- [x] kafka: remove rust-kafka
- [x] kafka: up service to the latest version / kafka-native
- [x] plugins: plugin sync vs async
- [x] async
- [x] threads / mutex - individual plugin execution
- [x] bot muilti thread
- [x] bot evented / async
- [x] bot actor model
- [x] bot even driven
- [x] a way to check for blocking code (tokio flags and log)
- [x] rust: use once_cell/Lazy instead of lazy_static!
- [x] thread pool
- [x] config: read file using tokio
- [ ] monorepo
- [ ] marv using event driven
- [ ] data-parallelism / rayon
- [ ] plugin: that track the execution time
- [ ] errors: play with anyhow
- [ ] macro: create a macro that wrap function and add execution time
- [ ] dispatch: static vs dynamic (templates vs dyn)
- [ ] use plugin as crate / external modules
- [ ] config: custom segments for plugins
- [ ] prometheus / metrics / aumentar cobertura
- [ ] tracing
- [ ] debug
- [ ] build (using docker and pipelines)
- [ ] cross platform build (arm, x86, windows, linux, macos)
- [ ] argparse
- [ ] housekeeping
- [ ] binstall
- [ ] FFI
- [ ] change log level by using signals
- [ ] logs: add debug messages
- [ ] logs: make it possible to change log level by using signals

### Plugins

#### Core

- [x] Ping
- [x] Channel / Join
- [x] Log

#### Extras

- [x] kafka consume
- [x] kafka produce
- [x] database
- [x] todo (create a todolist)
- [ ] default ask chatgpt
