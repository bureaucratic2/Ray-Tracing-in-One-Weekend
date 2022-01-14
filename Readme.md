## Ray Tracing in One Weekend
This is an implementation of [Peter Shirley's "Ray Tracing In One Weekend"](https://raytracing.github.io/books/RayTracingInOneWeekend.html) book in Rust after a painful C++ implementation experience.

Although I wrap almost all random number function calls in library, for performance reasons, I still use a `thread_rng()` call in `main.rs` instead of multiple dangerous unsafe function calls. So [`rand`](https://crates.io/crates/rand) dependency is needed. Besides, this library is intended for single thread context due to the unprotected rng in `lib.rs`: 
```rust
static mut RAND: Option<StdRng> = None;
```
I've tried to use Mutex and Arc to protect it, but the performance lost is unacceptable (about 90%, not joke). I'll explore some new ways to make this crate parallelism.

### Cover
![ray-tracing](image.png)