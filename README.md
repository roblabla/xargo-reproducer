Reproducer for https://github.com/japaric/xargo/issues/261


```
env RUST_TARGET_PATH=$(pwd) xargo rustc --release --target x86_64-unknown-linux-mygnu -- -C link-args=-nostartfiles
```

Error:

```
   Compiling reproducer v0.1.0 (/home/roblabla/Dropbox/dev/src/rust/reproducer)
error[E0523]: found two different crates with name `nocorelib` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
 --> src/main.rs:4:1
  |
4 | extern crate nocorelib;
  | ^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

error: could not compile `reproducer`.

To learn more, run the command again with --verbose.
```

This reproducer works by having both libcore and the reproducer binary depend on
nocorelib. When building in release, nocorelib will have the same hash, and this
will result in the above error.
