// The State trait defines the behavior shared by different post states
trait State {
    // 1. As we will learn later, `Box<T>` is a smart pointer. Smart pointers
    // are like normal pointers with some extra functionality.
    // `self: Box<Self>`: This syntax means theat the method is valid
    // only when called on a Box holding the type. This syntax takes
    // ownership of Box<Self>, invalidating the old state so the
    //  state value of the Post can transform into a new state.
    // 2. Return type is `Box<dyn State>`. When your function returns a
    // pointer-to-trait-on-heap in this way, you need to write the
    // return type with the dyn keyword
    // (https://doc.rust-lang.org/rust-by-example/trait/dyn.html)

    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content <'a> (&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published{
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content <'a> (&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// Public interface => Post
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // call the as_ref method on the Option to get a
    // reference to the value inside the Option rather 
    // than ownership of the value.
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    // `take`: take the Some value out of the state field and
    // leave a None in its place

    // We need to set state to None temporarily rather than
    // setting it directly with code like
    // self.state = self.state.request_review(); to get
    // ownership of the state value. This ensures Post can’t
    // use the old state value after we’ve transformed it into
    // a new state.
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}