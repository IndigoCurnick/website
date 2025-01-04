In this piece you will learn how to:

- Create a blog in Rust;
- using Rocket and Tera;
- that reads from markdown files;
- creates the blog pages automatically;
- manages the tags automatically.

You will not learn how to:

- Program in Rust (you need some background to follow this tutorial);
- styling the webpages (CSS);
- host this blog.

Let's get started. 

## Setting Up the Project 

Start by running 

```
cargo init blog
```

Add the following packages to your `Cargo.toml`

```
serde = { version = "1.0.134", features = ["derive"] }
serde_json = "1.0.77"
tera = "1.15.0"
rocket = "0.5.0-rc.1"
rocket_codegen = "0.4.4"
rocket_dyn_templates = { version = "0.1.0-rc.1", features = ["tera"] }
lazy_static = "1.4.0"
markdown = { git = "https://github.com/IndigoCurnick/markdown.rs", rev = "83a9c180f95fbfd7a688ad142263a8453d11c502" }
walkdir = "2.3.2"
chrono = { version = "0.4.23", features = ["serde"] }
```

`serde` and `serde_json` are used for serialising our structs. `rocket` will be the backend, and handle requests. `tera` is a templating language for HTML pages. Templating is how we kill work. Before I discovered templating I was manually making an endpoint for every single blog post - not sustainable!

 `markdown` is my fork of the markdown crate. I made this fork to add some extra features to the crate. Little did I know, there was actually a new crate made which has all of the features I added, which you can find [here](https://github.com/wooorm/markdown-rs). Feel free to use my fork, but I would suggest using this new crate if you can.

You will also need to use the nightly toolchain. The easiest way to do this is to make the file `rust-toolchain.toml` next to your `Cargo.toml` and add

```
[toolchain]
channel = "nightly"
```

to it.

## Creating the Blog Structs

Let's start by making the structs we need for the blog itself.

The blog will function in the following way: on boot, the server will look at all the markdown and json files in a directory. From this, it will construct the appropriate webpages. Let's start with the JSON

```
#[derive(Debug, Serialize, Deserialize)]
pub struct BlogJson {
    pub title: String,
    pub date: NaiveDate,
    pub desc: String,
    pub slug: String,
    pub tags: Vec<String>,
}
```

This JSON contains the metadata associated with the blog post. You could add or remove from this as you see fit to your use case. For example, if your blog had multiple writers, you could add an author field here. The `slug` is the url friendly extension. For example, the JSON file for my last article is 

```
{
    "title": "Tár is (Mostly) Innocent",
    "date": "2023-04-15",
    "desc": "My thoughts on Lydia Tár",
    "slug": "tar-is-innocent",
    "tags": [
        "film"
    ]
}
```

Now that we have a JSON, we can make the `BlogEntry` struct

```
type Slug = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogEntry {
    pub title: String,
    pub date: NaiveDate,
    pub desc: String,
    pub html: String,
    pub slug: Slug,
    pub tags: Vec<String>,
}
```

The purpose of the `BlogEntry` struct is to fuse the information from the JSON and the markdown file together. You can see that it only has one extra field compared with the `BlogJson` struct - `html`. This is the HTML of the actual blog post itself. We'll worry about converting the markdown files into HTML later. Adding the type `Slug` wasn't really all that necessary, and I probably wouldn't do it again if I was making this from scratch. 

```
impl BlogEntry {
    pub fn new(json: BlogJson, html: String) -> Self {
        return BlogEntry {
            title: json.title,
            date: json.date,
            desc: json.desc,
            html: html,
            slug: json.slug,
            tags: json.tags,
        };
    }
}
```

I think the above implementation pretty much speaks for itself.

Now for the whole blog we make the `Blog` struct 

```
#[derive(Debug, Serialize, Deserialize)]
pub struct Blog {
    pub hash: HashMap<Slug, BlogEntry>,
    pub entries: Vec<BlogEntry>,
    pub tags: Vec<String>,
}
```

`tags` will contain each unique tag once. `hash` is a `HashMap` that correlates the url slug to the `BlogEntry`. The entries is date-orderd `Vec` of the `BlogEntries`. This does mean that I duplicate the `BlogEntries`, increasing the RAM usage. But, since I want to be able to both iterate through the blog posts in date order AND access them by the slug I decided that for my use case, this extra memory usage was not a big deal given that it would decrease run time. Other implementations are possible to change this trade-off. Once idea I had was storing a date-orderd `Vec` of the slugs, and accessing the `BlogEntry` from the `HashMap` using them.

Then I created the following two functions 

```
fn get_blog_paths(base: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let base = PathBuf::from(base);
    if !base.is_dir() {
        panic!("BLOG_ROOT is not a directory!")
    }
    let mut markdown_files: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(base.clone()) {
        let entry = entry?;

        let name = match entry.file_name().to_str() {
            Some(x) => x,
            None => continue,
        };

        if name.contains(".json") {
            continue;
        }

        if !name.contains(".md") {
            continue;
        }

        markdown_files.push(PathBuf::from(entry.path()));
    }
    markdown_files.sort();
    markdown_files.reverse();
    Ok(markdown_files)
}

pub fn get_blog_entries(base: PathBuf) -> Blog {
    // TODO: Error
    let blog_paths = get_blog_paths(base).unwrap();

    let mut hashes: HashMap<Slug, BlogEntry> = HashMap::new();
    let mut entires: Vec<BlogEntry> = vec![];
    let mut tags: Vec<String> = vec![];

    for blog in blog_paths {
        let mut json_path = blog.parent().unwrap().to_path_buf();
        let name_split: Vec<&str> = blog
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(".")
            .collect();
        let name = format!("{}.json", name_split[0]);

        json_path.push(name);
        println!("json path is {}", json_path.display());
        let json_text = match fs::read_to_string(json_path) {
            Ok(x) => x,
            Err(_y) => continue,
        };

        let json_data: BlogJson = serde_json::from_str(&json_text).unwrap();

        let these_tags = json_data.tags.clone();

        let markdown = fs::read_to_string(blog).unwrap();

        let html = to_html(&markdown);

        let blog_entry = BlogEntry::new(json_data, html);

        hashes.insert(blog_entry.slug.clone(), blog_entry.clone());
        entires.push(blog_entry);

        for tag in these_tags {
            if !tags.contains(&tag) {
                tags.push(tag);
            }
        }
    }

    return Blog {
        hash: hashes,
        entries: entires,
        tags: tags,
    };
}
```

`get_blog_paths` just finds all of the markdown files in the directory you point it to. `get_blog_entries` actually opens the files, converts to HTML, reads the JSONs, sorts the blogs and returns the whole `Blog`. I think that both of these functions could be made more efficient in future. I particularly don't like how I ignore the JSON files in the `get_blog_paths` function and then reconstruct the JSON file name in the `get_blog_entries` function. This does raise an important note: the markdown and JSON files for each blog entry need to have the same name except the extension, and they should be next to each other.

## Context

Now that we can create all the blog entries, we hold them in memory forever. I use a `lazy_static` to do this. Since RAM is not an issue in my deployment, I decided that just keeping the entire blog in memory the whole time was the best idea. However, for other applications you may need to just keep the metadata in memory and read the actual blog from disk. 

```
pub static BLOG_ROOT: &str = "blog";

lazy_static! {
    pub static ref STATIC_BLOG_ENTRIES: Blog = get_blog_entries(PathBuf::from(BLOG_ROOT));
}
```

## Server 

Rocket makes building a backend very easy. This blog will have the following structure. A homepage (which I call index), a blog index, blog pages, and tag pages.

```
 #[get("/index")]
async fn index() -> Template {

    let mut context = rocket_dyn_templates::tera::Context::new();

    let blog_context = get_blog_context();
    context.insert("tags", &blog_context.tags);
    context.insert("blog", get_blog_context());
    Template::render("index", context.into_json())
}

#[get("/blog")]
async fn blog_index() -> Template {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("blog", get_blog_context());
    Template::render("blog_index", context.into_json())
}

fn get_blog_context() -> &'static Blog {
    return &STATIC_BLOG_ENTRIES;
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

#[catch(404)]
async fn not_found(req: &Request<'_>) -> Redirect {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("url", req.uri());
    Redirect::to(uri!(index))
}

#[catch(500)]
async fn error(req: &Request<'_>) -> Redirect {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("url", req.uri());
    Redirect::to(uri!(index))
}
```
 
 There isn't a whole lot to say about this. I made the little `get_blog_context` function to help with the lifetimes. `context` is essentially a JSON available to the tera files. I also added catchers for `404` and `500` errors, I decided to redirect to the homepage. You could just as easily display a custom page for these, if you prefer. `Template::render("blog", context.into_json())` will look for a file called `blog.html.tera`, which we will create in a moment.

Our main function looks like this 

```
#[rocket::main]
async fn main() {
    let port = std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))
        .unwrap();

    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));

    if let Err(e) = rocket::custom(figment)
        .mount("/", FileServer::from(relative!("assets/")))
        .register("/", catchers![not_found, error])
        .attach(Template::fairing())
        .mount("/", get_all_routes())
        .launch()
        .await
    {
        println!("Did not run. Error: {:?}", e)
    }
}

fn get_all_routes() -> Vec<Route> {
	let routes = vec![index, blog_index, blog_article, tag_page];
	return routes;
}
```

The `get_all_routes` function might seem like overkill, but if you add many more pages then this will become very helpful.

## Tera Templating

Our server is expecting there to be a folder called `templates` next to our `Cargo.toml` file. We can put all our Tera templates in there.

Tera templates are really easy to work with BUT remember that you don't have any kind of type checking nor any guarantee that the element you try to access exists. So, it's important to test your pages frequently, to make sure everything works as you expect.

The tags template I use is 

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
    {% include "utils/nav" %}
    <main>


        {% if blogs %}
        {% for blog in blogs %}
        <p>{{ blog.date }} <a href="/blog/{{blog.slug}}">{{ blog.title }}</a> </p>
        <ul>
            <li>
                {% for tag in blog.tags %}
                <a href="/blog/tag/{{ tag }}">#{{ tag }}</a>
                {% endfor %}
            </li>
            <li>{{ blog.desc | safe}}</li>
        </ul>
        {% endfor %}
        {% else %}
        <p>No blog found</p>

        {% endif %}
    </main>
</body>
```

The idea being we loop through the blogs, and display them. Note that we already filtered all the tags inside the Rocket function, so there's no reason to do that again here.
```
<!DOCTYPE html>
<html lang="en">

<head>
    <meta http-equiv="content-type" content="text/html; charset=UTF-8">
    <title>{{ blog.title }}</title>
    <link rel="stylesheet" type="text/css" href="/css/style.css">
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>

<body>
    {% include "utils/nav" %}
    <main>

    {{ blog.html | safe }}

    </main>

</body>
</html>
```

Each actual blog page is pretty simple - you only need to drop in the actual blog HTML.

The blog index is also quite simple 

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
    {% include "utils/nav" %}
    <main>
        

        {% if blog.entries %}
            {% for blog in blog.entries %}
                <p>{{ blog.date }} <a href="/blog/{{blog.slug}}">{{ blog.title }}</a> </p>
                <ul>
                    <li>
                        {% for tag in blog.tags %}
                            <a href="/blog/tag/{{ tag }}">#{{ tag }}</a>
                        {% endfor %}
                    </li>
                    <li>{{ blog.desc | safe}}</li>
                </ul>
            {% endfor %}
        {% else %}
            <p>No blog found</p>
    
        {% endif %}
    </main>

</body>
```

This just prints out all blogs, with the newest at the top. Of course, you can customise how these appear however you like.

Finally, that leaves the main index page. I'll just use a stripped down version of my own page, to demonstrate the idea

```
<!DOCTYPE html>
<html lang="en">

<head>
    <meta http-equiv="content-type" content="text/html; charset=UTF-8">
    <title>Indigo Curnick's Webpage</title>
    <link rel="stylesheet" type="text/css" href="/css/style.css">
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>

<body>
    <header>
        <h1>Indigo Curnick's Webpage</h1>
    </header>

    {% include "utils/nav" %}

    <main>

        <div class="row">
            <div class="column">
                <h3>Recents</h3>
                <ul style="padding:0">
                    {% if blog.entries %}
                    {% for i in [0,1,2,3] %}
                    <li>{{ blog.entries[i].date }} <a href="/blog/{{blog.entries[i].slug}}">{{ blog.entries[i].title
                            }}</a> </li>
                    {% endfor %}
                    {% else %}
                    <li>No blog found</li>
                    {% endif %}
                    <li>
                        <strong><a href="/blog">See all other blog posts...</a></strong>
                    </li>
                </ul>
            </div>
        </div>


        <h3>Tags</h3>
        <p align="center"></p>
        {% if tags %}
        {% for tag in tags %}
        <a href="/blog/tag/{{ tag }}">#{{ tag }}</a>
        {% endfor %}
        {% endif %}
        </p>


    </main>
</body>

</html>
```

So, I print out the 4 most recent blogs as well as all the unique tags on the homepage. 

Finally, there is the matter of the `{% include "utils/nav" %}` on each page. This is how you just pate the contents of one tera template into another. I use this to have the navigation bar be the same on every page, but you could leave this out if you prefer

## Closing

And... that's it! Making a blog in Rust is really quite easy. There's a lot of design choices to be made regarding speed and memory trade offs. There are also so many different ways you can customise these ideas that I couldn't possibly even give a glimpse of how it could be done. I hope this can serve as a starting point for you!