# Blog Tools Version 0.1.2

Since the last time I wrote about `blog-tools` the crate has had quite a bit of development, so I thought now would be a good time to shout out about it.

First, the create now comes with three working examples of a blog you can run right away. Simply clone the repo at https://github.com/IndigoCurnick/blog-tools and run

```
cargo run --example high
cargo run --example medium
cargo run --example low
```

What do the `high`, `medium` and `low` refer to? That's the bulk of the new features! `blog-tools` now has a bunch of features to help with managing RAM.

`high` refers to high RAM usage - the entire blog is stored in memory the whole time. `medium` refers to medium RAM usage - most of the metadata is stored in memory but the blog posts themselves stay on disc till needed. `low` refers to (you guessed it) low RAM usage - nothing is stored in memory at all. This would also be suitable in a serverless application.

## `high`

Let's look at the `high` module in more detail. Before version 0.1.0, this was the only mode supported. The main crux remains the same as before - place the blog, now the `HighBlog` struct, in a `lazy_static`

```
pub static BLOG_ROOT: &str = "examples/blog";

lazy_static! {
    pub static ref STATIC_BLOG_ENTRIES: HighBlog =
        get_high_blog(PathBuf::from(BLOG_ROOT), None, None).unwrap();
}
```

You can then use this in just the same was as before, for example, in Rocket we could do something like

```
#[get("/blog")]
fn blog_index() -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("blog", get_blog_context());
    Some(Template::render("blog_index", context.into_json()))
}
```

And then the template we can make cards for the blog previews like so 

```
<div class="blog-container">
    {% for entry in blog.entries %}

    <div class="blog-card">
        <a href="/blog/{{ entry.slug }}" class="no-underline black-links">
            <div class="card-body">
                <h4>
                    <span>{{ entry.title }}</span>
                </h4>
                <h5>
                    <span>{{ entry.date }}</span>
                </h5>
                <div>
                    {{entry.preview | safe}}
                </div>
            </div>
        </a>

        <p><code>blog-tools</code> also supports tags - try clicking one!</p>
        <div class="blog-tag-container">

            {% for tag in entry.tags %}
            <a href="/blog/tag/{{ tag }}" class="blog-tag">#{{ tag }}</a>
            {% endfor %}
        </div>

    </div>

    {% endfor %}
</div>
```

I won't cover every single way you can use it here as that's detailed in the example code!

## `medium`

With `medium` we still place the `MediumBlog` in a lazy static like so

```
pub static BLOG_ROOT: &str = "examples/blog";

lazy_static! {
    pub static ref STATIC_BLOG_ENTRIES: MediumBlog =
        get_medium_blog(PathBuf::from(BLOG_ROOT), None, None).unwrap();
}
```

The code for the preview page looks just the same!

```
#[get("/blog")]
fn blog_index() -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("blog", get_blog_context());
    Some(Template::render("blog_index", context.into_json()))
}
```

As does the template code, so I won't repeat it here. The difference comes when we want to return a specific blog post.

```
#[get("/blog/<date>/<slug>", rank = 2)]
fn blog_article(date: String, slug: String) -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    let all_blogs = get_blog_context();
    let complete_slug = format!("{}/{}", date, slug);
    let this_blog = match all_blogs.hash.get(&complete_slug) {
        Some(x) => x,
        None => return None,
    };

    context.insert(
        "blog",
        &this_blog
            .render(PathBuf::from_str(BLOG_ROOT).unwrap())
            .unwrap(),
    );
    Some(Template::render("blog", context.into_json()))
}
```

Since we don't have the actual blog entry in memory, we need to call this `render` method! This actually produces the HTML from the markdown on the disc then and there

## `low`

`low` works quite differently. Since we don't want to keep anything in memory here, there is no lazy static. Instead, we have to read everything from disc every time a request comes in. For instance, our main blog preview is 

```
#[get("/blog")]
fn blog_index() -> Option<Template> {
    // I only use this dummy struct to keep consistency with the other two blog modes
    #[derive(Serialize, Deserialize)]
    struct Blogs {
        entries: Vec<PreviewBlogEntry>,
        tags: Vec<String>,
    }

    let mut context = rocket_dyn_templates::tera::Context::new();

    let preview = preview_blogs(PathBuf::from_str(BLOG_ROOT).unwrap(), 2, None).unwrap();
    let tags = get_blog_tag_list(PathBuf::from_str(BLOG_ROOT).unwrap()).unwrap();
    context.insert(
        "blog",
        &Blogs {
            entries: preview,
            tags,
        },
    );
    Some(Template::render("blog_index", context.into_json()))
}
```

Notice in the line 

```
let preview = preview_blogs(PathBuf::from_str(BLOG_ROOT).unwrap(), 2, None).unwrap();
```

The second argument, the 2 in this case, is specifying the number of blog posts we want it to preview. Since `low` is intended for instances where the whole blog can't fit into memory at once, you almost certainly can't preview the entire blog. So, this will only bother to read the most recent N blogs!

When we want to get a specific blog

```
#[get("/blog/<date>/<slug>", rank = 2)]
fn blog_article(date: String, slug: String) -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    let blog_post =
        render_blog_post(PathBuf::from_str(BLOG_ROOT).unwrap(), date, slug, None).unwrap();

    context.insert("blog", &blog_post);
    Some(Template::render("blog", context.into_json()))
}
```

We again have to read it from the disc.

## Conclusion

I hope you find this crate helpful! You can find the repo [here](https://github.com/IndigoCurnick/blog-tools), the crates.io entry at [here](https://crates.io/crates/blog-tools) and the documentation [here](https://docs.rs/blog-tools/0.1.2/blog_tools/).

As of now, `blog-tools` is still very much a personal project I built for my own website. If you use it, I'd love to hear your feedback!! You can open issues on the GitHub, or [email me](mailto:indigocurnick@gmail.com)