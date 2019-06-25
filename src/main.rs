// std
use std::borrow::Cow::{ self, Borrowed, Owned };

// extern
use rustyline::error::ReadlineError;
use rustyline::{ Editor, Context, Helper };
use rustyline::config::{ Builder, ColorMode, EditMode, CompletionType };
use rustyline::hint::{ HistoryHinter, Hinter };
use rustyline::completion::{ FilenameCompleter, Completer, Pair };
use rustyline::highlight::{ Highlighter, MatchingBracketHighlighter };

struct RLHelper {
    completer: FilenameCompleter,
    highlighter: SynHighlighter,
    hinter: HistoryHinter,
}

struct SynHighlighter { }
impl Highlighter for SynHighlighter {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("\x1b[90m{}\x1b[0m", hint))
    }
    // fn highlight<'l>(&self, line: &'l str, _: usize) -> Cow<'l, str> {

    // }
}

impl Highlighter for RLHelper { 
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        self.highlighter.highlight_hint(hint)
    }
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }
}

impl Completer for RLHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
        ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for RLHelper {
    fn hint(&self, line: &str, a: usize, b: &Context) -> Option<String> {
        self.hinter.hint(line, a, b)
    }
}

impl Helper for RLHelper {}
fn main() {
    let rl_config_builder = Builder::new();
    let rl_config = rl_config_builder.color_mode(ColorMode::Enabled)
        .edit_mode(EditMode::Emacs)
        .history_ignore_space(true)
        .completion_type(CompletionType::Circular)
        .max_history_size(1000)
        .build();
    let mut ale = Editor::<()>::with_config(rl_config);

    if ale.load_history("./history.txt").is_err() {
        println!("No previous history!")
    };

    loop {
        let ale_input = ale.readline("$ ");
        match ale_input {
            Ok(line) => {
                ale.add_history_entry(line.as_ref());
                let op = cicada::run(line.as_ref());
                println!("{}", op.status);
                println!("{}", op.stdout);
                println!("\x1b[31m{}\x1b[0m", op.stderr);
            },
            Err(ReadlineError::Interrupted) =>{
                println!("ctrl-c");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("ctrl-d");
                break
            }
            Err(_) => {
                println!("encountered err");
                break
            }
        }
    }
}
