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
use dioxus_router::{Router, Route, Link};

#[cfg(feature = "desktop")]
use crate::desktop::impl_options::calculations::calculations;
#[cfg(feature = "desktop")]
use crate::desktop::impl_options::information::information;
#[cfg(feature = "desktop")]
use crate::desktop::impl_options::list::list;
#[cfg(feature = "desktop")]
use crate::desktop::impl_options::products::insert_product;

fn main_options(cx: Scope) -> Element {
	cx.render(rsx! {
		div {
			button {
				Link {
					id: "option",
					to: "/insert_information", "Insert information"
				}
			}
			button {
				Link {
					id: "option",
					to: "/show_list", "Show information"
				}
			}
			button {
				Link {
					id: "option",
					to: "/show_calculations", "Show calculations"
				}
			}
			button {
				Link {
					id: "option",
					to: "/information", "Information"
				}
			}
		}
    })
}

/// Function for rendering main menu window

#[cfg(feature = "desktop")]
pub fn main_menu(cx: Scope) -> Element {
	render! {
		rsx! {
			style {include_str!("./style/dark_theme.css")}

			Router {
	            self::main_options {}

				Route {
					to: "/insert_information", self::insert_product {}
				}
				Route {
					to: "/show_list", self::list {}
				}
				Route {
					to: "/show_calculations", self::calculations {}
				}
	            Route {
					to: "/information", self::information {}
				}
            }
		}
	}
}