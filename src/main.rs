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
    let target = "./target/blog".into();
    let config = leptos_ssg::BuildConfig::new(
        host,
        base_url,
        timestamp,
        "style.css",
        assets,
        "philt3r.png",
        "deadbaed",
        "broke my bed, now it's dead",
        "Philippe Loctaux",
        Some("https://philippeloctaux.com"),
        "deadbaed-dead-4444-baed-dddeadbaeddd",
    )
    .unwrap();
    let content_path: std::path::PathBuf = "./content/".into();
    let mut blog = leptos_ssg::Blog::new(target, config);

    let content = leptos_ssg::Content::scan_path(&content_path).unwrap();

    fn additional_js() -> Option<leptos::prelude::AnyView> {
        use leptos::prelude::*;

        let additional_js = view! {
            <script inner_html=r#"
            window.goatcounter = {
                path: function(p) { return location.host + p }
            };
        "#></script>
            <script data-goatcounter="https://goatcounter.philt3r.eu/count" async src="https://goatcounter.philt3r.eu/count.js"></script>
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
