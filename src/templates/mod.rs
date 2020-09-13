use handlebars::Handlebars;
use std::fmt;
use lazy_static::lazy_static;

impl fmt::Display for TemplateName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
pub enum TemplateName {
    Base,
    Home,
    Error,
}

impl TemplateName {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Home => "home",
            Self::Error => "error",
        }
    }
}
lazy_static! {
    pub static ref HBARS: Handlebars<'static> = {
        // Register templates
        let mut hb = Handlebars::new();
        hb.register_template_string(&TemplateName::Base.to_string(), include_str!("base.html")).unwrap();
        hb.register_template_string(&TemplateName::Home.to_string(), include_str!("home.html")).unwrap();
        hb.register_template_string(&TemplateName::Error.to_string(), include_str!("error.html")).unwrap();
        hb
    };
}
