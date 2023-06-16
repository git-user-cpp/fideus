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
use crate::desktop::information::information;

pub fn main_menu(cx: Scope) -> Element {
	render! {
		rsx! {
			body {
				// style: "background-color: black",

				div {
					button {
						onclick: move |event| {
							println!("{event:?}")
						},
						"Insert Products"
					}
					button {
						onclick: move |event| {
							println!("{event:?}")
						},
						"Show List"
					}
					button {
						onclick: move |event| {
							println!("{event:?}")
						},
						"Show total sum & Show percentage"
					}
					button {
						//TODO: launch information(cx)!!!!!!!!!!
						onclick: move |event| {
							println!("{event:?}")
						},
						"Info about the program"
					}
				}
			}
		}
	}
}