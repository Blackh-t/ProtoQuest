## Comparing Thread-based vs Async-based

Web Scraping Performance with Multiprocessing, measured using `std::time::Instant`

### How to run
```bash
    cargo test -- --nocapture
```

#### Threads

- Threads spawn OS-Thread thats blocks CPU from running other task, while it wait for the response.

- Threads are expensive to create and maintain, 
due to the spawning of OS-Thread requires resources and
can overhelm the system if too many threads are created.

- The threads blokking leads to delays, as the CPU must wait for each task 
to complete instead of efficiently moving on to other work.


#### Async Tokio
- Tokio spawn async task that can be paused and resumed, that is efficient for CPU to run other task while waiting for response.

- Tokio runs in "thread pool" which allows the system to handle multiple tasks in a pool of threads.
Therefore, it used less resources, due to use of resources are reserved for a pool that can 
handle multiple tasks efficiently.

- Tokio is non-blocking, which means it can handle multiple tasks at the same time,
by waiting for the response from the server it can run other tasks,
then resume the task when the response is ready.


#### Tests results:
Align with the understanding of Tokio Async let theads run while waiting for its response,
this lead to the Tokio have to wait until the threads are done, wherefore the threads are faster.

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
