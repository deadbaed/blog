use std::time::SystemTime;

fn main() {
    let sys_time = SystemTime::now();
    let timestamp = sys_time
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("current timestamp")
        .as_secs()
        .try_into()
        .expect("timestamp in i64");

    #[cfg(debug_assertions)]
    let host = "http://localhost:4343";
    #[cfg(debug_assertions)]
    let base_url = "/";

    #[cfg(not(debug_assertions))]
    let host = "https://philippeloctaux.com";
    #[cfg(not(debug_assertions))]
    let base_url = "/blog/";

    let assets = "./assets/".into();
    let paths = leptos_ssg::Paths {
        target: "./target/blog".into(),
        #[cfg(feature = "prod")]
        opengraph: "./target/opengraph".into(),
    };
    let styles = leptos_ssg::Styles {
        website: "style.css",
        #[cfg(feature = "prod")]
        opengraph: "opengraph_style.css",
    };
    let config = leptos_ssg::BuildConfig::new(
        host,
        base_url,
        timestamp,
        styles,
        assets,
        "philt3r.png",
        "deadbaed",
        "broke my bed, now it's dead",
        "Philippe Loctaux",
        Some("https://philippeloctaux.com"),
        "deadbaed-dead-4444-baed-dddeadbaeddd",
        #[cfg(feature = "prod")]
        "http://localhost:4444",
    )
    .unwrap();
    let content_path: std::path::PathBuf = "./content/".into();
    let mut blog = leptos_ssg::Blog::new(paths, config);

    let content = leptos_ssg::Content::scan_path(&content_path).unwrap();

    #[cfg(debug_assertions)]
    fn debug_auto_reload() -> leptos::prelude::AnyView {
        use leptos::prelude::*;

        view! {
        <script
        data-interval="2500"
        data-debug
        inner_html=r#"
// https://github.com/Kalabasa/simple-live-reload/blob/bb65d8b3f19af8c6477cdefb0c718c1db3d0cb69/script.js

/*Copyright 2025 Lean Rada.
  Permission is hereby granted, free of charge, to any person obtaining a copy of this
software and associated documentation files (the “Software”), to deal in the Software without restriction, including
without limitation therights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the
Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
  The above copyright notice and this permission notice shall be included in all copies or substantial portions of the
Software.
  THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.*/
if (
  location.hostname === "localhost" ||
  location.hostname === "127.0.0.1" ||
  location.hostname === "[::1]"
) {
  const interval = Number(document.currentScript?.dataset.interval || 1000);
  const debug = document.currentScript?.hasAttribute("data-debug") || false;

  let watching = new Set();
  watch(location.href);

  new PerformanceObserver((list) => {
    for (const entry of list.getEntries()) {
      watch(entry.name);
    }
  }).observe({ type: "resource", buffered: true });

  function watch(urlString) {
    if (!urlString) return;
    const url = new URL(urlString);
    if (url.origin !== location.origin) return;

    if (watching.has(url.href)) return;
    watching.add(url.href);

    if (debug) {
      console.log("[simple-live-reload] watching", url.href);
    }

    let focused = false;
    let etag, lastModified, contentLength;
    let request = { method: "head", cache: "no-store" };

    async function check() {
      try {
        if (document.hidden) return;
        if (focused) return;
      } finally {
        focused = document.hasFocus();
      }

      const res = await fetch(url, request);
      if (res.status === 405 || res.status === 501) {
        request.method = "get";
        request.headers = {
          Range: "bytes=0-0",
        };
        return check();
      }

      const newETag = res.headers.get("ETag");
      const newLastModified = res.headers.get("Last-Modified");
      const newContentLength = res.headers.get("Content-Length");

      if (
        (etag && etag !== newETag) ||
        (lastModified && lastModified !== newLastModified) ||
        (contentLength && contentLength !== newContentLength)
      ) {
        if (debug) {
          console.log("[simple-live-reload] change detected in", url.href);
        }
        try {
          location.reload();
        } catch (e) {
          location = location;
        }
      }

      etag = newETag;
      lastModified = newLastModified;
      contentLength = newContentLength;
    }

    check();
    setInterval(check, interval);
    document.addEventListener(
      "visibilitychange",
      () => !document.hidden && check()
    );
  }
}"#></script>
        }.into_any()
    }

    fn additional_js() -> Option<leptos::prelude::AnyView> {
        use leptos::prelude::*;

        let additional_js = view! {
            <script inner_html=r#"
            window.goatcounter = {
                path: function(p) { return location.host + p }
            };
        "#></script>
            <script data-goatcounter="https://goatcounter.philt3r.eu/count" async src="https://goatcounter.philt3r.eu/count.js"></script>
            {
              #[cfg(debug_assertions)]
              debug_auto_reload()
            }
        };
        Some(additional_js.into_any())
    }
    blog.add_404_page(additional_js);
    blog.add_index_page(&content, additional_js);
    blog.add_content_pages(&content, additional_js)
        .expect("processed markdown files");

    blog.add_content_assets(&content_path, &content);
    blog.add_atom_feed(&content);

    let path = blog.build().expect("files written to disk");
    println!("Wrote files to {}", path.display());
}
