use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct PannableSpaceProps {

}

pub fn ComponentPannableSpace(props: PannableSpaceProps) -> Element {
    rsx! {
        div {
            "This is the pannable space"
        }
    }
}

