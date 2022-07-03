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
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PasswordError;

impl fmt::Display for PasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid password")
    }
}

impl Error for PasswordError{} 

pub fn get_password() -> Result<String, PasswordError>{
    let password1 = rpassword::prompt_password("Insert password: ").unwrap();
    let password2 = rpassword::prompt_password("Insert password again: ").unwrap();
    if password1 != password2 {
        return Err(PasswordError);
    } else if password1.trim().is_empty() {
        return Err(PasswordError);
    } else {
        return Ok(password1);
    }
}
