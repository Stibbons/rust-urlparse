
pub struct tuple_struct(~str, ~str, ~str);

pub fn url_parse(url: &str) -> ~tuple_struct
{
    let mut out = ~"";
    out.push_str(url.slice(0, url.len()));
    let s = tuple_struct(out, ~"", ~"");
    return ~s;
}

mod test_url_parse;
