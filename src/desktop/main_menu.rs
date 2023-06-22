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

#[cfg(feature = "desktop")]
use dioxus::prelude::*;

#[cfg(feature = "desktop")]
use crate::desktop::window::create_config;

#[cfg(feature = "desktop")]
use crate::desktop::impl_options::calculations::calculations;
#[cfg(feature = "desktop")]
use crate::desktop::impl_options::information::information;
#[cfg(feature = "desktop")]
use crate::desktop::impl_options::list::list;
#[cfg(feature = "desktop")]
use crate::desktop::impl_options::products::insert_product;

/// Function for rendering main menu window

#[cfg(feature = "desktop")]
pub fn main_menu(cx: Scope) -> Element {
	render! {
		rsx! {
			style {include_str!("./style/dark_theme.css")}

			div {
				button {
					style: "color: black",
					onclick: move |_| {
						insert_information(cx)
					},
					"Insert Products"
				}
			}

			div {
				button {
					style: "color: black",
					onclick: move |_| {
						show_list(cx)
					},
					"Show List"
				}
			}

			div {
				button {
					style: "color: black",
					onclick: move |_| {
						show_calculations(cx)
					},
					"Show total sum & Show percentage"
				}
			}

			div {
				button {
					style: "color: black",
					onclick: move |_| {
						show_information(cx)
					},
					"Info about the program"
				}
			}
		}
	}
}

/// Function for rendering window for entering products

#[cfg(feature = "desktop")]
fn insert_information(cx: Scope) {
	let window = dioxus_desktop::use_window(cx);

	window.new_window(VirtualDom::new(insert_product), create_config("FiDeus Insert Products"));
}

/// Function for rendering window for entering products

#[cfg(feature = "desktop")]
fn show_list(cx: Scope) {
	let window = dioxus_desktop::use_window(cx);

	window.new_window(VirtualDom::new(list), create_config("FiDeus List of Products"));
}

/// Function for rendering window for getting calculations

#[cfg(feature = "desktop")]
fn show_calculations(cx: Scope) {
	let window = dioxus_desktop::use_window(cx);

	window.new_window(VirtualDom::new(calculations), create_config("FiDeus Calculations"));
}

/// Function for rendering window with info about the program

#[cfg(feature = "desktop")]
fn show_information(cx: Scope) {
	let window = dioxus_desktop::use_window(cx);

	window.new_window(VirtualDom::new(information), create_config("FiDeus Information"));
}