use crate::Unit;
use std::error;
use std::fmt;
use std::num::ParseFloatError;

pub enum Token {
    Unit(Unit),
    Number,
}

#[derive(Debug)]
pub enum ScanError {
    TokenError(Error),
    ParseError(ParseFloatError),
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScanError::TokenError(err) => err.fmt(f),
            ScanError::ParseError(err) => err.fmt(f),
        }
    }
}

impl error::Error for ScanError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ScanError::TokenError(err) => Some(err),
            ScanError::ParseError(err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {}

%%
%class Lexer
%result_type Token
([bB]ytes?|b|B)           return Ok(Token::Unit(Unit::Byte));
([kK]ilobytes?|kb?|KB?)   return Ok(Token::Unit(Unit::Kilobyte));
([mM]egabytes?|mb?|MB?)   return Ok(Token::Unit(Unit::Megabyte));
([gG]igabytes?|gb?|GB?)   return Ok(Token::Unit(Unit::Gigabyte));
([tT]erabytes?|tb?|TB?)   return Ok(Token::Unit(Unit::Terabyte));
([pP]etabytes?|pb?|PB?)   return Ok(Token::Unit(Unit::Petabyte));
([eE]xabytes?|eb?|EB?)    return Ok(Token::Unit(Unit::Exabyte));
([kK]ibibytes?|kib?|KiB?) return Ok(Token::Unit(Unit::Kibibyte));
([mM]ebibytes?|mib?|MiB?) return Ok(Token::Unit(Unit::Mebibyte));
([gG]ibibytes?|gib?|GiB?) return Ok(Token::Unit(Unit::Gibibyte));
([pP]ebibytes?|pib?|PiB?) return Ok(Token::Unit(Unit::Pebibyte));
([tT]ebibytes?|tib?|TiB?) return Ok(Token::Unit(Unit::Tebibyte));
([eE]xbibytes?|eib?|EiB?) return Ok(Token::Unit(Unit::Exbibyte));
" "                       /* Skip whitespace */
[0-9.]+([eE][-+]?[0-9]+)? return Ok(Token::Number);
%%

    pub fn read_number(&mut self) -> Result<f64, ScanError> {
        let token = self.yylex().map_err(|err| ScanError::TokenError(err))?;
        match token {
            Token::Unit(_) => Err(ScanError::TokenError(Error::Unmatch)),
            Token::Number => {
                let text = self.yytext();
                let num = text.parse::<f64>();
                num.map_err(|err| ScanError::ParseError(err))
            }
        }
    }

    pub fn read_unit(&mut self) -> Result<Unit, ScanError> {
        let token = self.yylex().map_err(|err| ScanError::TokenError(err))?;
        match token {
            Token::Unit(u) => Ok(u),
            Token::Number => Err(ScanError::TokenError(Error::Unmatch))
        }
    }

    pub fn read_eof(&mut self) -> Result<(), ScanError> {
        let token = self.yylex().err();
        match token {
            Some(Error::EOF) => Ok(()),
            _ => Err(ScanError::TokenError(Error::Unmatch)),
        }
    }
