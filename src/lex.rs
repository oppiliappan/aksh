pub enum IoModifier {
    Less,
    Great,
    GreatGreat,
    GreatAmp,
    GreatGreatAmp,
}

pub enum Token {
    Command(String),
    Arg(String),
    Pipe,
    IoMod(IoModifier),
    Amp,
    File(String),
    LiteralWord(String)
}

pub fn tokenize(cicada_tokens: Vec<(String, String)>) -> Vec<Token> {
    let mut is_prev_token_pipe: bool    = false;
    let mut is_prev_token_io_mod: bool = false;
    let mut tokens = vec![];

    for item in cicada_tokens {
        match item {
            (_, ">".into()) => {
                tokens.push(Token::IoMod::Great);
            },

            (_, ">>".into()) => {
                tokens.push(Token::IoMod::GreatGreat);
            },

            (_, "&>".into()) | (_, ">&".into()) => {
                tokens.push(Token::IoMod::GreatAmp);
            },

            (_, "&&>".into()) | (_, ">&&".into()) => {
                tokens.push(Token::IoMod::GreatGreatAmp);
            },

            (_, "|".into()) => {
                tokens.push(Token::Pipe);
            },

            ("\"".into(), x) => {
                tokens.push(Token::LiteralWord(x));
            },

            // subshells can go here eg.: `which ls`

            (_, x) => {
                if is_prev_token_pipe == true {
                    tokens.push(Token::Command(x));
                } else if is_prev_token_io_mod == true {
                    tokens.push(Token::File(x));
                } else {
                    tokens.push(Token::Arg(x));
                }
                is_prev_token_pipe   = false;
                is_prev_token_io_mod = false;
            }

        }
    }
}
