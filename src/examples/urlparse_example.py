import urlparse

url = 'http://www.gurlge.com:80/path/file.html;params?a=1#fragment'
o = urlparse.urlparse(url)
print o.scheme
print o.netloc
print o.hostname
print o.port
print o.path
print o.params
print o.query
print o.fragment
print o.username
print o.password
