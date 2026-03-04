use crate::{
    input::{str::StrInput, BorrowedInput, BufferedInput},
    parser::{Event, ParseResult, Parser, ParserTrait, SpannedEventReceiver},
    scanner::{ScanError, Span},
};
use alloc::{string::String, vec::Vec};

enum AnyParser<'input, I, T>
where
    I: Iterator<Item = char>,
    T: BorrowedInput<'input>,
{
    String {
        parser: Parser<'input, StrInput<'input>>,
        name: String,
    },
    Iter {
        parser: Parser<'static, BufferedInput<I>>,
        name: String,
    },
    Custom {
        parser: Parser<'input, T>,
        name: String,
    },
}

impl<'input, I, T> AnyParser<'input, I, T>
where
    I: Iterator<Item = char>,
    T: BorrowedInput<'input>,
{
    fn get_anchor_offset(&self) -> usize {
        match self {
            AnyParser::String { parser, .. } => parser.get_anchor_offset(),
            AnyParser::Iter { parser, .. } => parser.get_anchor_offset(),
            AnyParser::Custom { parser, .. } => parser.get_anchor_offset(),
        }
    }

    fn set_anchor_offset(&mut self, offset: usize) {
        match self {
            AnyParser::String { parser, .. } => parser.set_anchor_offset(offset),
            AnyParser::Iter { parser, .. } => parser.set_anchor_offset(offset),
            AnyParser::Custom { parser, .. } => parser.set_anchor_offset(offset),
        }
    }
}

/// A parser implementation that utilizes a stack for parsing.
pub struct ParserStack<'input, I = core::iter::Empty<char>, T = StrInput<'input>>
where
    I: Iterator<Item = char>,
    T: BorrowedInput<'input>,
{
    parsers: Vec<AnyParser<'input, I, T>>,
    current: Option<(Event<'input>, Span)>,
    stream_end_emitted: bool,
}

impl<'input, I, T> ParserStack<'input, I, T>
where
    I: Iterator<Item = char>,
    T: BorrowedInput<'input>,
{
    /// Creates a new, empty parser stack.
    #[must_use]
    pub fn new() -> Self {
        Self {
            parsers: Vec::new(),
            current: None,
            stream_end_emitted: false,
        }
    }

    /// Pushes a string parser onto the stack.
    pub fn push_str_parser(&mut self, mut parser: Parser<'input, StrInput<'input>>, name: String) {
        if let Some(parent) = self.parsers.last() {
            parser.set_anchor_offset(parent.get_anchor_offset());
        }
        self.parsers.push(AnyParser::String { parser, name });
    }

    /// Pushes an iterator parser onto the stack.
    pub fn push_iter_parser(&mut self, mut parser: Parser<'static, BufferedInput<I>>, name: String) {
        if let Some(parent) = self.parsers.last() {
            parser.set_anchor_offset(parent.get_anchor_offset());
        }
        self.parsers.push(AnyParser::Iter { parser, name });
    }

    /// Pushes a custom parser onto the stack.
    pub fn push_custom_parser(&mut self, mut parser: Parser<'input, T>, name: String) {
        if let Some(parent) = self.parsers.last() {
            parser.set_anchor_offset(parent.get_anchor_offset());
        }
        self.parsers.push(AnyParser::Custom { parser, name });
    }

    /// Returns the names of the parsers currently in the stack.
    #[must_use]
    pub fn stack(&self) -> Vec<String> {
        self.parsers
            .iter()
            .map(|p| match p {
                AnyParser::String { name, .. }
                | AnyParser::Iter { name, .. }
                | AnyParser::Custom { name, .. } => name.clone(),
            })
            .collect()
    }

    fn next_event_impl(&mut self) -> Result<(Event<'input>, Span), ScanError> {
        loop {
            let Some(any_parser) = self.parsers.last_mut() else {
                return Ok((
                    Event::StreamEnd,
                    Span::empty(crate::scanner::Marker::new(0, 1, 0)),
                ));
            };

            let res = match any_parser {
                AnyParser::String { parser, .. } => parser.next_event(),
                AnyParser::Iter { parser, .. } => parser.next_event(),
                AnyParser::Custom { parser, .. } => parser.next_event(),
            };

            match res {
                Some(Ok((Event::StreamEnd, _))) | None => {
                    let popped = self.parsers.pop().unwrap();
                    if let Some(parent) = self.parsers.last_mut() {
                        parent.set_anchor_offset(popped.get_anchor_offset());
                    }
                }
                Some(Err(e)) => {
                    let popped = self.parsers.pop().unwrap();
                    if let Some(parent) = self.parsers.last_mut() {
                        parent.set_anchor_offset(popped.get_anchor_offset());
                    }
                    if e.info().contains("EOF") {
                        continue;
                    }
                    return Err(e);
                }
                Some(Ok((Event::DocumentEnd, span))) => {
                    if self.parsers.len() == 1 {
                        return Ok((Event::DocumentEnd, span));
                    }

                    // Check if it has more documents
                    let peek_res = match self.parsers.last_mut().unwrap() {
                        AnyParser::String { parser, .. } => parser.peek(),
                        AnyParser::Iter { parser, .. } => parser.peek(),
                        AnyParser::Custom { parser, .. } => parser.peek(),
                    };

                    match peek_res {
                        Some(Ok((Event::StreamEnd, _))) | None => {
                            let popped = self.parsers.pop().unwrap();
                            if let Some(parent) = self.parsers.last_mut() {
                                parent.set_anchor_offset(popped.get_anchor_offset());
                            }
                        }
                        _ => {
                            return Err(ScanError::new_str(span.start, "multiple documents not supported here"));
                        }
                    }
                }
                Some(Ok(event)) => {
                    return Ok(event);
                }
            }
        }
    }
}

impl<'input, I, T> Default for ParserStack<'input, I, T>
where
    I: Iterator<Item = char>,
    T: BorrowedInput<'input>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'input, I, T> ParserTrait<'input> for ParserStack<'input, I, T>
where
    I: Iterator<Item = char>,
    T: BorrowedInput<'input>,
{
    fn peek(&mut self) -> Option<Result<&(Event<'input>, Span), ScanError>> {
        if let Some(ref x) = self.current {
            Some(Ok(x))
        } else {
            if self.stream_end_emitted {
                return None;
            }
            match self.next_event_impl() {
                Ok(token) => {
                    self.current = Some(token);
                    Some(Ok(self.current.as_ref().unwrap()))
                }
                Err(e) => Some(Err(e)),
            }
        }
    }

    fn next_event(&mut self) -> Option<ParseResult<'input>> {
        if self.current.is_some() {
            return self.current.take().map(Ok);
        }
        if self.stream_end_emitted {
            return None;
        }
        match self.next_event_impl() {
            Ok(token) => {
                if let Event::StreamEnd = token.0 {
                    self.stream_end_emitted = true;
                }
                Some(Ok(token))
            }
            Err(e) => Some(Err(e)),
        }
    }

    fn load<R: SpannedEventReceiver<'input>>(
        &mut self,
        _recv: &mut R,
        _multi: bool,
    ) -> Result<(), ScanError> {
        todo!()
    }
}
