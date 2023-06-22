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

/// Function for showing calculations

#[cfg(feature = "desktop")]
pub fn calculations(cx: Scope) -> Element {
	render! {
		rsx! {
			style {include_str!("../style/dark_theme.css")},

			h1 {
				style: "color: red",
				"Need to be implemented"
			}
		}
	}
}