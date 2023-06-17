/*
Copyright 2023 Andrew Kushyk

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use crate::desktop::main_menu::main_menu;

/// Function for running as a desktop program

pub fn run_desktop() {
	let config = Config::new().with_window(
		WindowBuilder::default()
			.with_title("FiDeus")
	);

	dioxus_desktop::launch_cfg(app, config);
}

/// Interface render

fn app(cx: Scope) -> Element {
	main_menu(cx)
}