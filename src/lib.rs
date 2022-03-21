use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub mod tests;

pub struct Await<F> {
    recv: mpsc::Receiver<F>,
    done: bool,
    result: Option<F>
}

impl<F: Send + 'static> Await<F> {
    /// Create async task
    /// 
    /// ## Usage
    /// 
    /// ```
    /// use std::time::Duration;
    /// use wait_not_await::Await;
    /// 
    /// let mut awaiter = Await::new(move || {
    ///     std::thread::sleep(Duration::from_secs(3));
    /// 
    ///     "Hello, Wolrd!".to_string()
    /// });
    /// 
    /// if let Some(result) = awaiter.wait(None) {
    ///     println!("Result: {}", result);
    /// }
    /// ```
    pub fn new<T>(task: T) -> Await<F> where T: Fn() -> F + Send + 'static {
        let (sender, receiver) = mpsc::channel() as (mpsc::Sender<F>, mpsc::Receiver<F>);

        let awaiter = Await {
            recv: receiver,
            done: false,
            result: None
        };

        thread::spawn(move || {
            sender.send(task());
        });

        awaiter
    }

    /// Awaiter result
    pub fn result(&mut self) -> &Option<F> {
        if !self.done {
            if let Ok(result) = self.recv.try_recv() {
                self.done = true;
                self.result = Some(result);
            }
        }
        
        &self.result
    }

    /// Wait for execution result
    /// 
    /// If `timeout = None`, then this function will wait
    /// until the inner function will not return its result
    pub fn wait(&mut self, timeout: Option<Duration>) -> &Option<F> {
        if !self.done {
            match timeout {
                Some(timeout) => {
                    if let Ok(result) = self.recv.recv_timeout(timeout) {
                        self.done = true;
                        self.result = Some(result);
                    }
                },
                None => {
                    if let Ok(result) = self.recv.recv() {
                        self.done = true;
                        self.result = Some(result);
                    }
                }
            }
        }

        &self.result
    }
}
