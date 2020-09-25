Stranger Than Usual blog generator
==================================

This is a static HTML generator for my blog „[Stranger Than Usual](https://blog.strangerthanusual.de/)“.
Right now (2020-09-25) Stranger Than Usual is still dynamically created via ruby on rails.

There are several reasons why I don't want to use rails anymore. The most important ones are:
- I have to keep updating ruby on rails, which is time consuming and may break stuff. If I do not update, security holes will stay unfixed.
- I don't really need a dynamically generated website, static HTML will do just as well, is more secure and probably faster.

One important disclaimer: __This is a single purpose generator__. If you want a statically generated website, use one of
many static website generators, for example:
- [HUGO](https://gohugo.io/)
- [Cobalt](https://github.com/cobalt-org/cobalt.rs)
- [Jekyll](https://jekyllrb.com/)

All of those are more flexible (i.e. easier to adjust to your use case), better maintained and probably less buggy.

So why did I write this generator at all? Simple: Because I could. Because I wanted to. And because it makes it easier to
port my old blog to the static site generator without breaking too many links. That being said, it took a lot of time to write
compared to just setting up a site with an existing static generator.

Usage
-----

Go to the main repository directory and call
```
make
```

Start a local server with
```
bin/run-test-server
```

Requirements
------------

- a current version of [rust](https://www.rust-lang.org/) and cargo (rust's build tool) for the generator itself
- [GNU make](https://www.gnu.org/software/make/) (or something compatible, if there still is such a thing) to manage all the generating
- the [brotli command line tool](https://github.com/google/brotli) for brotli compression
- the [zopfli command line tool](https://github.com/google/zopfli) for gz compression (or you can just use gz, but then you need to change compression.makefile)
- [nginx](https://nginx.org/) (and possibly some plugins I implcitely used) to run a local test server
- [sassc](https://github.com/sass/sassc) for compiling the style sheets
- maybe some other stuff I forgot to mention

Files / directories
-------------------

- generator: The rust tool for the html generation
- content: the raw blogposts and other content files
- assets: static assets for the blog (such as scss)
- bin: one or more helpful scripts
- cfg: configurations files for test and live deployments
- dist: where all the generated stuff is put (not checked in, can be safely removed after usage)
