# Using blog-tools

A few months ago, I wrote an article on making a blog in Rust. Since then, this idea has had quite a bit of development. I needed to make blogs at work too. This was a good opportunity to improve the code. One thing lead to another and eventually I published the [blog library on crates.io](https://crates.io/crates/blog-tools). You can install this like you would any other Rust crate with 

```
cargo add blog-tools
```

In this article I will walk you through setting up a basic blog.

Start by making a new Rust project with

```
cargo init basic-blog
```

First things first, for this tutorial we will be using Rocket, which currently requires the nightly toolchain, so add a file next to the `Cargo.toml` file called `rust-toolchain.toml` and add the following to it

```
[toolchain]
channel = "nightly"
```

Once that's been done, let's add our dependencies. For this tutorial you need to add these deps to your `Cargo.toml`

```
[dependencies]
tokio = { version = "1.29.1", features = ["full"] }
tera = "1.15.0"
rocket = "0.5.0-rc.3"
rocket_codegen = "0.4.4"
rocket_dyn_templates = { version = "0.1.0-rc.1", features = ["tera"] }
lazy_static = "1.4.0"
blog-tools = "0.0.2"
```

We'll be using Rocket for the web server, Tera for the templating and my blog-tools for the blog logic.

Next, next to the `Cargo.toml` file you need to make an `assets` and `templates` folder. Let's start a basic web server pointing at those folders

```
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    let port = 8080_u16;

    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));

    if let Err(e) = rocket::custom(figment)
        .mount("/", FileServer::from(relative!("assets/")))
        .register("/", catchers![])
        .attach(Template::fairing())
        // .attach(config)
        .mount("/", routes![ping])
        // .manage(bucket_info)
        .launch()
        .await
    {
        println!("Did not run. Error: {:?}", e)
    }
}

#[derive(Responder)]
#[response(status = 200, content_type = "text")]
struct RawOkText(&'static str);

#[get("/ping")]
fn ping() -> RawOkText {
    return RawOkText("pong");
}

```

That's all you need in terms of the server. We'll add a few more routes in a moment. Now, if you run this project, and go to `localhost:8080/ping` you should see "pong". This ping-pong is good for using to monitor your blog if you host it, by the way.

We're going to have three endpoints for the blog itself. We want a blog index which lists all the blogs we have. We also want a blog page for an individual blog. Finally, since blog-tools supports tags, we can have a page that shows off which blogs belong to which tags.

Let's add some super simple CSS to make this whole thing look a little nice. Inside the `assets` folder add a folder called `css`. Inside that folder add `style.css`. Add the following CSS 

```
.blog-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-around;
    gap: 2rem;
    max-width: 1200px;
}

.blog-card {
    display: flex;
    flex-direction: column;
    width: clamp(20rem, calc(20rem + 2vw), 22rem);
    box-shadow: 0 .1rem 1rem rgba(0, 0, 0, 0.1);
    border-radius: 1em;
    background: #f4f4f4;
    overflow: hidden;
    flex-grow: 1;
    padding: .25em .75em;
    border-radius: 1em;
    border: solid 1px black;
}

.blog-card:hover {
    background: pink;
}

.blog-tag-container {
    display: flex;
    flex-wrap: wrap;
}

.blog-tag {
    padding: .25em .75em;
    border-radius: 1em;
    font-size: 1rem;
    background: linear-gradient(135deg, #cccccc, #ffffff);
    color: blue;
}
```

CSS is obviously very customisable, so feel free to change this as much as you like. 

Now, inside `templates` make `blog_index.html.tera`, add

```
<!DOCTYPE html>
<html lang="en">

<head>
    <meta http-equiv="content-type" content="text/html; charset=UTF-8">
    <title>Blog</title>
    <link rel="stylesheet" type="text/css" href="/css/style.css">
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>

<body>
    <header>
        <h1>Blog</h1>
    </header>
    <main>


        {% if blog.entries %}

        <div class="blog-container">
            {% for entry in blog.entries %}

            <div class="blog-card">
                <a href="/blog/{{ entry.slug }}" class="no-underline black-links">
                    <div class="card-body">
                        <h5>
                            <span>{{ entry.date }}</span>
                        </h5>
                        <div>
                            {{entry.preview | safe}}
                        </div>
                    </div>
                </a>

                <div class="blog-tag-container">
                    {% for tag in entry.tags %}
                    <a href="/blog/tag/{{ tag }}" class="blog-tag">#{{ tag }}</a>
                    {% endfor %}
                </div>

            </div>

            {% endfor %}
        </div>
        {% else %}
        <p>No blog found</p>

        {% endif %}
    </main>
</body>
```

This is looping through each of the blogs that we will provide, and adding them into a little CSS card, with a link.

Now make `blog.html.tera` and add

```
<!DOCTYPE html>
<html lang="en">

<head>
    <meta http-equiv="content-type" content="text/html; charset=UTF-8">
    <title>{{ blog.title }}</title>
    <link rel="stylesheet" type="text/css" href="/css/style.css">
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">

    {% if blog.author_name %}
    <meta name="author" content="{{ blog.author_name }}">
    {% endif %}

    {% if blog.author_webpage %}
    <link rel="author" href="{{ blog.author_webpage }}">
    {% endif %}

    {% if blog.keywords %}
    <meta name="keywords" content="
    {% for word in blog.keywords %}
    {{word}},
    {% endfor %}
    ">
    {% endif %}

    {% if blog.desc %}
    <meta name="description" content="{{ blog.desc }}">
    {% endif %}
</head>

<body>
    <main>

        {{ blog.html | safe }}

    </main>

</body>

</html>
```

This is where the text for our blog will go. As you can see in the head, the blog-tools crate supports a wide variety of SEO tools.

Finally, make a `tags.html.tera` and add the following

```
<!DOCTYPE html>
<html lang="en">

<head>
    <meta http-equiv="content-type" content="text/html; charset=UTF-8">
    <title>Tags</title>
    <link rel="stylesheet" type="text/css" href="/css/style.css">
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>

<body>
    <header>
        <h1>#{{ tag }}</h1>
    </header>
    <main>


        {% if blogs %}

        <div class="blog-container">
            {% for entry in blogs %}

            <div class="blog-card">
                <a href="/blog/{{ entry.slug }}" class="no-underline black-links">
                    <div class="card-body">
                        <h5>
                            <span>{{ entry.date }}</span>
                        </h5>
                        <div>
                            {{entry.preview | safe}}
                        </div>
                    </div>
                </a>

                <div class="blog-tag-container">
                    {% for tag in entry.tags %}
                    <a href="/blog/tag/{{ tag }}" class="blog-tag">#{{ tag }}</a>
                    {% endfor %}
                </div>

            </div>

            {% endfor %}
        </div>

        {% else %}
        <p>No blog found</p>

        {% endif %}
    </main>
</body>
```

The tags page will list all the blogs that have that specific tag.

Now make a new folder next to your `Cargo.toml` called `blog`. Let's make some blog articles! You can organise this folder however you like, but the one rule is every markdown file must have a JSON file next to it which share the same name (except for the extension).

A structure that I found helpful was 

- blog
    - 2023
        - 2023-01-01
            - my_first_blog.json
            - my_first_blog.md
        - (other folders)
        
So, I have a folder for the year, and then inside that a folder with the publication date for each blog. Again, it doesn't really matter how you choose to structure this. For the purposes of this tutorial you can put whatever you like in the markdown files. The JSON files have to have a very specific format

```
{
"title": String,
"date": ISO 8601 Date i.e. YYYY-MM-DD,
"desc": Optional<String>,
"slug": String,
"tags": [String],
"keywords": Optional<[String]>,
"canonical_link": Optional<String>,
"author_name": Optional<String>,
"author_webpage": Optional<String>
}
```

For instance, here's the JSON file for one of the blogs 

```
{
    "title": "My First Blog",
    "date": "2023-08-18",
    "desc": "A blog!",
    "slug": "first-blog",
    "tags": [
        "philosophy",
        "blog",
        "science"
    ],
    "keywords": [
        "blog",
        "first",
        "time",
        "paper",
        "rock"
    ],
    "author_name": "Indigo Curnick",
    "author_website": "https://indigocurnick.xyz"
}
```

Once you have some blogs, back to `main.rs`. Add the following to actually make the blog 

```
pub static BLOG_ROOT: &str = "blog";

lazy_static! {
    pub static ref STATIC_BLOG_ENTRIES: Blog = get_blog(PathBuf::from(BLOG_ROOT), None, None);
}

fn get_blog_context() -> &'static Blog {
    return &STATIC_BLOG_ENTRIES;
}
```

And that's it! The blog-tools crate is actually really easy to use. There's some configuration options which I won't go over here, but you can check the docs for them.

Finally, we need to add our three endpoints

```
#[get("/blog")]
fn blog_index() -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("blog", get_blog_context());
    Some(Template::render("blog_index", context.into_json()))
}

#[get("/blog/<slug>")]
fn blog_article(slug: String) -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    let all_blogs = get_blog_context();
    let this_blog = match all_blogs.hash.get(&slug) {
        Some(x) => x,
        None => return None,
    };
    context.insert("blog", this_blog);
    Some(Template::render("blog", context.into_json()))
}

#[get("/blog/tag/<slug>")]
fn tag_page(slug: String) -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    let all_blogs = get_blog_context();

    let mut these_blogs: Vec<&BlogEntry> = vec![];

    for blog in &all_blogs.entries {
        if blog.tags.contains(&slug) {
            these_blogs.push(&blog);
        }
    }

    context.insert("blogs", &these_blogs);
    context.insert("tag", &slug);
    Some(Template::render("tags", context.into_json()))
}
```

Now we have a way to take an incoming request, and process the correct template using the blog-tools crate. Don't forget to add these routes to the server!

```
...
.mount("/", routes![ping, blog_index, blog_article, tag_page])
...
```

If you run again and go to `localhost:8080/blog` you should see a nice blog, all templated out with cards. 

If you'd like to see the full code for this tutorial, it's inside the [blog-tools repo](https://github.com/IndigoCurnick/blog-tools). You can also run it from the repo using 

```
cargo +nightly run --example blog  
```

That's really all you need to know in order to make a simplistic blog in Rust!