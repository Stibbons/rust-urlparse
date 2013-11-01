
#[test]
use super::super::urlparse::urlparse;

#[test]
fn test_urlparse()
{
    let url = "http://www.gurlge.com:80/path/file.html;params?a=1#fragment";

    let parsed_url = urlparse(url);

    println!("..url={}", parsed_url.scheme);
}


// #[test]
// fn test_parse_ipv6()
// {
//     let urls = ["http://[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]:80/index.html",
//             "http://[1080:0:0:0:8:800:200C:417A]/index.html",
//             "http://[3ffe:2a00:100:7031::1]",
//             "http://[1080::8:800:200C:417A]/foo",
//             "http://[::192.9.5.5]/ipng",
//             "http://[::FFFF:129.144.52.38]:80/index.html",
//             "http://[2010:836B:4179::836B:4179"];
// }

// #[test]
// fn test_cached_urlparse(){

// }
