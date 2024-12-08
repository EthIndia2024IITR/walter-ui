use std::process::{Command, Stdio};

use ratatui::{style::{Color, Style}, text::{Line, Text}, widgets::{ListState, ScrollbarState, TableState}};
use serde::Serialize;

use crate::utils::{BlobInfo, StorageInfo};

pub enum CurrentScreen {
    Splash,
    Dashboard,
    Uploader,
    Migrator,
    SharderAndEpochExtender,
}

pub struct App {
    pub sui_active_address: String,
    pub sui_active_env: String,
    pub current_screen: CurrentScreen,
    pub should_quit: bool,
    pub table_state: TableState,
    pub scrollbar_state: ScrollbarState,
    pub user_blobs: Vec<BlobInfo>,
    pub walrus_system_info: String,
    pub is_editing: bool,
    pub filename: String,
    pub pinata_api_key: String,

    pub shard_pass: String,
    pub shard_encrypt: bool,
    pub extender_blob_id: String,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Splash,
            should_quit: false,
            table_state: TableState::default().with_selected(0),
            user_blobs: Vec::new(),
            scrollbar_state: ScrollbarState::new(0),
            sui_active_address: String::new(),
            sui_active_env: String::new(),
            walrus_system_info: String::new(),
            is_editing: false,
            filename: String::new(),
            pinata_api_key: String::new(),
            shard_pass: String::new(),
            shard_encrypt: false,
            extender_blob_id: String::new(),
        }
    }
    pub fn next_row(&mut self) {
        if !self.user_blobs.is_empty() {
            let i = match self.table_state.selected() {
                Some(i) => {
                    if i >= self.user_blobs.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.table_state.select(Some(i));
            self.scrollbar_state = self.scrollbar_state.position(i);
        }
    }
    pub fn prev_row(&mut self) {
        if !self.user_blobs.is_empty() {
            let i = match self.table_state.selected() {
                Some(i) => {
                    if i <= 0 {
                        self.user_blobs.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.table_state.select(Some(i));
            self.scrollbar_state = self.scrollbar_state.position(i);
        }
    }

    pub fn upload_file(&mut self) {
        // let output
        // yahan likhna file upload logic
    }
}
