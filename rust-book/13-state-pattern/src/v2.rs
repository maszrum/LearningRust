pub fn do_v2() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

struct Post;

impl Post {
    pub fn new() -> Draft {
        Draft { content: String::new() }
    }
}

struct Draft {
    content: String
}

impl Draft {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReview {
        PendingReview {
            content: self.content
        }
    }
}

struct PendingReview {
    content: String
}

impl PendingReview {
    pub fn approve(self) -> Published {
        Published {
            content: self.content
        }
    }
}

struct Published {
    content: String
}

impl Published {
    pub fn content(&self) -> &String {
        &self.content
    }
}
