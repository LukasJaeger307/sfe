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
pub enum OperationMode {
    Gcm,
}

impl OperationMode {
    pub fn to_integer(&self) -> u32 {
        match self {
            OperationMode::Gcm => 1,
        }
    }

    pub fn from_integer(integer : u32) -> Option<OperationMode> {
        match integer {
            1 => Some(OperationMode::Gcm),
            _ => None,
        }
    }
}

#[cfg(test)]
mod operation_mode_tests {
    use super::*;

    #[test]
    fn test_operation_mode_to_integer() {
        assert_eq!(OperationMode::Gcm.to_integer(), 1);
    }
    
    #[test]
    fn test_operation_mode_from_integer() {
        assert_eq!(OperationMode::from_integer(0), None);
        assert_eq!(OperationMode::from_integer(1).unwrap(), OperationMode::Gcm);
        assert_eq!(OperationMode::from_integer(2), None);
    }
}
