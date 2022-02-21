use blog_post::Post;

fn main() {
    let mut post = Post::new();
    let content: &str = "I ate a salad for launch today";

    post.add_text(content);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(content, post.content());

}
