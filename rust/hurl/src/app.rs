use log::{debug, trace};
use std::convert::TryFrom;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::config;
use crate::errors::{Error, HurlResult};
use crate::session::make_safe_pathname;

/// A command line HTTP client
#[derive(StructOpt, Debug)]
#[structopt(name = "hurl")]
pub struct App {
    /// Activate quite mode.
    ///
    /// This overrides any verbose settings.
    #[structopt(short, long)]
    pub quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Form mode.
    #[structopt(short, long)]
    pub form: bool,

    /// Basic authentication.
    ///
    /// A string of the form `username:password`. If only
    /// `username` is given then you will be prompted
    /// for a password. If you wish to use no password
    /// then use the form `username:`.
    #[structopt(short, long)]
    pub auth: Option<String>,

    /// Bearer token authentication.
    ///
    /// A token which will be sent as "Bearer <token>" in
    /// the authorization header.
    #[structopt(short, long)]
    pub token: Option<String>,

    /// Session name.
    #[structopt(long)]
    pub session: Option<String>,

    /// Session storage location.
    pub session_dir: Option<PathBuf>,

    /// If true then use the stored session to augment the request,
    /// but do not modify what is stored.
    #[structopt(long)]
    pub read_only: bool,

    /// Default transport.
    ///
    /// If a URL is given without a transport, i.e. example.com/foo
    /// http will be used as the transport by default. If this flag
    /// is set then https will be used instead.
    #[structopt(short, long)]
    pub secure: bool,

    /// The HTTP Method to use, one of: HEAD, GET, POST, PUT, PATCH, DELETE
    #[structopt(subcommand)]
    pub cmd: Option<Method>,

    /// The URL to issue a request to if a method subcommand is not specified.
    pub url: Option<String>,

    /// Configuration file.
    ///
    /// A TOML file which is stored by default at HOME/.config/hurl/config
    /// where HOME is platform dependent.
    ///
    /// The file supports the following optional keys wth the given types:
    /// verbose: u8
    /// form: bool
    /// auth: string
    /// token: string
    /// secure: bool
    ///
    /// Each option has the same meaning as the corresponding configuration
    /// option with the sae name. The verbose setting is a number from 0
    /// meaning no logging to 5 meaning maximal log output.
    #[structopt(short, long, env = "HURL_CONFIG", parse(from_os_str))]
    pub config: Option<PathBuf>,

    /// The parameters for the request if a method subcommand is not specified.
    ///
    /// There are seven types of parameters that can be added to a command-line.
    /// Each type of parameter is distinguished by the unique separator between
    /// the key and value.
    ///
    /// Header -- key:value
    ///
    ///   e.g. XAPI-TOKEN:abc123
    ///
    /// File upload -- key@filename
    ///
    ///   this simulates a file upload via multipart/form-data nd requires --form
    ///
    /// Query parameter -- key==value
    ///
    ///   e.g. foo==bar becomes example.com?foo=bar
    ///
    /// Data field -- key=value
    ///
    ///   e.g. foo=bar becomes {"foo":"bar"} for JSON or form encoded
    ///
    /// Data field from file -- key=@filename
    ///
    ///   e.g. foo=@bar.txt becomes {"foo":"the contexts of bar.txt"} or form encoded
    ///
    /// Raw JSON data where the value should be parsed to JSON first --key:=value
    ///
    ///   e.g. foo:=[1,2,3] becomes {"foo":[1,2,3]}
    ///
    /// Raw JSON data from file -- key:=@filename
    ///
    ///   e.g. foo:=@bar.json becomes {"foo":{"bar":"this is from bar.json"}}
    #[structopt(parse(try_from_str = parse_param))]
    pub parameters: Vec<Parameter>,
}

impl App {
    pub fn validate(&mut self) -> HurlResult<()> {
        if self.cmd.is_none() && self.url.is_none() {
            return Err(Error::MissingUrlAndCommand);
        }
        Ok(())
    }

    pub fn process_config_file(&mut self) {
        let config_path = config::config_file(self);
        let config_opt = config::read_config_file(config_path);
        if let Some(mut config) = config_opt {
            if self.verbose == 0 {
                if let Some(v) = config.verbose {
                    self.verbose = v;
                }
            }
            if !self.form {
                if let Some(f) = config.form {
                    self.form = f;
                }
            }
            if !self.secure {
                if let Some(s) = config.secure {
                    self.secure = s;
                }
            }
            if self.auth.is_none() {
                self.auth = config.auth.take();
            }
            if self.token.is_none() {
                self.token = config.token.take();
            }
        }
    }

    pub fn log_level(&self) -> Option<&'static str> {
        if self.quiet || self.verbose <= 0 {
            return None;
        }

        match self.verbose {
            1 => Some("error"),
            2 => Some("warn"),
            3 => Some("info"),
            4 => Some("debug"),
            _ => Some("trace"),
        }
    }

    pub fn host(&self) -> String {
        if let Some(url) = &self.url {
            make_safe_pathname(url)
        } else if let Some(cmd) = &self.cmd {
            make_safe_pathname(&cmd.data().url)
        } else {
            unreachable!()
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "screaming_snake_case")]
pub enum Method {
    HEAD(MethodData),
    GET(MethodData),
    PUT(MethodData),
    POST(MethodData),
    PATCH(MethodData),
    DELETE(MethodData),
}

impl Method {
    pub fn data(&self) -> &MethodData {
        use Method::*;
        match self {
            HEAD(x) => x,
            GET(x) => x,
            PUT(x) => x,
            POST(x) => x,
            PATCH(x) => x,
            DELETE(x) => x,
        }
    }
}

impl From<&Method> for reqwest::Method {
    fn from(m: &Method) -> reqwest::Method {
        match m {
            Method::HEAD(_) => reqwest::Method::HEAD,
            Method::GET(_) => reqwest::Method::GET,
            Method::PUT(_) => reqwest::Method::PUT,
            Method::POST(_) => reqwest::Method::POST,
            Method::PATCH(_) => reqwest::Method::PATCH,
            Method::DELETE(_) => reqwest::Method::DELETE,
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct MethodData {
    /// The URL to request.
    pub url: String,

    /// The headers, data, and query parameters to add to the request.
    ///
    /// There are seven types of parameters that can be added to a command-line.
    /// Each type of parameter is distinguished by the unique separator between
    /// the key and value.
    ///
    /// Header -- key:value
    ///
    ///   e.g. XAPI-TOKEN:abc123
    ///
    /// File upload -- key@filename
    ///
    ///   this simulates a file upload via multipart/form-data nd requires --form
    ///
    /// Query parameter -- key==value
    ///
    ///   e.g. foo==bar becomes example.com?foo=bar
    ///
    /// Data field -- key=value
    ///
    ///   e.g. foo=bar becomes {"foo":"bar"} for JSON or form encoded
    ///
    /// Data field from file -- key=@filename
    ///
    ///   e.g. foo=@bar.txt becomes {"foo":"the contexts of bar.txt"} or form encoded
    ///
    /// Raw JSON data where the value should be parsed to JSON first --key:=value
    ///
    ///   e.g. foo:=[1,2,3] becomes {"foo":[1,2,3]}
    ///
    /// Raw JSON data from file -- key:=@filename
    ///
    ///   e.g. foo:=@bar.json becomes {"foo":{"bar":"this is from bar.json"}}
    #[structopt(parse(try_from_str = parse_param))]
    pub parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub enum Parameter {
    // :
    Header { key: String, value: String },
    // =
    Data { key: String, value: String },
    // :=
    RawJsonData { key: String, value: String },
    // ==
    Query { key: String, value: String },
    // @
    FormFile { key: String, value: String },
    // =@
    DataFile { key: String, value: String },
    // :=@
    RawJsonDataFile { key: String, value: String },
}

impl Parameter {
    pub fn is_form_file(&self) -> bool {
        match *self {
            Parameter::FormFile { .. } => true,
            _ => false,
        }
    }

    pub fn is_data(&self) -> bool {
        match *self {
            Parameter::Header { .. } => false,
            Parameter::Query { .. } => false,
            _ => true,
        }
    }
}

#[derive(Debug)]
enum Token<'a> {
    Text(&'a str),
    Escape(char),
}

/// This scans the `src` argument by looking for any escaped `@`, `\`, `=`, and `:` characters
/// (which would be preceeded by a `\`) while tokenizing the remaining characters.
fn gather_escapes<'a>(src: &'a str) -> Vec<Token<'a>> {
    let mut tokens = Vec::new();
    let mut start = 0;
    let mut end = 0;
    let mut chars = src.chars();
    loop {
        let a = chars.next();
        if a.is_none() {
            if start != end {
                tokens.push(Token::Text(&src[start..end]))
            }
            return tokens;
        }
        let c = a.unwrap();
        if c != '\\' {
            end += 1;
            continue;
        }
        let b = chars.next();
        if b.is_none() {
            tokens.push(Token::Text(&src[start..end + 1]));
            return tokens;
        }
        let c = b.unwrap();
        match c {
            '\\' | '=' | '@' | ':' => {
                if start != end {
                    tokens.push(Token::Text(&src[start..end]));
                }
                tokens.push(Token::Escape(c));
                end += 2;
                start = end;
            }
            _ => end += 2,
        }
    }
}

fn parse_param(src: &str) -> HurlResult<Parameter> {
    debug!("Parsing: {}", src);
    let separators = [":=@", "=@", "==", ":=", "@", "=", ":"];
    let tokens = gather_escapes(src);

    let mut found = Vec::new();
    // Index where the separator starts.
    let mut idx = 0;
    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::Text(s) => {
                for sep in separators.iter() {
                    if let Some(n) = s.find(sep) {
                        found.push((n, sep));
                    }
                }
                if !found.is_empty() {
                    idx = i;
                    // break since we found a separator.
                    break;
                }
            }
            Token::Escape(_) => {}
        }
    }
    if found.is_empty() {
        return Err(Error::ParameterMissingSeparator(src.to_owned()));
    }
    // Sort the found separator tokens so the first element is the earliest index and longest
    // separator.
    found.sort_by(|(ai, asep), (bi, bsep)| ai.cmp(bi).then(bsep.len().cmp(&asep.len())));
    let sep = found.first().unwrap().1;
    trace!("Found separator: {}", sep);

    let mut key = String::new();
    let mut value = String::new();
    for (i, token) in tokens.iter().enumerate() {
        if i < idx {
            match token {
                Token::Text(s) => key.push_str(&s),
                Token::Escape(c) => {
                    key.push('\\');
                    key.push(*c);
                }
            }
        } else if i > idx {
            match token {
                Token::Text(s) => value.push_str(&s),
                Token::Escape(c) => {
                    value.push('\\');
                    value.push(*c);
                }
            }
        } else {
            if let Token::Text(s) = token {
                let parts: Vec<&str> = s.splitn(2, sep).collect();
                let k = parts.first().unwrap();
                let v = parts.last().unwrap();
                key.push_str(k);
                value.push_str(v);
            } else {
                unreachable!();
            }
        }
    }

    if let Ok(separator) = Separator::try_from(*sep) {
        match separator {
            Separator::At => Ok(Parameter::FormFile { key, value }),
            Separator::Equal => Ok(Parameter::Data { key, value }),
            Separator::Colon => Ok(Parameter::Header { key, value }),
            Separator::ColonEqual => Ok(Parameter::RawJsonData { key, value }),
            Separator::EqualEqual => Ok(Parameter::Query { key, value }),
            Separator::EqualAt => Ok(Parameter::DataFile { key, value }),
            Separator::Snail => Ok(Parameter::RawJsonDataFile { key, value }),
        }
    } else {
        unreachable!();
    }
}

#[derive(Debug)]
enum Separator {
    Colon,
    Equal,
    At,
    ColonEqual,
    EqualEqual,
    EqualAt,
    Snail,
}

impl TryFrom<&str> for Separator {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ":" => Ok(Separator::Colon),
            "=" => Ok(Separator::Equal),
            "@" => Ok(Separator::At),
            ":=" => Ok(Separator::ColonEqual),
            "==" => Ok(Separator::EqualEqual),
            "=@" => Ok(Separator::EqualAt),
            ":=@" => Ok(Separator::Snail),
            _ => Err(()),
        }
    }
}
