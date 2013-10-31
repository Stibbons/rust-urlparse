use super::url_parse;
use super::tuple_struct;

#[test]
fn test_url_parse()
{
    let url = "http://www.gurlge.com:80/path/file.html;params?a=1#fragment";

    let ~tuple_struct(x, _, _) = url_parse(url);

    println!("Ma variable statique: {}", x);
}
