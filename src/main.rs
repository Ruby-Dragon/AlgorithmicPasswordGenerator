/*
Copyright (C) 2022 Ruby-Dragon

This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod input;
mod choose;
mod algorithms;

fn main() {
    //print prompts and get input
    println!("Algorithmic Password Generator, Copyright (C) 2022 Ruby-Dragon\nThis program comes with ABSOLUTELY NO WARRANTY. This is free software, and you are welcome to redistribute it under certain conditions. Enter -d when done to exit.");
    let mut input_words = input::input::get_words();

    //output finished code
    println!("Generated Password:");
    println!("{}", choose::choose::run_algorithms(input_words));
}
