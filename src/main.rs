pub mod ramdisk;

use iced::widget::{Button, Column, Container, PickList, container, text};
use iced::{Alignment, Length::Fill, Size, application};

use crate::ramdisk::{MountInfo, RamdiskMount, get_tmpfs_mounts};

#[allow(unused)]
struct AppState {
    ramdisk_mounts: Vec<RamdiskMount>,
    selected_ramdisk: Option<RamdiskMount>,
    selected_ramdisk_stats: Option<MountInfo>,
}

#[derive(Debug, Clone)]
#[allow(unused)]
enum Message {
    DeviceSelected(&'static str), // TODO: Change `&'static str` to `RamdiskMount`
    RemoveSelectedDrive,
}

impl AppState {
    #[allow(unused)]
    fn new() -> Self {
        Self {
            ramdisk_mounts: get_tmpfs_mounts().unwrap(),
            selected_ramdisk: None,
            selected_ramdisk_stats: None,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DeviceSelected(mount) => println!("{}", mount),
            _ => println!("TODO"),
        }
    }

    fn view(&self) -> Container<'_, Message> {
        let mount_list = ["/home/foolan/iced-docs", "/home/foolan/compilation-folder"];
        // TODO: Change `&'static str` to `RamdiskMount`
        let mounts_picklist = PickList::new(mount_list, Some(mount_list[0]), |device| {
            Message::DeviceSelected(device)
        });
        let mut column = Column::with_children([mounts_picklist.into()]).spacing(15);

        match self.selected_ramdisk.as_ref() {
            Some(mount) => {
                column = column.push(
                    text(format!(
                        "Disk details for ramdisk device {}",
                        mount.mount_point()
                    ))
                    .width(Fill)
                    .align_x(Alignment::Center),
                );
                column = column.push(
                    text(format!(
                        "Free space: {}",
                        mount.get_stats().unwrap().total_space()
                    ))
                    .width(Fill)
                    .align_x(Alignment::Center),
                );
                column = column.push(
                    text(format!("Free space: {}", 10))
                        .width(Fill)
                        .align_x(Alignment::Center),
                );
                column = column.push(
                    Button::new("Remove ramdisk device")
                        .width(Fill)
                        .on_press(Message::RemoveSelectedDrive),
                );
            }
            None => {}
        }
        container(column).padding(15).align_x(Alignment::Center)
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState::new()
    }
}

fn main() -> iced::Result {
    application(AppState::default, AppState::update, AppState::view)
        .window_size(Size {
            width: 250_f32,
            height: 250_f32,
        })
        .title("TMPFS GUI")
        .run()
}
