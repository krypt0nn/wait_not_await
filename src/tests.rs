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

    assert_eq!(&Some("Hello, World!".to_string()), awaiter.wait(None));
    assert_eq!(&Some("Hello, World!".to_string()), awaiter.result());
}

#[test]
fn test_wait_not_enough() {
    let mut awaiter = await_hello_world();

    assert_eq!(&None, awaiter.wait(Some(Duration::from_millis(500))));
    assert_eq!(&None, awaiter.result());
}

#[test]
fn test_wait_more() {
    let mut awaiter = await_hello_world();

    assert_eq!(&Some("Hello, World!".to_string()), awaiter.wait(Some(Duration::from_millis(1500))));
    assert_eq!(&Some("Hello, World!".to_string()), awaiter.result());
}

#[test]
fn test_result_loop() {
    let mut awaiter = await_hello_world();

    while let None = awaiter.result() {}

    assert_eq!(&Some("Hello, World!".to_string()), awaiter.result());
    assert_eq!(&Some("Hello, World!".to_string()), awaiter.wait(None));
}
