pub use hypertext::{
    html_elements, rsx, rsx_move, rsx_static, Attribute, GlobalAttributes, Raw, RenderIterator,
    Renderable, VoidElement,
};

pub trait HtmxAttributes: GlobalAttributes {
    #![allow(non_upper_case_globals)]
    const hx_get: Attribute = Attribute;
    const hx_post: Attribute = Attribute;
}

impl<T: GlobalAttributes> HtmxAttributes for T {}
