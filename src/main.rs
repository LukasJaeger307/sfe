/* 
 * Copyright 2022, Lukas Jäger
 *
 * This file is part of SFE.
 *
 * SFE is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * SFE is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with SFE.  If not, see <http://www.gnu.org/licenses/>.
 */
mod crypto_parameters;
mod file_loader;
mod password_error;
mod password_getter;

use crate::password_getter::get_password;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password : String = match get_password() {
        Ok(string) => string,
        Err(error) => {
            return Err(error.into());
        }
    };
    println!("Your password is {}", password);
    Ok(())
}
