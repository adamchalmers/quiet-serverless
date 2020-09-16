use handlebars::Handlebars;
use lazy_static::lazy_static;

pub enum TemplateName {
    Base,
    Home,
    Error,
    NewPost,
    PostList,
}

impl TemplateName {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Home => "home",
            Self::Error => "error",
            Self::NewPost => "new_post",
            Self::PostList => "post_list",
        }
    }
}
lazy_static! {
    pub static ref HBARS: Handlebars<'static> = {
        // Register templates
        let mut hb = Handlebars::new();
        hb.register_template_string(&TemplateName::Base.name(), include_str!("templates/base.html")).unwrap();
        hb.register_template_string(&TemplateName::Home.name(), include_str!("templates/home.html")).unwrap();
        hb.register_template_string(&TemplateName::Error.name(), include_str!("templates/error.html")).unwrap();
        hb.register_template_string(&TemplateName::NewPost.name(), include_str!("templates/new_post.html")).unwrap();
        hb.register_template_string(&TemplateName::PostList.name(), include_str!("templates/post_list.html")).unwrap();
        hb
    };
}
