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
- [x] kafka: rdkafka: consumer / tokio
- [ ] kafka: rdkafka: producer / tokio
- [ ] plugins: plugin sync vs async
- [ ] async
- [ ] threads / mutex - individual plugin execution
- [ ] data-parallelism / rayon
- [ ] thread pool
- [ ] macros
- [ ] dispatch: static vs dynamic (templates vs dyn)
- [ ] actor model
- [ ] use plugin as crate / external modules
- [ ] config: custom segments for plugins
- [ ] prometheus / metrics / aumentar cobertura
- [ ] tracing
- [ ] debug
- [ ] tests
- [ ] bot muilti thread
- [ ] bot evented / async
- [ ] bot actor model
- [ ] bot even driven
- [ ] build (using docker and pipelines)
- [ ] cross platform build (arm, x86, windows, linux, macos)
- [ ] argparse
- [ ] housekeeping
- [ ] binstall
- [ ] kafka: serialize messages with: cbor, protobuf, messagepack
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
