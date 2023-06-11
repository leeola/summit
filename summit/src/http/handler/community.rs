use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "community.stpl")]
pub struct Template {
    pub title: String,
    pub posts: Vec<Post>,
}
pub struct Post {
    pub title: String,
}

pub async fn handler() -> String {
    let ctx = Template {
        title: "Some Title".into(),
        posts: vec![],
    };

    ctx.render_once().unwrap()
}
