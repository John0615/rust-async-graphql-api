
use async_graphql::{Object};

struct Book {
    id: i32,
    title: String,
    author: String,
}

#[Object]
impl Book {
    async fn id(&self) -> i32 {
        self.id
    }
    async fn title(&self) -> &str {
        &self.title
    }

    async fn author(&self) -> &str {
        &self.author
    }
}

#[Object]
impl User {
    async fn id(&self) -> i32 {
        self.id
    }
    async fn username(&self) -> &str {
        &self.username
    }
    async fn phone(&self) -> i64 {
        self.phone
    }
}

struct User {
    id: i32,
    username: String,
    phone: i64,
}



pub struct Mutation;

#[Object]
impl Mutation {
    async fn signup(&self, _username: String, _password: String) -> Vec<Book> {
        vec![
            Book {
                id: 1,
                title: "踢啊四四四口大口大口大口".to_string(),
                author: "John".to_string(),
            },
            Book {
                id: 2,
                title: "上课送我我哦饿哦饿".to_string(),
                author: "Jobin".to_string(),
            },
            Book {
                id: 3,
                title: "i为i为i开".to_string(),
                author: "Edward".to_string(),
            },
        ]
    }

    async fn login(&self, username: String, _password: String) -> Vec<User> {
        vec![
            User {
                id: 1,
                username: "John".to_string(),
                phone: 17722443741,
            },
            User {
                id: 2,
                username: "Jobin".to_string(),
                phone: 13366778877,
            },
            User {
                id: 3,
                username: "Edward".to_string(),
                phone: 13567678798,
            },
            User {
                id: 3,
                username: username.to_string(),
                phone: 13567678798,
            },
        ]
    }
}