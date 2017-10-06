use ::{Request, Command};

#[test]
fn test_request_get_serialization() {
    let mut request = Request::new(Command::Get);
    request.set_key(Some(b"Hello"));

    let expected: Vec<u8> = vec![
        0x80, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x48, 0x65, 0x6c, 0x6c,
        0x6f,
    ];

    let mut result: Vec<u8> = vec![];
    request.write(&mut result).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_request_delete_serialization() {
    let mut request = Request::new(Command::Delete);
    request.set_key(Some(b"Hello"));

    let expected: Vec<u8> = vec![
        0x80, 0x04, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x48, 0x65, 0x6c, 0x6c,
        0x6f,
    ];

    let mut result: Vec<u8> = vec![];
    request.write(&mut result).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_request_quit_serialization() {
    let request = Request::new(Command::Quit);

    let expected: Vec<u8> = vec![
        0x80, 0x07, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let mut result: Vec<u8> = vec![];
    request.write(&mut result).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_request_noop_serialization() {
    let request = Request::new(Command::Noop);

    let expected: Vec<u8> = vec![
        0x80, 0x0a, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let mut result: Vec<u8> = vec![];
    request.write(&mut result).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_request_version_serialization() {
    let request = Request::new(Command::Version);

    let expected: Vec<u8> = vec![
        0x80, 0x0b, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let mut result: Vec<u8> = vec![];
    request.write(&mut result).unwrap();
    assert_eq!(result, expected);
}