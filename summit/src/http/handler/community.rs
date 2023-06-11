use sailfish::TemplateOnce;
use tracing::info;

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
    info!("community");

    let ctx = Template {
        title: "Some Title".into(),
        posts: vec![],
    };

    ctx.render_once().unwrap()
}
