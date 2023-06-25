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

use dioxus::html::style;
#[cfg(feature = "desktop")]
use dioxus::prelude::*;

/// Function for showing information about the program

#[cfg(feature = "desktop")]
pub fn information(cx: Scope) -> Element {
	render! {
		rsx! {
			style {include_str!("../style/dark_theme.css")},

			h1 {
				style: "color: red",
				"Information about the program"
			}

			p {
				style: "color: white",
				"Copyright 2023 Andrew Kushyk"
			}

			p {
				style: "color: white",
				"Licensed under the Apache License, Version 2.0 (the \"License\");
				   you may not use this file except in compliance with the License.
				   You may obtain a copy of the License at"
			}

			a {
				style: "color: red",
				href: "http://www.apache.org/licenses/LICENSE-2.0",
				"http://www.apache.org/licenses/LICENSE-2.0"
			}

			p {
				style: "color: white",
				"Unless required by applicable law or agreed to in writing, software
				distributed under the License is distributed on an \"AS IS\" BASIS,
				WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
				See the License for the specific language governing permissions and
				limitations under the License."
			}

			p {
				style: "color: white",
				"------------------------------"
			}

			p {
				style: "color: white",
				"Project repository: "
				a {
					style: "color: red",
					href: "https://github.com/git-user-cpp/fideus",
					"https://github.com/git-user-cpp/fideus"
				}
			}

			p {
				style: "color: white",
				"Project owner: "
				a {
					style: "color: red",
					href: "https://github.com/git-user-cpp",
					"https://github.com/git-user-cpp"
				}
			}

			p {
				style: "color: white",
				"Contributors: "
				a {
					style: "color: red",
					href: "https://github.com/git-user-cpp/fideus/graphs/contributors",
					"https://github.com/git-user-cpp/fideus/graphs/contributors"
				}
			}
		}
	}
}