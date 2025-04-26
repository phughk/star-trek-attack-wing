use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct PannableSpaceProps {
    #[props(default = 36.0)]
    width: f64,
    #[props(default = 36.0)]
    height: f64,
}

#[component]
pub fn ComponentPannableSpace(props: PannableSpaceProps) -> Element {
    rsx! {
        div {
            "This is the pannable space with width {props.width} and height {props.height}."
        }
    }
}

