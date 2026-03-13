use saphyr_parser_bw::{Event, Parser};

#[test]
fn test_valid_surrogate_pair() {
    let mut parser = Parser::new_from_str(r#""\uD834\uDD1E""#);
    let mut events = vec![];
    while let Some(Ok(event)) = parser.next() {
        if event.0 == Event::StreamEnd {
            break;
        }
        events.push(event.0);
    }
    
    // Check that we got a single Scalar event with the correct character
    let mut found = false;
    for ev in events {
        if let Event::Scalar(val, _style, _, _) = ev {
            assert_eq!(val, "\u{1D11E}");
            found = true;
        }
    }
    assert!(found, "Did not find expected scalar event");
}

#[test]
fn test_unpaired_high_surrogate() {
    let mut parser = Parser::new_from_str(r#""\uD834""#);
    let mut err = None;
    while let Some(event) = parser.next() {
        match event {
            Err(e) => {
                err = Some(e);
                break;
            }
            Ok((Event::StreamEnd, _)) => break,
            _ => {}
        }
    }
    assert!(err.is_some(), "Expected error for unpaired high surrogate");
}

#[test]
fn test_unpaired_low_surrogate() {
    let mut parser = Parser::new_from_str(r#""\uDD1E""#);
    let mut err = None;
    while let Some(event) = parser.next() {
        match event {
            Err(e) => {
                err = Some(e);
                break;
            }
            Ok((Event::StreamEnd, _)) => break,
            _ => {}
        }
    }
    assert!(err.is_some(), "Expected error for unpaired low surrogate");
}

#[test]
fn test_reversed_surrogate_pair() {
    let mut parser = Parser::new_from_str(r#""\uDD1E\uD834""#);
    let mut err = None;
    while let Some(event) = parser.next() {
        match event {
            Err(e) => {
                err = Some(e);
                break;
            }
            Ok((Event::StreamEnd, _)) => break,
            _ => {}
        }
    }
    assert!(err.is_some(), "Expected error for reversed surrogate pair");
}
