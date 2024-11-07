## Comparing Thread-based vs Async-based

Web Scraping Performance with Multiprocessing, measured using `std::time::Instant`

### How to run
```bash
    cargo test -- --nocapture
```

#### Threads

- Spawn OS-Thread thats blocks CPU from running other task, while it wait for the response.

- Expensive to create and maintain, 
due to the spawning of OS-Thread requires resources and
can overhelm the system if too many threads are created.

- Blocking leads to delays, as the CPU must wait for each task 
to complete instead of efficiently moving on to other work.


#### Async Tokio
- Spawn async task that can be paused and resumed, that is efficient for CPU to run other task while waiting for response.

- Runs in a "thread pool," which lets the system handle multiple tasks using a group of threads. This saves resources because the threads in the pool share the work and use resources more efficiently.

- Non-blocking, which means it can handle multiple tasks at the same time,
by waiting for the response from the server it can run other tasks,
then resume the task when the response is ready.


#### Tests results:
The test results align with the understanding of Tokio Async, where threads take over the CPU while waiting for a response. This approach causes delays in Tokio until the threads complete their tasks, which is why the threads appear to be faster in the test results.

```bash
running 2 tests
Thread took: 1.464676217s
test tests::test_thread_get_target ... ok
Tokio took: 1.636161456s
test tests::test_get_target ... ok
````


#### Referaces:
- https://stackoverflow.com/questions/75836002/what-is-the-benefit-of-using-tokio-instead-of-os-threads-in-rust

- https://www.zenrows.com/blog/rust-web-scraping#rust-web-scraping

- https://peq42.com/blog/parallel-and-asynchronous-programming-in-rust/
