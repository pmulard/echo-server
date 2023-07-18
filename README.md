##  Overview 
<p>
A TCP echo server actually consists of two servers, a client and a listener, which transmit data back and forth. It is a fairly simple concept - the client sends data to the listener, and the listener sends the same data immediately back to the client. This server uses standard in and out, so the end result is the client types into their terminal and the same text is then printed out. It looks like this:
</p>

```bash
Hello world
Hello world
```

## Running the project

From the project directory, start by running the listener server,

```bash
cargo run
```

To start the client, the binary file must be created first. Open a second terminal and run,
```bash
rustc /src/client.rs
```

Afterwards, the binary can be run directly,
```bash
./client.exe
```
