+++
title = "Migrating my blog to my own engine"
date = 2025-08-05T02:48:36+02:00[Europe/Paris]
uuid = "47f8749a-5599-434d-b300-856b576e63f8"
description = "Why am I creating my own blog engine"
+++

## Begginings

I created my blog in 2016 with a very popular blog engine at the time: [Jekyll](https://jekyllrb.com). Combined with hosting provided by GitHub Pages, it is a quick way to start publishing content. I did not write a lot when I started my blog, so it was not a big deal. That worked fine for a couple of years, until I was tired of managing Ruby dependencies and warnings, and I remember getting a bit of trouble when I was running Windows.

At some point I switched to [Zola](https://getzola.org), and I liked it a lot: instead of managing many dependencies, there is a single binary to run to create/preview/build websites, and a good feature set out of the box.

But so far the look of my blog was pretty generic, for a simple reason: I disliked writting CSS. In fact, I prefered searching for a theme that I could use rather than spending time making my own. Fortunately, there is a big pool of [Zola](https://www.getzola.org/themes/) and [Jekyll](https://jekyllrb.com/docs/themes/) themes ready to be used, so it was prefect for me!

I started by blog when I did not know how to make web applications, but now I can! And now I can write some CSS, thanks to a wonderful tool named [TailwindCSS](https://tailwindcss.com). Tailwind made me tolereate writting CSS, it helps me a lot to understand what I do, now it is making me enjoy writting CSS!

A couple of years ago I redesigned [my website](https://philippeloctaux.com), and started to use [Leptos](https://leptos.dev) for the templating / components part. It turns out I like Leptos *a lot* to make reusable components.

I wanted to give a fresh look to my blog, but I did not want to write HTML templates by hand or deal with raw CSS, I wanted to do it with Leptos and Tailwind! While browsing the web I found someone [building their blog with Leptos and TailwindCSS](https://blog.jlewis.sh/post/building-this-blog) as well!

## Feature set

In my opinion, static sites generators are awesome for blogs -- You write a piece a content, give it to a tool, and it will emit files that are ready to be viewed in a web browser. That is pretty powerful, and makes the website portable, no lock-in to a specific tool! So my new blog had to be statically generated.

I want to have an Web feed, it is non negociable. Web feeds are awesome, Zola and Jekyll generate an Atom feed by default, so my blog will have an Atom feed.

It has to look nice, whether on desktop or mobile. Tailwind makes this easy: simply write the CSS for a mobile screen, and make some adjustements when making the screen bigger.

Something I wanted to have is nice code blocks, I used [`highlight.js`](https://highlightjs.org/) for this, it's a pretty nice library! If JavaScript is completely blocked on the reader's browser this is not a problem, there is a fallback option which shows the code block without any syntax highlighting. Currently highlight.js is being pulled from a CDN, it could be vendored at some point, the JavaScript files would be need to be downloaded during build time.

### Content

All content is written in [Markdown](https://commonmark.org), the excellent crate [`pulldown-cmark`](https://crates.io/crates/pulldown-cmark/) is used to parse the content.

A tiny bit of metadata is required in every piece of content:

- `title`: The Tagline of the content. Used on the homepage and on the Web feed.
- `date`: Moment of publication. I started to use the crate [jiff](https://docs.rs/jiff/0.2.15/jiff/fmt/temporal/index.html) in all my projects, I like its API. An example moment is `2025-07-01T12:56:42+02:00[Europe/Paris]`, I discovered [RFC 9657](https://www.rfc-editor.org/rfc/rfc9557.html) extends [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html), it allows the IANA Time Zone to be in the date/time string!
- `uuid`: The unique identifier of the content, it is only used for the Web feed.

#### Filename

To be considered for inclusion, the filename of every piece of content must either be:
- `YYYY-MM-DD-slug.md` when there is no additional assets
- `YYYY-MM-DD-slug/index.md` when assets will be put next to the `index.md` file.

The `slug` will appear in the URL of the content. It can be anything, but it **must not change after publication**, otherwise it will be published as a new content.

The `YYYY-MM-DD` part is not used on the final website anywhere, it is simply there to make the files sort naturally by name on the filesystem. It also checked against the `date` tag in the metadata in case a typo was made.

#### Custom components

During markdown parsing, when an HTML block is detected, HTML tags are scanned against a list of custom HTML tags in `leptos_ssg` to see if they match (they must not clash with HTML tags from the W3C).

If there is a match, a Leptos view is rendered on the HTML page, but not on the Web feed (to keep things simple, since all Web feed readers might not display it correctly). The content author should put a paragraph above or below the custom component to explain what is happening, for Web browsers without JavaScript (if the component requires it), or for Web feed readers.

The first custom component is `ImageGrid`, which will render a grid of images from a folder. It can be used like this:

```html
&lt;ImageGrid src="path-to-images/" /&gt;
```

And it will look like this (for Web feed readers, here is [the folder with sample images](sample-images/)):

<ImageGrid src="sample-images/" />

### Web feed

An Atom feed is generated, and not an RSS feed, because the tools I used previously generated Atom feeds, and because it is standardized. It seems a bit simpler to understand compared to RSS 1.0 and RSS 2.0, at a first glance.

Every piece of content requires to have its own UUID, to make sure there will never be any conflicts. I would like to thank those sites:

- [https://www.alanwsmith.com/en/2a/ne/cw/if/](https://www.alanwsmith.com/en/2a/ne/cw/if/) For a quick "getting started" guide
- [https://peterbabic.dev/blog/using-uuid-in-atom-feed/](https://peterbabic.dev/blog/using-uuid-in-atom-feed/) For explaining why UUIDs are needed

In the Web feed, the UUID of every piece of content is a UUIDv5 composed of:
- The UUIDv4 of the site, which is `deadbaed-dead-4444-baed-dddeadbaeddd` (very random huh?)
- The UUIDv4 of the piece of content

That way, every single piece of content is guaranteed to always have the same UUID since the moment of publication.

The only downside to this approach is to remember to generate an UUIDv4 when writting new content, but it should be fine.

## It will never be finished

I am aware that a blogging engine will never be finished and it will never have all the features big engines have, and it will have some bugs.

But I am very happy to make my own tools, and use them in production!

The name of the engine is `leptos_ssg`, the source code is [available here](https://github.com/deadbaed/leptos_ssg), and there is an [instance to showcase the engine](https://deadbaed.github.io/leptos_ssg/)!

## Write more

I hope re-building my blog (and *accidentally* writting my own blog engine) will encourage me to write more!

And the writting better be about projects I am working on, and not about the blog engine itself 😁
