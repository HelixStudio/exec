# exec

Yet another _blazingly fast_ code execution engine written in Rust.

## Paths

- GET /api/v1/status
- GET /api/v1/runtimes
- POST /api/v1/execute
- POST /api/v1/test

Check the `models.rs` file for request & response models.

To run code within a container for each request, the server must be started using the `--privileged` argument, like this:

```bash
docker run --privileged -p 8080:8080 stefanasandei/exec:0.2
```

This is required by the `clone` syscall.

![benchmark](https://media.discordapp.net/attachments/1112091519269212351/1127951335459917955/image.png)

## Supported languages

- C
- C++
- Rust
- Haskell

## License

Copyright 2023 Asandei Stefan-Alexandru. All rights reserved.
