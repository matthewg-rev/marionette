use dioxus::prelude::*;
use dioxus::desktop::{Config, LogicalSize, DesktopContext, WindowBuilder, use_window};
use dioxus_html_macro::*;

use futures::StreamExt;
use futures_channel::mpsc::{channel, Receiver, Sender};

#[macro_export]
macro_rules! on_result {
    ($msgbox:expr, $value:ident, $code:block) => {
        spawn(async move {
            if let Some($value) = $msgbox.receiver.next().await {
                $code
            }
        });
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MsgType {
    Success,

    Info,
    Warning,
    Error,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MsgButtons {
    Ok,
    OkCancel,
    YesNo,
    YesNoCancel,
    AbortRetryIgnore,
    RetryCancel
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MsgResult {
    None,

    Ok,
    Cancel,
    Yes,
    No,
    Abort,
    Retry,
    Ignore,
}

impl std::fmt::Debug for MsgResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MsgResult::None => write!(f, "None"),
            MsgResult::Ok => write!(f, "Ok"),
            MsgResult::Cancel => write!(f, "Cancel"),
            MsgResult::Yes => write!(f, "Yes"),
            MsgResult::No => write!(f, "No"),
            MsgResult::Abort => write!(f, "Abort"),
            MsgResult::Retry => write!(f, "Retry"),
            MsgResult::Ignore => write!(f, "Ignore"),
        }
    }
}

pub struct Msg {
    pub msg: String,
    pub msg_title: String,
    pub msg_type: MsgType,
    pub msg_buttons: MsgButtons,

    pub sender: Sender<MsgResult>,
    pub receiver: Receiver<MsgResult>,
}

pub fn msg_icon(t: MsgType) -> String {
    match t {
        MsgType::Success => "",
        MsgType::Info => "",
        MsgType::Warning => "",
        MsgType::Error => "",
    }.to_string()
}

pub fn icon_color(t: MsgType) -> String {
    match t {
        MsgType::Success => "#00d26a",
        MsgType::Info => "#4977ff",
        MsgType::Warning => "#ffa500",
        MsgType::Error => "#ed1c24",
    }.to_string()
}

#[derive(Clone, PartialEq, Props)]
struct AppProps {
    icon: MsgType,
    title: String,
    msg: String,
    buttons: MsgButtons,
}

impl Msg {
    pub fn new(msg: String, msg_title: String, msg_type: MsgType, msg_buttons: MsgButtons) -> Self {
        let (tx, rx) = channel::<MsgResult>(1);
        Self {
            msg,
            msg_title,
            msg_type,
            msg_buttons,
            sender: tx,
            receiver: rx,
        }
    }

    pub fn get_msg_height(&self) -> f32 {
        // TODO: JavaScript Rust Interop using bytestream impl
        200.0
    }

    pub fn get_msg_width(&self) -> f32 {
        // TODO: JavaScript Rust Interop using bytestream impl
        200.0
    }

    pub fn new_dom(&self) -> VirtualDom {
        let props = AppProps {
            icon: self.msg_type.clone(),
            title: self.msg_title.clone(),
            msg: self.msg.clone(),
            buttons: self.msg_buttons.clone(),
        };

        let senderClone = self.sender.clone();
        VirtualDom::new_with_props(move |props: AppProps| {
            let senderSignal = use_context_provider(|| Signal::new(senderClone.clone()));

            rsx!(
                style { {include_str!("resources/styles/msgbox/msgbox.css")} }
                {html!(
                    <div class="msgbox_icon_title">
                        <span class="msgbox_icon" style="color: {icon_color(props.icon.clone())}">{msg_icon(props.icon.clone())}</span>
                        <span class="msgbox_title">{props.title.clone()}</span>
                    </div>
                    <span class="msgbox_message">{props.msg.clone()}</span>
                    <div class="button_row">
                    {
                        match props.buttons.clone() {
                            MsgButtons::Ok => html!(
                                <div class="msgbox_buttons">
                                    <div class="msgbox_button" onclick={move |_| {
                                        let window = use_window();
                                        let mut sender = use_context::<Signal<Sender<MsgResult>>>();
                                        let _ = sender.write().to_owned().try_send(MsgResult::Ok);
                                        window.close();
                                    }}>"Ok"</div>
                                </div>
                            ),
                            MsgButtons::OkCancel => {
                                html!(
                                    <div class="msgbox_button" onclick={move |_| {
                                        let window = use_window();
                                        let mut sender = use_context::<Signal<Sender<MsgResult>>>();
                                        let _ = sender.write().to_owned().try_send(MsgResult::Ok);
                                        window.close();
                                    }}>"Ok"</div>
                                    <div class="msgbox_button" onclick={move |_| {
                                        let window = use_window();
                                        let mut sender = use_context::<Signal<Sender<MsgResult>>>();
                                        let _ = sender.write().to_owned().try_send(MsgResult::Cancel);
                                        window.close();
                                    }}>"Cancel"</div>
                                )
                            },
                            MsgButtons::YesNo => {
                                html!(
                                    <div class="msgbox_button" onclick={move |_| {
                                        let window = use_window();
                                        let mut sender = use_context::<Signal<Sender<MsgResult>>>();
                                        let _ = sender.write().to_owned().try_send(MsgResult::Yes);
                                        window.close();
                                    }}>"Yes"</div>
                                    <div class="msgbox_button" onclick={move |_| {
                                        let window = use_window();
                                        let mut sender = use_context::<Signal<Sender<MsgResult>>>();
                                        let _ = sender.write().to_owned().try_send(MsgResult::No);
                                        window.close();
                                    }}>"No"</div>
                                )
                            },
                            _ => html!()
                        }
                    }
                </div>
            )})
        }, props)
    }

    pub fn display(&self, desktop: &DesktopContext) {
        let (msgbox_width, msgbox_height) = (self.get_msg_width(), self.get_msg_height());

        let cfg = Config::default().with_window(
            WindowBuilder::new()
                .with_title(self.msg_title.as_str())
                .with_resizable(false)
                .with_min_inner_size(LogicalSize::new(msgbox_width, msgbox_height))
                .with_max_inner_size(LogicalSize::new(msgbox_width, msgbox_height))
                .with_always_on_top(true)
        ).with_menu(None);

        desktop.new_window(self.new_dom(), cfg);
    }
}