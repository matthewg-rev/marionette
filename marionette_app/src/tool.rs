use dioxus::{html::s, prelude::*};
use dioxus_html_macro::html;
use dioxus_desktop::{use_window};
use crate::states::selector::SelectorState;

#[component]
pub fn Tool(cx: Scope) -> Element {
    let window = use_window(cx);
    window.set_resizable(true);

    let selector_state = use_shared_state::<SelectorState>(cx).unwrap();
    let selected = selector_state.read().selected_path.clone();

    cx.render(rsx!(
        style { include_str!("resources/styles/tool/tool.css") },
        style { include_str!("resources/styles/tool/widget.css") },
        
        style { include_str!("resources/styles/tool/widgets/graph.css") },
        style { include_str!("resources/styles/tool/widgets/text_editor.css") },
        style { include_str!("resources/styles/tool/widgets/clock.css") },

        script { include_str!("resources/scripts/tool/jquery-3.7.1.min.js")},
        script { include_str!("resources/scripts/tool/dagre.min.js") },

        script { include_str!("resources/scripts/tool/graphing/data_provider.js") }
        script { include_str!("resources/scripts/tool/graphing/graph.js") },
        script { include_str!("resources/scripts/tool/graphing/graph_vertex.js") },
        script { include_str!("resources/scripts/tool/graphing/graph_edge.js")}

        script { include_str!("resources/scripts/tool/rendering/line.js") },
        script { include_str!("resources/scripts/tool/rendering/text.js") },
        script { include_str!("resources/scripts/tool/rendering/renderer.js") },
        script { include_str!("resources/scripts/tool/rendering/box_renderer.js") },

        script { include_str!("resources/scripts/tool/widget.js") },
        script { include_str!("resources/scripts/tool/widgets/graph.js") },
        script { include_str!("resources/scripts/tool/widgets/clock.js") },
        script { include_str!("resources/scripts/tool/widgets/text_editor.js") },

        script { include_str!("resources/scripts/tool/canvas.js") },
        script { include_str!("resources/scripts/tool/toolbar.js") },
        script { include_str!("resources/scripts/tool/tool.js") },

        html!(
            <div id="toolbar"></div>
            <div class="canvas">
            </div>
        )
    ))
}