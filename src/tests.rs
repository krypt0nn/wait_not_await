use super::*;

#[allow(dead_code)]
fn await_hello_world() -> Await<String> {
    Await::new(move || {
        std::thread::sleep(Duration::from_secs(1));

        "Hello, World!".to_string()
    })
}

#[test]
fn test_wait_none() {
    let mut awaiter = await_hello_world();

    assert_eq!(Some(&"Hello, World!".to_string()), awaiter.wait(None));
    assert_eq!(Some(&"Hello, World!".to_string()), awaiter.result());
}

#[test]
fn test_wait_not_enough() {
    let mut awaiter = await_hello_world();

    assert_eq!(None, awaiter.wait(Some(Duration::from_millis(500))));
    assert_eq!(None, awaiter.result());
}

#[test]
fn test_wait_more() {
    let mut awaiter = await_hello_world();

    assert_eq!(Some(&"Hello, World!".to_string()), awaiter.wait(Some(Duration::from_millis(1500))));
    assert_eq!(Some(&"Hello, World!".to_string()), awaiter.result());
}

#[test]
fn test_result_loop() {
    let mut awaiter = await_hello_world();

    while let None = awaiter.result() {}

    assert_eq!(Some(&"Hello, World!".to_string()), awaiter.result());
    assert_eq!(Some(&"Hello, World!".to_string()), awaiter.wait(None));
}

#[test]
fn test_then() {
    let greeting = await_hello_world();

    let (send, recv) = std::sync::mpsc::channel();

    greeting.then(move |result| {
        send.send(result.clone()).unwrap();
    });

    assert_eq!("Hello, World!".to_string(), recv.recv_timeout(Duration::from_millis(1500)).unwrap());
}

#[test]
fn test_mut() {
    let mut result = "Is not completed yet";

    let mut awaiter = Await::new(move || {
        std::thread::sleep(Duration::from_secs(1));

        result = "Hello, World!";

        result
    });

    assert_eq!("Is not completed yet", result);
    assert_eq!(&"Hello, World!", awaiter.wait(None).unwrap());
    assert_eq!("Is not completed yet", result);
}
