pub mod ramdisk;

use iced::{
    Alignment,
    Length::{Fill, FillPortion},
    Size, application,
    widget::{Button, Column, Container, Row, Text, container, pick_list, text},
};
use iced_aw::{ICED_AW_FONT_BYTES, number_input};
use nix::unistd::{getegid, geteuid};
use rfd::FileDialog;
use std::env::var;

use crate::ramdisk::{DataStorageUnit, EmptyDataStorageUnit, RamdiskMount};

struct AppState {
    ramdisk_mounts: Vec<RamdiskMount>,
    selected_ramdisk: Option<RamdiskMount>,
    current_page: AppPage,
    status_text: Option<String>,
    ramdisk_size: f64,
    ramdisk_unit: EmptyDataStorageUnit,
    ramdisk_location: String,
}

#[derive(Debug, Clone)]
enum Message {
    DeviceSelected(RamdiskMount),
    RemoveSelectedDrive,
    DeviceListUpdated,
    ResetPage,
    RamdiskUnitSizeChanged(EmptyDataStorageUnit),
    RamdiskSizeChanged(f64),
    OpenNewRamdiskPage,
    CreateRamdisk(DataStorageUnit),
    SelectNewRamdiskLocation,
}

#[derive(PartialEq)]
enum AppPage {
    Info,
    NoRamdiskDevices,
    SelectRamdiskDevice,
    NewRamdiskDevice,
}

// TODO: Make functions that return widgets with the common settings
impl AppState {
    fn new() -> Self {
        let ramdisk_mounts = RamdiskMount::from_existing(true).unwrap();
        let is_ramdisk_mounts_empty = ramdisk_mounts.is_empty();

        Self {
            ramdisk_mounts,
            selected_ramdisk: None,
            current_page: {
                if is_ramdisk_mounts_empty {
                    AppPage::NoRamdiskDevices
                } else {
                    AppPage::SelectRamdiskDevice
                }
            },
            status_text: None,
            ramdisk_size: 1.0,
            ramdisk_unit: EmptyDataStorageUnit::Mebibyte,
            ramdisk_location: format!("{}/ramdisk", var("HOME").unwrap()),
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
            Message::ResetPage => {
                self.current_page = self.appropriate_page();
                self.update(Message::DeviceListUpdated);
            }
            Message::OpenNewRamdiskPage => self.current_page = AppPage::NewRamdiskDevice,
            Message::RamdiskUnitSizeChanged(unit) => self.ramdisk_unit = unit,
            Message::RamdiskSizeChanged(size) => self.ramdisk_size = size,
            Message::CreateRamdisk(size) => {
                let ramdisk = RamdiskMount::new(
                    &self.ramdisk_location,
                    size,
                    geteuid().as_raw(),
                    getegid().as_raw(),
                );
                self.current_page = AppPage::Info;
                self.status_text = match ramdisk {
                    Ok(_) => {
                        self.update(Message::DeviceListUpdated);
                        Some(format!("Successfully created ramdisk with the size {size}"))
                    },
                    Err(e) =>  match e.kind() {
                        std::io::ErrorKind::PermissionDenied => Some("Error: Failed to create ramdisk due to insufficient permissions\nDid you run this program as root?".to_string()),
                        _ => Some(format!("Error: {}", e.kind()))
                    }
                };
            }
            Message::SelectNewRamdiskLocation => {
                let new_location = FileDialog::new().pick_folder();
                if let Some(new_path) = new_location
                    && let Ok(path) = new_path.into_os_string().into_string()
                {
                    self.ramdisk_location = path;
                }
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
        .width(FillPortion(17));
        let refresh_button: Button<'_, Message> = Button::new("🗘")
            .width(FillPortion(3))
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

            sel_ramdisk_col = sel_ramdisk_col.push(
                Button::new("Create new Ramdisk")
                    .on_press(Message::OpenNewRamdiskPage)
                    .width(Fill),
            );
        }

        // NoRamdiskDevices page
        let mut no_ramdisks_col: Column<'_, Message> = Column::new().spacing(spacing);
        no_ramdisks_col = no_ramdisks_col.push(
            text("It looks like you don't have any ramdisk devices.")
                .width(Fill)
                .align_x(Alignment::Center),
        );
        no_ramdisks_col = no_ramdisks_col.push(
            Button::new("Create New Ramdisk")
                .width(Fill)
                .on_press(Message::OpenNewRamdiskPage),
        );
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
                    .on_press(Message::ResetPage),
            );
        }

        // NewRamdiskDevice page
        let mut input_row: Row<'_, Message> = Row::new().spacing(5);
        let empty_data_storage_units = vec![
            EmptyDataStorageUnit::Kibibyte,
            EmptyDataStorageUnit::Mebibyte,
            EmptyDataStorageUnit::Gibibyte,
        ];
        let unit_input = pick_list(
            empty_data_storage_units,
            Some(self.ramdisk_unit),
            Message::RamdiskUnitSizeChanged,
        )
        .width(FillPortion(1));
        let size_input = number_input(&self.ramdisk_size, 1.0..=1024.0, |size| {
            Message::RamdiskSizeChanged(size)
        })
        .width(FillPortion(3));

        input_row = input_row.push(size_input);
        input_row = input_row.push(unit_input);

        let mut new_ramdisk_dev_col = Column::new().spacing(spacing);
        let current_folder_text: Text = text(format!(
            "The new ramdisk device will be created at {}",
            self.ramdisk_location
        ))
        .width(Fill)
        .align_x(Alignment::Center);
        let select_folder_button: Button<'_, Message> = Button::new("Choose Folder 🗁")
            .on_press(Message::SelectNewRamdiskLocation)
            .width(Fill);
        let create_ramdisk_button: Button<'_, Message> = Button::new("Create Ramdisk")
            .on_press(Message::CreateRamdisk(
                self.ramdisk_unit.to_data_storage_unit(self.ramdisk_size),
            ))
            .width(Fill);
        let cancel_button: Button<'_, Message> = Button::new("Cancel")
            .on_press(Message::ResetPage)
            .width(Fill);

        new_ramdisk_dev_col = new_ramdisk_dev_col.push(input_row);
        new_ramdisk_dev_col = new_ramdisk_dev_col.push(current_folder_text);
        new_ramdisk_dev_col = new_ramdisk_dev_col.push(select_folder_button);
        new_ramdisk_dev_col = new_ramdisk_dev_col.push(create_ramdisk_button);
        new_ramdisk_dev_col = new_ramdisk_dev_col.push(cancel_button);

        // Code shared by all pages
        let sel_ramdisk_page = container(sel_ramdisk_col);
        let no_ramdisks_page = container(no_ramdisks_col);
        let info_page = container(info_col);
        let new_ramdisk_page = container(new_ramdisk_dev_col);

        match self.current_page {
            AppPage::SelectRamdiskDevice => sel_ramdisk_page,
            AppPage::NoRamdiskDevices => no_ramdisks_page,
            AppPage::Info => info_page,
            AppPage::NewRamdiskDevice => new_ramdisk_page,
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
        .font(ICED_AW_FONT_BYTES)
        .window_size(Size {
            width: 300_f32,
            height: 400_f32,
        })
        .title("Easy Linux Ramdisk")
        .run()
}
