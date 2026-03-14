use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Center, Element, Length, Task, window};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::process::Command;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    RunSysCommand,
    RunSysCommand2,
    CancelShutdown,
    InputChanged(String),
    OpenAbout,
    WindowOpened(window::Id),
}

struct App {
    main_window_id: Option<window::Id>,
    duration_input: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            main_window_id: None,
            duration_input: String::from("40"),
        }
    }
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let (id, task) = window::open(window::Settings {
            size: iced::Size::new(600.0, 400.0),
            ..Default::default()
        });
        let app = Self {
            main_window_id: Some(id),
            ..Default::default()
        };
        (app, task.map(Message::WindowOpened))
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RunSysCommand => {
                let mut cmd = Command::new("shutdown");
                cmd.args(["/s", "/t", "0"]);
                #[cfg(windows)]
                cmd.creation_flags(CREATE_NO_WINDOW);
                let _ = cmd.spawn();
            }

            Message::InputChanged(value) => {
                self.duration_input = value.chars().filter(|c| c.is_digit(10)).collect();
            }

            Message::RunSysCommand2 => {
                if let Ok(minutes) = self.duration_input.parse::<u32>() {
                    let seconds = minutes * 60;
                    let mut cmd = Command::new("shutdown");
                    cmd.args(["/s", "/t", &seconds.to_string()]);
                    #[cfg(windows)]
                    cmd.creation_flags(CREATE_NO_WINDOW);
                    let _ = cmd.spawn();
                }
            }

            Message::CancelShutdown => {
                let mut cmd = Command::new("shutdown");
                cmd.arg("/a");
                #[cfg(windows)]
                cmd.creation_flags(CREATE_NO_WINDOW);
                let _ = cmd.spawn();
            }

            Message::OpenAbout => {
                let (_, task) = window::open(window::Settings {
                    size: iced::Size::new(500.0, 350.0),
                    resizable: false,
                    ..Default::default()
                });
                return task.discard();
            }

            Message::WindowOpened(id) => {
                if self.main_window_id.is_none() {
                    self.main_window_id = Some(id);
                }
            }
        }
        Task::none()
    }

    fn view(&'_ self, id: window::Id) -> Element<'_, Message> {
        if self.is_about_window(id) {
            return self.about_view();
        }

        let input_field = text_input("分钟...", &self.duration_input)
            .on_input(Message::InputChanged)
            .width(Length::Fixed(80.0))
            .padding(10);

        let controls = row![
            button(text("立即关机"))
                .on_press(Message::RunSysCommand)
                .padding(10),
            button(text(format!("延时 {} 分钟", self.duration_input)))
                .on_press(Message::RunSysCommand2)
                .padding(10),
            button(text("取消任务"))
                .on_press(Message::CancelShutdown)
                .padding(10),
            button(text("关于"))
                .on_press(Message::OpenAbout)
                .padding(10),
        ]
        .spacing(15)
        .align_y(Alignment::Center);

        container(
            column![
                text("简易系统计划任务管理器").size(32),
                row![text("设定时长:"), input_field, text("分钟")]
                    .spacing(10)
                    .align_y(Alignment::Center),
                controls,
            ]
            .spacing(30)
            .align_x(Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }

    fn is_about_window(&self, id: window::Id) -> bool {
        self.main_window_id != Some(id)
    }

    fn about_view(&self) -> Element<'_, Message> {
        container(
            column![
                text("关于本程序").size(22),
                text("version1.0rc1"),
                text("作者是超可爱的喵"),
                text("本程序是自由软件，不提供任何担保，以GNU GPLv3.0开放源代码"),
                text("真正做实事的人是难以令人理解的。"),
                text("没有素质的人不把人当人。"),
                text("只想着写代码的人是孤独的。"),
                text("有些神人难以理解祂们做错了什么，到底伤害到了谁。"),
                text("驶离 Windows 舒适区吧，下一站：GNU/Linux (LoongArch Edition)"),
                text("Powered by iced, a cross-platform GUI toolkit").size(12)
            ]
            .spacing(12)
            .align_x(Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(30)
        .into()
    }
} 
