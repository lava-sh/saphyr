use saphyr_parser_bw::{Parser, Event, EventReceiver};

struct Collector(Vec<Event<'static>>);
impl EventReceiver<'static> for Collector {
    fn on_event(&mut self, ev: Event<'static>) {
        self.0.push(ev);
    }
}

#[test]
fn test_unclosed_flow_sequence_at_eof() {
    let input = "[";
    let mut parser = Parser::new_from_str(input);
    let mut collector = Collector(Vec::new());
    let res = parser.load(&mut collector, false);
    
    println!("Events: {:?}", collector.0);
    println!("Result: {:?}", res);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().info(), "unexpected EOF while parsing a flow sequence");
}

#[test]
fn test_unclosed_flow_mapping_at_eof() {
    let input = "{";
    let mut parser = Parser::new_from_str(input);
    let mut collector = Collector(Vec::new());
    let res = parser.load(&mut collector, false);
    
    println!("Events: {:?}", collector.0);
    println!("Result: {:?}", res);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().info(), "unexpected EOF while parsing a flow mapping");
}

#[test]
fn test_unclosed_implicit_flow_mapping_at_eof() {
    let input = "[ a:";
    let mut parser = Parser::new_from_str(input);
    let mut collector = Collector(Vec::new());
    let res = parser.load(&mut collector, false);
    
    println!("Events: {:?}", collector.0);
    println!("Result: {:?}", res);
    assert!(res.is_err());
    let info = res.unwrap_err().info().to_string();
    assert_eq!(info, "unexpected EOF while parsing a flow mapping");
}

#[test]
fn test_unclosed_quoted_scalar_at_eof() {
    let input = "\"abc";
    let mut parser = Parser::new_from_str(input);
    let mut collector = Collector(Vec::new());
    let res = parser.load(&mut collector, false);
    
    println!("Events: {:?}", collector.0);
    println!("Result: {:?}", res);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().info(), "while scanning a quoted scalar, found unexpected end of stream");
}
