# Demo of GPIO Zero-Cost Abstractions for the STM32F303

This is a sprint towards a bare-metal implementation of a primitive subset of a PAC or HAL, but nothing more than a personal playground to better understand the concepts laid down in the [Embedded Rust Book](https://docs.rust-embedded.org/book/intro/index.html). It is executing on the linux host, of course.


There is a typesafe, threadsafe `static mut` constant [`BOARD`](src/board.rs), which is implemented as a singleton according to the concepts explained in the [Embedded Rust Book](https://docs.rust-embedded.org/book/peripherals/singletons.html).

This `Board` type carefully manages an array of 96 `Option(())` marker flags, one for each GPIOx pins of the STM32F303 MCU. Clients of this code can obtain a singleton of a `Port` instance, which cannot be used for anything else but creating one of seven readily configured input or output ports. This is achieved by mapping the identifiers of (GpioId, Pin) to an index in the array and calling the `take()` method of the `Option`, which sets the array entry for that port from `Some(())` to `None`. Trying to regain this particular port again results in returning an `Err()`.

Yes, this is incredibly rudimentary and error-prone, and an associative data structure would be much less troublesome. But we don't want to use a heap. One of the types from the `heapless` crate comes to mind. However, the memory footprint of this would be roughly **ten times** than that of the naked array.

The port variants are:

| I/O Mode | Pin Mode  | Output Mode |
|----------|-----------|-------------|
| Input    | Floating  | -- |
|          | Pull Up   | -- |
|          | Pull Down | -- |
| Output   | Pull Up   | PushPull |
|          |           | OpenDrain |
|          | Pull Down | PushPull |
|          |           | OpenDrain |


For these distinctions, a set of generic implementations, marker structs, marker traits are used, which constitute zero-cost abstractions in the runtime. The idea is that it is impossible to create other combinations than shown above or to use non-sensical methods on the variants (such as setting a pin) on an input or reading an output pin. 

This concept is explained in the [Embedded Rust Book](https://docs.rust-embedded.org/book/static-guarantees/design-contracts.html)

In a real world application, when creating these variants, the appropriate bits would need to be set in the GPIOx port control registers, which we have only indicated but omitted. For this, the appropriate set of `Register`s are provided to ports. The "application" only creates ports it actually uses. The resulting memory footprint would be minimal, since the entire complex type system disappears from the binary.
