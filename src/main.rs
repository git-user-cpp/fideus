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

// Module for using this program via console/terminal;

#[cfg(feature = "console")]
mod console;

// Module for using this program via GUI;

#[cfg(feature = "desktop")]
mod desktop;

mod product_structure;

#[cfg(feature = "console")]
use crate::console::run_console::run_console;
#[cfg(feature = "desktop")]
use crate::desktop::run_desktop::run_desktop;

fn main() {
	#[cfg(feature = "desktop")]
	run_desktop();

	#[cfg(feature = "console")]
	run_console();
}
