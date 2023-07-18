##  Overview 
<p>
A TCP echo server is a fairly simple concept - a client sends data to a listener server, and the listener sends the same data immediately back to the client. In addition to the echo server, this project contains a client server using standard in and out. So, the end result is the client types into their terminal and the same text is then printed out again. It looks like this:
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
