 
# Object Orientation is a Religion

I'm currently working on a lengthy article on reason, which is coming along slowly. In the meantime, I thought I'd make a short video on a quip I often say - object orientation is a religion. Now, this might seem a little abstract so I wanted to break it down for a while. Thankfully, the perfect opportunity presented itself in the form of a one minute video on Instagram - which you can view here [https://www.instagram.com/reel/C1VFRAOtvDX](https://www.instagram.com/reel/C1VFRAOtvDX)

If you don't want to go to Instagram (or it gets deleted in future) that's fine, I'll copy all the relevant code out below. 

We start with a simple `class` which represents a book 

```
public class Book {
	
	private String title;
	private String author;
	private String text;
	
	public Book(String title, String author, String text) {
		this.title = title;
		this.author = author;
		this.text = text;
	}
	
	public String getTitle() { return title; }
	public String getAuthor() { return author; }
	public void printBook() {
		// Some code to print a book
	}
	public void saveToFile() {
		// Some code to save to file
	}
}
```

We're told that the problem with this class is that it has "more than one responsibility" - managing the book's properties, printing the book, and saving it to file. Therefore, we need to refactor this into different classes. The "solution" to this "problem" is to split the `Book` class into three smaller classes - one for managing the book's properties, one for saving a book and one for printing a book 

```
public class Book {
	
	private String title;
	private String author;
	private String text;
	
	public Book(String title, String author, String text) {
		this.title = title;
		this.author = author;
		this.text = text;
	}
	
	public String getTitle() { return title; }
	public String getAuthor() { return author; }	
}

public class BookPrinter {
	public void printBook(Book book) {
		// Some code to print a book
	}
}

public class BookSaver {
	public void saveToFile(Book book) {
		// Some code to save to file
	}
}
```

And apparently this is better because SOLID principles, or something. Let's do this in C just to see what we could do different

```
struct Book
{
    char title[50];
    char author[50];
    char text[1000];
};

struct Book new_book(char title[], char author[], char text[])
{
    struct Book book = {title, author, text};

    return book;
}

void print_book(struct Book book) {
    // Some code here
}

void save_book(struct Book book) {
    // Some code here
}
```

So now we have turned the book class into a simple struct. What is the consequences of this? Each of the previous methods just become functions. What is the consequence of _that_? We don't even need to _entertain ideas of which methods belong to which class_. This program is now significantly more extensible than the Java program originally presented. Suppose now the needs of the program change, and we need to load a book from disc. In C there is only one thing to do 

```
struct Book load_book() {
	// Some code here to load a book
}
```

But in an object orientated fashion, what's the solution here? Maybe we need a new `BookLoader` class?

```
class BookLoader {
	public Book loadBook() {
		// Some code here
	}
}
```

Or maybe loading and saving a book is close enough together, since they both interact with the file system? What about a `FileBook` class?

```
class FileBook {
	public void saveToFile(Book book) {
		// Some code to save to file
	}
	
	public Book loadBook() {
		// Some code here
	}
}
```

Or maybe these could just be a part of the original `Book` class...

The point here is becoming clear, I hope. Object orientation is a religion. You are not permitted to actually program without appeal to some arbitrary set of principles. You must not do what coders do (solve problems) but appease some kind of malevolent architectural God through the sacrifice of more classes. Endless classes. Notice though, that the entire discussion of this blog post _has nothing to do with saving and printing books_. Nothing valuable was discussed in this blog post.

Object orientation forces upon you a discussion of the organisation of code which is totally irrelevant to the problem itself. It does this a matter of faith. You are an acolyte of the most organised religion ever devised.

Just use a function.
