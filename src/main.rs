pub mod ramdisk;

use iced::widget::{Button, Column, Container, Row, container, pick_list, text};
use iced::{Alignment, Length::Fill, Size, application};

use crate::ramdisk::{MountInfo, ramdisk_mount::RamdiskMount};

#[allow(unused)]
struct AppState {
    ramdisk_mounts: Vec<RamdiskMount>,
    selected_ramdisk: Option<RamdiskMount>,
    selected_ramdisk_stats: Option<MountInfo>,
    current_page: AppPage,
    status_text: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(unused)]
enum Message {
    DeviceSelected(RamdiskMount),
    RemoveSelectedDrive,
    DeviceListUpdated,
    InfoPageDismissed,
}

#[allow(unused)]
#[derive(PartialEq)]
enum AppPage {
    Info,
    NoRamdiskDevices,
    SelectRamdiskDevice,
    NewRamdiskDevice,
}

// TODO: Make functions that return widgets with the common settings
impl AppState {
    #[allow(unused)]
    fn new() -> Self {
        let ramdisk_mounts = RamdiskMount::from_existing(true).unwrap();
        let is_ramdisk_mounts_empty = ramdisk_mounts.is_empty();
        Self {
            ramdisk_mounts,
            selected_ramdisk: None,
            selected_ramdisk_stats: None,
            current_page: {
                if is_ramdisk_mounts_empty {
                    AppPage::NoRamdiskDevices
                } else {
                    AppPage::SelectRamdiskDevice
                }
            },
            status_text: None,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DeviceSelected(mount) => {
                self.selected_ramdisk = Some(mount);
                self.update(Message::DeviceListUpdated);
            }
            Message::DeviceListUpdated => {
                self.ramdisk_mounts = RamdiskMount::from_existing(true).unwrap();
                if self.current_page != AppPage::Info {
                    self.current_page = self.appropriate_page()
                };
            }
            Message::RemoveSelectedDrive => {
                self.current_page = AppPage::Info;
                self.status_text = match self.selected_ramdisk.as_ref().unwrap().remove() {
                    Ok(()) => {
                        self.selected_ramdisk = None;
                        self.update(Message::DeviceListUpdated);
                        Some("Successfully unmounted this ramdisk device".to_string())
                    }
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::PermissionDenied => Some(
                            "Error: Failed to unmount due to insufficient permissions\nDid you run this program as root?".to_string(),
                        ),
                        _ => Some(format!("Error: {}", e.kind())),
                    },
                };
            }
            Message::InfoPageDismissed => {
                self.current_page = self.appropriate_page();
                self.update(Message::DeviceListUpdated);
            }
        }
    }

    fn view(&self) -> Container<'_, Message> {
        let spacing = 15.0;

        // SelectRamdiskDevice page
        let mut sel_ramdisk_col = Column::new().spacing(spacing);

        let mut picklist_row: Row<'_, Message> = Row::new().spacing(5);
        let mount_list = self.ramdisk_mounts.clone();
        let mounts_picklist = pick_list(mount_list, self.selected_ramdisk.clone(), |device| {
            Message::DeviceSelected(device)
        })
        .width(Fill);
        let refresh_button: Button<'_, Message> = Button::new("🗘")
            .width(35)
            .on_press(Message::DeviceListUpdated);

        picklist_row = picklist_row.push(mounts_picklist);
        picklist_row = picklist_row.push(refresh_button);
        sel_ramdisk_col = sel_ramdisk_col.push(picklist_row);

        if let Some(mount) = self.selected_ramdisk.as_ref() {
            let mount_stats = mount.get_stats().unwrap();

            sel_ramdisk_col = sel_ramdisk_col.push(
                text(format!(
                    "Disk details for ramdisk device {}",
                    mount.mount_point()
                ))
                .width(Fill)
                .align_x(Alignment::Center),
            );

            sel_ramdisk_col = sel_ramdisk_col.push(
                text(format!(
                    "Total space: {}",
                    mount_stats.total_space().auto_convert()
                ))
                .width(Fill)
                .align_x(Alignment::Center),
            );

            sel_ramdisk_col = sel_ramdisk_col.push(
                text(format!(
                    "Free space: {}",
                    mount_stats.free_space().auto_convert()
                ))
                .width(Fill)
                .align_x(Alignment::Center),
            );

            sel_ramdisk_col = sel_ramdisk_col.push(
                Button::new("Remove ramdisk device")
                    .width(Fill)
                    .on_press(Message::RemoveSelectedDrive),
            );
        }

        // NoRamdiskDevices page
        let mut no_ramdisks_col: Column<'_, Message> = Column::new().spacing(spacing);
        no_ramdisks_col = no_ramdisks_col.push(
            text("It looks like you don't have any ramdisk devices.")
                .width(Fill)
                .align_x(Alignment::Center),
        );
        no_ramdisks_col = no_ramdisks_col.push(Button::new("Create New Ramdisk").width(Fill));
        no_ramdisks_col = no_ramdisks_col.push(
            Button::new("Check for new Ramdisks")
                .width(Fill)
                .on_press(Message::DeviceListUpdated),
        );

        // Info page
        let mut info_col: Column<'_, Message> = Column::new().spacing(spacing);
        if let Some(status_text) = self.status_text.as_ref() {
            info_col = info_col.push(text(status_text).width(Fill).align_x(Alignment::Center));
            info_col = info_col.push(
                Button::new("Dismiss")
                    .width(Fill)
                    .on_press(Message::InfoPageDismissed),
            );
        }

        let sel_ramdisk_page = container(sel_ramdisk_col);
        let no_ramdisks_page = container(no_ramdisks_col);
        let info_page = container(info_col);

        match self.current_page {
            AppPage::SelectRamdiskDevice => sel_ramdisk_page,
            AppPage::NoRamdiskDevices => no_ramdisks_page,
            AppPage::Info => info_page,
            _ => container(text("TODO")),
        }
        .padding(spacing)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .height(Fill)
    }

    /// Returns the appropriate page for the current situation based only on whether there are ramdisks currently mounted or not.
    fn appropriate_page(&self) -> AppPage {
        if self.ramdisk_mounts.is_empty() {
            AppPage::NoRamdiskDevices
        } else {
            AppPage::SelectRamdiskDevice
        }
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
