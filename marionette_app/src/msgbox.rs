use dioxus::prelude::*;
use dioxus_html_macro::html;
use dioxus_desktop::{Config, DesktopContext, LogicalSize, use_window, WindowBuilder};
use futures_channel::mpsc::{channel, Receiver, Sender};

#[macro_export]
macro_rules! on_result {
    ($cx:expr, $msgbox:expr, $value:ident, $code:block) => {
        $cx.spawn(async move {
            if let Some($value) = $msgbox.receiver.next().await {
                $code
            }
        });
    }
}

#[derive(Clone)]
pub enum MsgSize {
    Fit,
    UseMaxCharsPerLine(i32),
}

#[derive(Clone)]
pub enum MsgType {
    Success,

    Info,
    Warning,
    Error,
}

#[derive(Clone)]
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
    pub msg_height: MsgSize,

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

struct AppProps {
    icon: MsgType,
    title: String,
    msg: String,
    buttons: MsgButtons,
    sender: Sender<MsgResult>,
}

impl Msg {
    pub fn new(msg: String, msg_title: String, msg_type: MsgType, msg_buttons: MsgButtons, msg_height: MsgSize) -> Self {
        let (tx, rx) = channel::<MsgResult>(1);
        Self {
            msg,
            msg_title,
            msg_type,
            msg_buttons,
            msg_height,
            sender: tx,
            receiver: rx,
        }
    }

    pub fn get_lines(&self, max_chars_per_line: i32) -> i32 {
        let mut lines = 1;
        for (chars, c) in self.msg.chars().enumerate() {
            if c == '\n' {
                if self.msg.chars().count()-1 == chars {
                    break;
                }
                lines += 1;
            }
        }
        let mut chars = 0;
        for c in self.msg.chars() {
            if c == '\t' {
                chars += 4;
            } else if c == '\n' {
                chars = 0;
            } else if chars >= max_chars_per_line {
                chars = 0;
                lines += 1;
            } else {
                chars += 1;
            }
        }
        lines
    }

    pub fn get_longest_line(&self) -> i32 {
        let mut longest_line = 0;
        let mut chars = 0;
        for c in self.msg.chars() {
            if c == '\n' {
                if chars > longest_line {
                    longest_line = chars;
                }
                chars = 0;
            } else if c == '\t' {
                chars += 4;
            } else {
                chars += 1;
            }
        }
        if chars > longest_line {
            longest_line = chars;
        }
        longest_line
    }

    pub fn get_msg_height(&self) -> f32 {
        let mut msgbox_height = 31.0 + 32.0;
        let scale = match self.msg_height {
            MsgSize::Fit => {
                let max_chars_per_line = self.get_longest_line();
                self.get_lines(max_chars_per_line)
            },
            MsgSize::UseMaxCharsPerLine(max_chars_per_line) => {
                self.get_lines(max_chars_per_line)
            },
        };
        let msg_container_height = 13.33 + (scale as f32 * 21.0);
        msgbox_height += msg_container_height;
        msgbox_height
    }

    pub fn get_msg_width(&self) -> f32 {
        let max_chars_per_line = match self.msg_height {
            MsgSize::Fit => {
                self.get_longest_line()
            },
            MsgSize::UseMaxCharsPerLine(max_chars_per_line_user) => {
                max_chars_per_line_user
            },
        };
        // hey matt, if you are reading this and wondering
        // "why the hell am i adding 50.0 out of nowhere?"
        // its because the msgbox container has 25.0 padding on each side
        // so shut up and deal with it
        (6.0 * max_chars_per_line as f32) + 50.0
    }

    pub fn new_dom(&self) -> VirtualDom {
        VirtualDom::new_with_props(move |cx| {
            let window = use_window(cx);
            cx.render(rsx!(
                    style { include_str!("resources/styles/msgbox/msgbox.css") }
                    html!(
                        <div class="msgbox_icon_title">
                            <span class="msgbox_icon" style="color: {icon_color(cx.props.icon.clone())}">{msg_icon(cx.props.icon.clone())}</span>
                            <span class="msgbox_title">{cx.props.title.clone()}</span>
                        </div>
                        <span class="msgbox_message">{cx.props.msg.clone()}</span>
                        <div class="button_row">
                        {
                            match cx.props.buttons.clone() {
                                MsgButtons::Ok => html!(
                                    <div class="msgbox_buttons">
                                        <div class="msgbox_button" onclick={move |_| {
                                            let _ = cx.props.sender.to_owned().try_send(MsgResult::Ok);
                                            window.close();
                                        }}>"Ok"</div>
                                    </div>
                                ),
                                MsgButtons::OkCancel => html!(
                                    <div class="msgbox_button" onclick={move |_| {
                                        let _ = cx.props.sender.to_owned().try_send(MsgResult::Ok);
                                        window.close();
                                    }}>"Ok"</div>
                                    <div class="msgbox_button" onclick={move |_| {
                                        let _ = cx.props.sender.to_owned().try_send(MsgResult::Cancel);
                                        window.close();
                                    }}>"Cancel"</div>
                                ),
                                MsgButtons::YesNo => html!(
                                    <div class="msgbox_button" onclick={move |_| {
                                        let _ = cx.props.sender.to_owned().try_send(MsgResult::Yes);
                                        window.close();
                                    }}>"Yes"</div>
                                    <div class="msgbox_button" onclick={move |_| {
                                        let _ = cx.props.sender.to_owned().try_send(MsgResult::No);
                                        window.close();
                                    }}>"No"</div>
                                ),
                                _ => html!()
                            }
                        }
                    </div>
                )))
        }, AppProps {
            icon: self.msg_type.clone(),
            title: self.msg_title.clone(),
            msg: self.msg.clone(),
            buttons: self.msg_buttons.clone(),
            sender: self.sender.clone()
        })
    }

    pub fn display(&self, desktop: &DesktopContext) {
        let (msgbox_width, msgbox_height) = (self.get_msg_width(), self.get_msg_height());

        let cfg = Config::default().with_window(
            WindowBuilder::new()
                .with_title(self.msg_title.as_str())
                .with_resizable(false)
                .with_min_inner_size(LogicalSize::new(msgbox_width, msgbox_height))
                .with_max_inner_size(LogicalSize::new(msgbox_width, msgbox_height))
        );

        desktop.new_window(self.new_dom(), cfg);
    }
}