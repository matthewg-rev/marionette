use dioxus::{prelude::*};
use dioxus::desktop::{use_window};
use pyo3::class;
use std::rc::Rc;
use crate::states::selector::SelectorState;

#[derive(Clone, PartialEq, Props)]
pub struct ToolCanvasProps {

}

pub fn ToolCanvas(props: ToolCanvasProps) -> Element {
    rsx! {
        div {
            id: "tool-canvas",
            class: "canvas"
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ToolbarProps {
    pub children: Element,
}

pub fn Toolbar(props: ToolbarProps) -> Element {
    let mut element_info = use_signal(Vec::<(&str, Rc<MountedData>)>::new);

    rsx! {
        div {
            id: "toolbar",
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ToolbarCategoryProps {
    #[props(default=rsx!{})]
    pub expanded: Element,
    pub name: String
}

pub fn ToolbarCategory(props: ToolbarCategoryProps) -> Element {
    let mut expanded = use_signal(|| false);
    let mut class_list = use_signal(|| "toolbar-category");

    rsx! {
        div {
            class: class_list(),
            onclick: move |event: MouseEvent| {
                expanded.toggle();
                if expanded() {
                    class_list.set("toolbar-category toolbar-category-selected");
                } else {
                    class_list.set("toolbar-category");
                }

                event.stop_propagation();
            },
            
            if expanded() {
                div {
                    class: "toolbar-category-dropdown",
                    {props.expanded}
                }
            }
            "{props.name}"
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ToolbarToolProps {
    pub name: String,
    #[props(default=Callback::new(|_| {}))]
    pub onclick: Callback<MouseEvent>,
}

pub fn ToolbarTool(props: ToolbarToolProps) -> Element {
    rsx! {
        div {
            class: "toolbar-tool",
            onclick: props.onclick,
            "{props.name}"
        }
    }
}

#[component]
pub fn Tool() -> Element {
    let window = use_window();
    window.set_resizable(true);

    let selector_state = use_context::<Signal<SelectorState>>();
    let selected = selector_state.read().selected_path.clone();

    rsx! {
        style { {include_str!("resources/styles/tool/tool.css")} }
        style { {include_str!("resources/styles/tool/widget.css")} }
        
        style { {include_str!("resources/styles/tool/widgets/graph.css")} }
        style { {include_str!("resources/styles/tool/widgets/text_editor.css")} }
        style { {include_str!("resources/styles/tool/widgets/clock.css")} }
        style { {include_str!("resources/styles/tool/widgets/log.css")} }

        script { {include_str!("resources/scripts/tool/jquery-3.7.1.min.js")} }
        script { {include_str!("resources/scripts/tool/dagre.min.js")} }

        script { {include_str!("resources/scripts/tool/graphing/data_provider.js")} }
        script { {include_str!("resources/scripts/tool/graphing/graph.js")} }
        script { {include_str!("resources/scripts/tool/graphing/graph_vertex.js")} }
        script { {include_str!("resources/scripts/tool/graphing/graph_edge.js")} }

        script { {include_str!("resources/scripts/tool/rendering/line.js")} }
        script { {include_str!("resources/scripts/tool/rendering/text.js")} }
        script { {include_str!("resources/scripts/tool/rendering/renderer.js")} }
        script { {include_str!("resources/scripts/tool/rendering/box_renderer.js")} }

        script { {include_str!("resources/scripts/tool/widget.js")} }
        script { {include_str!("resources/scripts/tool/widgets/graph.js")} }
        script { {include_str!("resources/scripts/tool/widgets/clock.js")} }
        script { {include_str!("resources/scripts/tool/widgets/text_editor.js")} }
        script { {include_str!("resources/scripts/tool/widgets/log.js")} }

        script { {include_str!("resources/scripts/tool/canvas.js")} }
        script { {include_str!("resources/scripts/tool/toolbar.js")} }
        script { {include_str!("resources/scripts/tool/tool.js")} }

        Toolbar {
            ToolbarCategory {
                name: "File",
                expanded: rsx! {
                    ToolbarTool {
                        name: "Exit",
                        onclick: |_| {
                            let window = use_window();
                            window.close();
                        }
                    }
                }
            }

            // TODO: Interact with Canvas Widget system
            ToolbarCategory {
                name: "Widgets",

                expanded: rsx! { 
                    ToolbarCategory {
                        name: "Analysis",

                        expanded: rsx! {
                            ToolbarTool {
                                name: "Graph View"
                            }
                            ToolbarTool {
                                name: "Text View"
                            }
                            ToolbarTool {
                                name: "Log View"
                            }
                        }
                    }
                    ToolbarCategory {
                        name: "Miscellaneous",

                        expanded: rsx! {
                            ToolbarTool {
                                name: "Clock"
                            }
                        }
                    }
                }
            }
        }

        ToolCanvas {}
    }
}