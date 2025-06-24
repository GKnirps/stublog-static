---
title: String encoding in Go
filename: golang_string_encoding
date: 2025-06-24T17:29:55+02:00
update-date:
tags: golang, unicode, encoding, utf-8
category: hoellenmaschinen
summary: Where Go makes it easy to mess up your string encoding and where Go kind of has your back.
image:
image-alt:
language: en
---

(Usually I write stuff in German, but this could be interesting to some people I know who do not speak German, so English it is.)

As frequent readers (do I have those?) know, text encoding is a small hobby / pet peeve of mine. A lot of things would be easier if we just used [UTF-8 everywhere](https://utf8everywhere.org/). But a consistent text encoding is only useful if it is used correctly and its validity is enforced. Most languages do a bad job at enforcing valid encoding. For example:

- C does not have strings, C has null-terminated `char`-arrays. Encoding optional
- C++ has strings that do not enforce encoding, newer versions have some questionable unicode strings
- Java encodes strings in UTF-16 and does not check for unpaired [surrogates](/blogposts/unicode_scalar_value)
- Javascript works similar to Java in that regard

Python and rust enforce valid unicode. Except that python allows to encode surrogate code points (but at least it does fail when it tries to encode that string as UTF-8). However, I'm not here to talk about any of these languages. I'm here to talk about [Go](https://go.dev/), since I'm probably going to use it professionally again next week.

Strings in Go are just gloriefied immutable byte arrays. They have no inherent guarantee to contain a valid string of any encoding. Since Go is such a young language, this is a shame. The worst part: Developers don't expect it. They take a modern language, read strings from external sources, modify them, write them to external sinks and are oblivious about the fact that they are just one step away from splitting a code point in the middle.

I talked about [the strengths and weaknesses of Go](/blogposts/golang_meinung) in another blogpost. Here, I'm going to ask the question: How bad is the situation? What can go wrong, where is easy to make mistakes and where is it hard to do so? Let's start with the basics.

### Thing I already knew

Source files in Go are strictly UTF-8. So in theory, string literals are also strictly UTF-8. Unless they are not because you can use escape sequences to do stuff like this:

```
var s = "foo\xffbar"
```

There, an invalid byte in the middle of the string. Go has the `unicode/utf8` package in its standard library, so we can at least check if the string is valid:

```
utf8.ValidString(s) // evaluates to false for the string above
```

You can also just create a string from a byte slice without validation. I have seen many developers doing exactly this, without even thinking about any problems:

```
c := []byte{0x66, 0x6f, 0x6f, 0xff}
s := string(c)
```

It should come to no surprise that you can just split a string in the middle of a code point:

```
s := "🐍"
s1 := s[:2] // s1 now contains an incomplete code point
```

Just as with casting a byte array to a string, this is something I have seen in the wild several times. Barely anyone thinks about it.

This situation is bad. It is far too easy to get invalidly encoded strings. And Golang is used primarily for web servers. Those get untrusted input all the time. So maybe at least the interfaces that are commonly used are protected? I have never tried this before, so let's do it!

### Unicode validation at the border of golang web services

I have written a [small test web server](https://gitlab.com/GKnirps/go_unicode_web) for a few test cases. I want to test three interfaces (or two, depending on how you count):

- URL-parameters for GET requests
- HTML form data from a POST request
- JSON data (in this case from POST request, but it does not really matter)

The first two can count more or less as one, because the interface in Golang is the same: A `http.Request`-object has a field `Form` (this field needs to be populated by a call to `ParseForm()`, but that detail does not really matter here). `Form` is basically a map from parameter name to parameter value(s). So this is what I did:

```
if err := req.ParseForm(); err != nil {
    response.WriteHeader(http.StatusBadRequest)
    fmt.Fprintln(response, "bad form data")
    return
}

query, ok := req.Form["q"]
// [I left out some validation code here]
if utf8.ValidString(query[0]) {
    log.Printf("query string '%s' is valid utf-8", query[0])
} else {
    log.Println("query string is not valid utf-8")
}
response.WriteHeader(http.StatusOK)
fmt.Fprintln(response, "looks good")
```

So basically: I try to parse the form data and return an error if any problem occurs. Then I get the string from the form map and log whether it is valid UTF-8. Then I return with a success status code. Ideally, `ParseForm()` should fail for invalid UTF-8. Let's see what happens:

```
$ curl "http://localhost:8090/form?q=foo%ff"
looks good
```

Needless to say, there is a `query string is not valid utf-8` message in the server log. And that was the most simple case. POST-body form data has basically the same result, no matter whether I use `%ff` to escape the byte or use a raw `0xff`-byte in the body.

So that's form data. What about JSON? After all, we all write bloated single-page javascript apps with a server that is basically a JSON-API. Does Go's JSON parsing do it better? As a matter of fact, it does! Assume I have this struct

```
type Input struct {
	Query string `json:"q"`
}
```

and then handle it like this:

```
body, err := io.ReadAll(req.Body)
// [omitted error handling]

var input Input
err = json.Unmarshal(body, &input)
// [omitted error handling]

if utf8.ValidString(input.Query) {
    if strings.ContainsRune(input.Query, '�') {
        log.Printf("query string '%s' is valid utf-8 but contains a replacement character", input.Query)
    } else {
        log.Printf("query string '%s' is valid utf-8", input.Query)
    }
} else {
    log.Println("query string is not valid utf-8")
}
response.WriteHeader(http.StatusOK)
fmt.Fprintln(response, "looks good")
```

No matter what I threw at it, it was always handles gracefully. An escaped surrogate code point? Replaced with the replacement character. An unescaped surrogate code point? Replaced with three replacement characters. An unescaped `0xff`-byte? Replaced with the replacement character. An overlong encoding of `"}`? Replaced with eight replacement characters.

So while I'm a bit unhappy that it does this silently and I have not found an option to let the parsing fail instead of replacing unexpected bytes, this is at least valid behaviour and leads to valid encoding.

### Conclusions

Always check strings from untrusted sources for valid encoding. I cannot stress this enough: Most standard library functions will ignore encoding! You have to check manually. `json.Unmarshal` may have your back, but unless you _know for certain_ that this is the case, always check (in addition to other security measures you should take with untrusted input).

Also: do not split strings in arbitrary places, do not cast byte arrays to strings without checking for valid encoding and be very careful with escape sequences in string literals.
