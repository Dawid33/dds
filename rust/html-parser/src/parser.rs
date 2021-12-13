pub struct ParseState{
    script_nesting_level : u32,
    parser_pause : bool,
}

pub struct Parser {

}

impl ParseState {
    pub fn new() -> Self {
        Self {
            script_nesting_level : 0,
            parser_pause : false,
        }
    }
}