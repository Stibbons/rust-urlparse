
#[test]
use super::super::urlparse::urlparse;

#[test]
fn test_urlparse()
{
    let url = "http://www.gurlge.com:80/path/file.html;params?a=1#fragment";

    let parsed_url = urlparse(url);

    println!("..url={}", parsed_url.scheme);
}
