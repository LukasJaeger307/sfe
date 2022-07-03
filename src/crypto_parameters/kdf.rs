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
#[derive(PartialEq, Debug)]
pub enum Kdf {
    Pbkdf2,
}

impl Kdf {
    pub fn to_integer(&self) -> u32 {
        match self {
            Kdf::Pbkdf2 => 1,
        }
    }

    pub fn from_integer(integer : u32) -> Option<Kdf> {
        match integer {
            1 => Some(Kdf::Pbkdf2),
            _ => None,
        }
    }
}

#[cfg(test)]
mod kdf_tests {
    use super::*;

    #[test]
    fn test_kdf_to_integer() {
        assert_eq!(Kdf::Pbkdf2.to_integer(), 1);
    }
    
    #[test]
    fn test_kdf_from_integer() {
        assert_eq!(Kdf::from_integer(0), None);
        assert_eq!(Kdf::from_integer(1).unwrap(), Kdf::Pbkdf2);
        assert_eq!(Kdf::from_integer(2), None);
    }
}
