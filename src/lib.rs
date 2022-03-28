use std::thread;
use std::sync::mpsc;
use std::time::Duration;

mod tests;

pub struct Await<F> {
    done: bool,
    result: Option<F>,
    then_sender: mpsc::Sender<Box<dyn FnOnce(&F) + Send>>,
    result_receiver: mpsc::Receiver<F>
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
    pub fn new<T: FnOnce() -> F + Send + 'static>(task: T) -> Await<F> {
        let (result_sender, result_receiver) = mpsc::channel::<F>();
        let (then_sender, then_receiver) = mpsc::channel::<Box<dyn FnOnce(&F) + Send>>();

        let awaiter = Await {
            done: false,
            result: None,
            then_sender,
            result_receiver
        };

        thread::spawn(move || {
            let result = task();

            while let Ok(callable) = then_receiver.try_recv() {
                callable(&result);
            }

            result_sender.send(result);
        });

        awaiter
    }

    /// Awaiter result
    pub fn result(&mut self) -> Option<&F> {
        if !self.done {
            if let Ok(result) = self.result_receiver.try_recv() {
                self.done = true;
                self.result = Some(result);
            }
        }

        self.result.as_ref()
    }

    /// Wait for execution result
    /// 
    /// If `timeout = None`, then this function will wait
    /// until the inner function will not return its result
    pub fn wait(&mut self, timeout: Option<Duration>) -> Option<&F> {
        if !self.done {
            match timeout {
                Some(timeout) => {
                    if let Ok(result) = self.result_receiver.recv_timeout(timeout) {
                        self.done = true;
                        self.result = Some(result);
                    }
                },
                None => {
                    if let Ok(result) = self.result_receiver.recv() {
                        self.done = true;
                        self.result = Some(result);
                    }
                }
            }
        }

        self.result.as_ref()
    }

    /// Specify callback to be executed when the task will be completed
    /// 
    /// ## Example:
    /// 
    /// ```
    /// use wait_not_await::Await;
    /// use std::time::Duration;
    /// 
    /// let task = Await::new(move || {
    ///     std::thread::sleep(Duration::from_secs(3));
    /// 
    ///     "Hello, Wolrd!".to_string()
    /// });
    /// 
    /// task.then(move |result| {
    ///     println!("Task result: {}", result);
    /// });
    /// ```
    pub fn then<C: FnOnce(&F) + Send + 'static>(&self, callable: C) -> bool {
        self.then_sender.send(Box::new(callable)).is_ok()
    }
}
