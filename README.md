# bitmax-rs
bitmax.io rust client

See `examples/request.rs` for REST API usage example. You can use `cargo run --example request` to run it,
note that the example uses `BITMAX_PRIVATE` and `BITMAX_PUBLIC` environmental variables for your private
and public Bitmax API keys respectively. `examples/websocket.rs` contains usage example for the websocket API.

# Status:
Only Cash/Margin API is implemented, Futures API is not supported at the moment.

REST API is almost complete, with one exception of placing batch orders.

Websocket subscriptions are almost complete, with one exception of order/balance subscription/messages.

A foundation is laid for websocket requests, but only some are implemented (and REST API is preffered over them, for now).

Experimental APIs are not supported.

You can find Bitmax API documentation [here](https://bitmax-exchange.github.io/bitmax-pro-api).
