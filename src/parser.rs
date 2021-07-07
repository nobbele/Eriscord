use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Command<'a> {
    pub action: &'a str,
    pub unparsed_args: UnparsedArgs<'a>,
}

#[derive(Debug)]
pub struct UnparsedArgs<'a> {
    rest: &'a str,
}

#[derive(Debug)]
pub struct Args<'a> {
    pub args: HashSet<&'a str>, // Not sure what collection to use here
    pub set_args: HashMap<&'a str, &'a str>,
    pub words: Vec<&'a str>,
    pub text: Option<String>,
}

impl<'a> UnparsedArgs<'a> {
    pub fn parse(&self, ends_in_text: bool) -> Args<'a> {
        if self.rest.len() == 0 {
            return Args {
                args: HashSet::new(),
                set_args: HashMap::new(),
                words: Vec::new(),
                text: None,
            };
        }
        let components: Vec<&str> = self.rest.split(' ').collect();
        let components_iter = components.iter();

        let mut args: HashSet<&str> = HashSet::new();
        let mut set_args: HashMap<&str, &str> = HashMap::new();
        let mut words: Vec<&str> = Vec::new();

        let (text, long_word_count) = if ends_in_text {
            let mut text_words = components_iter
                .clone()
                .copied()
                .rev()
                .take_while(|&component| !component.starts_with('-') && !component.contains('='))
                .collect::<Vec<&str>>();
            text_words.reverse();
            let count = text_words.len();
            let text = text_words.join(" ");
            (Some(text), count)
        } else {
            (None, 0)
        };

        for &component in components_iter.take(components.len() - long_word_count) {
            if let Some(arg) = component.strip_prefix('-') {
                if let Some((k, v)) = arg.split_once('=') {
                    set_args.insert(k, v);
                } else {
                    args.insert(arg);
                }
            } else {
                words.push(component);
            }
        }

        Args {
            args,
            set_args,
            words,
            text,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandParser {
    prefix: String,
}

impl CommandParser {
    pub fn new(prefix: String) -> Self {
        CommandParser { prefix }
    }

    pub fn parse_command<'a>(&self, message: &'a str) -> Option<Command<'a>> {
        let message = message.strip_prefix(&self.prefix)?;

        let (action, rest) = if let Some((idx, _)) = message.char_indices().find(|&(_, c)| c == ' ')
        {
            (&message[..idx], &message[(idx + 1)..])
        } else {
            (&message[..], &message[0..0])
        };

        Some(Command {
            action,
            unparsed_args: UnparsedArgs { rest },
        })
    }
}
