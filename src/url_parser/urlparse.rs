
pub struct ParsedUrl
{
    scheme: ~str,
    netloc: ~str,
    path: ~str,
    params: ~str,
    query: ~str,
    fragment: ~str
}

/*
 * Parse a URL into 6 components
 *
 * <scheme>://<netloc>/<path>;<params>?<query>#<fragment>
 * Return a 6-tuple: (scheme, netloc, path, params, query, fragment).
 * Note that we don't break the components up in smaller bits
 * (e.g. netloc is a single string) and we don't expand % escapes.
 */
pub fn urlparse(url: &str) -> ~ParsedUrl
{
    let mut out = ~"";
    out.push_str(url.slice(0, url.len()));
    let s = ParsedUrl{scheme: out,
                      netloc: ~"",
                      path: ~"",
                      params: ~"",
                      query: ~"",
                      fragment: ~""};
    return ~s;
}
