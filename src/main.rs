pub mod ramdisk;

use iced::widget::{Button, Column, Container, container, pick_list, text};
use iced::{Alignment, Length::Fill, Size, application};

use crate::ramdisk::{MountInfo, ramdisk_mount::RamdiskMount};

#[allow(unused)]
struct AppState {
    ramdisk_mounts: Vec<RamdiskMount>,
    selected_ramdisk: Option<RamdiskMount>,
    selected_ramdisk_stats: Option<MountInfo>,
}

#[derive(Debug, Clone)]
#[allow(unused)]
enum Message {
    DeviceSelected(RamdiskMount),
    RemoveSelectedDrive,
}

impl AppState {
    #[allow(unused)]
    fn new() -> Self {
        Self {
            ramdisk_mounts: RamdiskMount::from_existing(true).unwrap(),
            selected_ramdisk: None,
            selected_ramdisk_stats: None,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DeviceSelected(mount) => {
                self.selected_ramdisk = Some(mount);
            }
            _ => println!("TODO"),
        }
    }

    fn view(&self) -> Container<'_, Message> {
        let mount_list = self.ramdisk_mounts.clone();
        let mounts_picklist = pick_list(mount_list, self.selected_ramdisk.clone(), |device| {
            Message::DeviceSelected(device)
        })
        .width(Fill);
        let mut main_column = Column::new().spacing(15);
        let mut secondary_column: Column<'_, Message> = Column::new().spacing(15);

        if !self.ramdisk_mounts.is_empty() {
            main_column = main_column.push(mounts_picklist)
        }

        match self.selected_ramdisk.as_ref() {
            Some(mount) => {
                let mount_stats = mount.get_stats().unwrap();
                main_column = main_column.push(
                    text(format!(
                        "Disk details for ramdisk device {}",
                        mount.mount_point()
                    ))
                    .width(Fill)
                    .align_x(Alignment::Center),
                );
                main_column = main_column.push(
                    text(format!(
                        "Total space: {}",
                        mount_stats.total_space().to_mebibytes()
                    ))
                    .width(Fill)
                    .align_x(Alignment::Center),
                );
                main_column = main_column.push(
                    text(format!(
                        "Free space: {}",
                        mount_stats.free_space().to_mebibytes()
                    ))
                    .width(Fill)
                    .align_x(Alignment::Center),
                );
                main_column = main_column.push(
                    Button::new("Remove ramdisk device")
                        .width(Fill)
                        .on_press(Message::RemoveSelectedDrive),
                );
            }
            None => {}
        }

        secondary_column = secondary_column.push(
            text("It looks like you don't have any ramdisk devices.")
                .width(Fill)
                .align_x(Alignment::Center),
        );
        secondary_column = secondary_column.push(Button::new("Create New Ramdisk").width(Fill));

        let output = if !self.ramdisk_mounts.is_empty() {
            container(main_column)
        } else {
            container(secondary_column)
        };

        output.padding(15).align_x(Alignment::Center)
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
