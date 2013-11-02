
use extra::url;

impl Url {
    fn get() {
        return ~"";
    }
}

pub fn urlparse(url_str: &str) -> ~url::Url
{
    return ~url::from_str(url_str).unwrap();
}

pub fn urlunparse(parsed_url: &url::Url) -> ~str
{
    return ~"";
}

pub fn urljoin(base: &str, url: &str)-> ~str
{
    return ~"";
}
