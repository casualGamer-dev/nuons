/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

//! Manage the configuration of the application.

use std::fs::{File, OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

use xdg::BaseDirectories;

use bookmarks::BookmarkManager;
use popup_manager::PopupManager;
use super::{App, AppResult, APP_NAME};

impl App {
    /// Create the default configuration files and directories if it does not exist.
    fn create_config_files(&self, config_path: &Path) -> AppResult {
        let xdg_dirs = BaseDirectories::with_prefix(APP_NAME)?;

        let bookmarks_path = BookmarkManager::config_path();

        let stylesheets_path = xdg_dirs.place_config_file("stylesheets")?;
        let scripts_path = xdg_dirs.place_config_file("scripts")?;
        create_dir_all(stylesheets_path)?;
        create_dir_all(scripts_path)?;

        let keys_path = xdg_dirs.place_config_file("keys")?;
        let webkit_config_path = xdg_dirs.place_config_file("webkit")?;
        let hints_css_path = xdg_dirs.place_config_file("stylesheets/hints.css")?;
        self.create_default_config_file(config_path, include_str!("../../config/config"))?;
        self.create_default_config_file(&keys_path, include_str!("../../config/keys"))?;
        self.create_default_config_file(&webkit_config_path, include_str!("../../config/webkit"))?;
        self.create_default_config_file(&hints_css_path, include_str!("../../config/stylesheets/hints.css"))?;
        self.create_default_config_file(&bookmarks_path, include_str!("../../config/bookmarks"))?;

        let (popup_whitelist_path, popup_blacklist_path) = PopupManager::config_path();
        OpenOptions::new().create(true).write(true).open(&popup_whitelist_path)?;
        OpenOptions::new().create(true).write(true).open(&popup_blacklist_path)?;

        Ok(())
    }

    /// Create the config file with its default content if it does not exist.
    fn create_default_config_file(&self, path: &Path, content: &'static str) -> AppResult {
        if !path.exists() {
            let mut file = File::create(path)?;
            write!(file, "{}", content)?;
        }
        Ok(())
    }

    /// Create the variables accessible from the config files.
    pub fn create_variables(app: Rc<Self>) {
        let application = app.clone();
        app.app.add_variable("url", move || {
            application.webview.get_uri().unwrap()
        });
    }

    /// Create the missing config files and parse the config files.
    pub fn parse_config(&self) {
        let xdg_dirs = BaseDirectories::with_prefix(APP_NAME).unwrap();
        let config_path = xdg_dirs.place_config_file("config")
            .expect("cannot create configuration directory");
        self.handle_error(self.create_config_files(config_path.as_path()));
        self.handle_error(self.app.parse_config(config_path));
    }
}
