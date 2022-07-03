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
pub enum BlockCipher {
    Aes,
    Camellia,
}

impl BlockCipher {
    fn to_integer(&self) -> u32 {
        match self {
            BlockCipher::Aes => 1,
            BlockCipher::Camellia => 2,
        }
    }

    fn from_integer(integer : u32) -> Option<BlockCipher> {
        match integer {
            1 => Some(BlockCipher::Aes),
            2 => Some(BlockCipher::Camellia),
            _ => None,
        }
    }
}

#[cfg(test)]
mod block_cipher_tests {
    use super::*;

    #[test]
    fn test_block_cipher_to_integer() {
        assert_eq!(BlockCipher::Aes.to_integer(), 1);
        assert_eq!(BlockCipher::Camellia.to_integer(), 2);
    }

    #[test]
    fn test_block_cipher_from_integer() {
        assert_eq!(BlockCipher::from_integer(0), None);
        assert_eq!(BlockCipher::from_integer(1).unwrap(), BlockCipher::Aes);
        assert_eq!(BlockCipher::from_integer(2).unwrap(), BlockCipher::Camellia);
        assert_eq!(BlockCipher::from_integer(3), None);
    }
}

#[derive(PartialEq, Debug)]
pub enum OperationMode {
    Gcm,
}

impl OperationMode {
    fn to_integer(&self) -> u32 {
        match self {
            OperationMode::Gcm => 1,
        }
    }

    fn from_integer(integer : u32) -> Option<OperationMode> {
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

#[derive(PartialEq, Debug)]
pub enum Kdf {
    Pbkdf2,
}

impl Kdf {
    fn to_integer(&self) -> u32 {
        match self {
            Kdf::Pbkdf2 => 1,
        }
    }

    fn from_integer(integer : u32) -> Option<Kdf> {
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

#[derive(PartialEq, Debug)]
pub enum KeySize {
    Size128,
    Size192,
    Size256,
}

impl KeySize {
    fn to_integer(&self) -> u32 {
        match self {
            KeySize::Size128 => 128, 
            KeySize::Size192 => 192, 
            KeySize::Size256 => 256, 
        }
    }

    fn from_integer(integer : u32) -> Option<KeySize> {
        match integer {
            128 => Some(KeySize::Size128),
            192 => Some(KeySize::Size192),
            256 => Some(KeySize::Size256),
            _ => None,
        }
    }
}

#[cfg(test)]
mod key_size_tests {
    use super::*;

    #[test]
    fn test_key_size_to_integer() {
        assert_eq!(KeySize::Size128.to_integer(), 128);
        assert_eq!(KeySize::Size192.to_integer(), 192);
        assert_eq!(KeySize::Size256.to_integer(), 256);
    }
    
    
    #[test]
    fn test_key_size_from_integer() {
        assert_eq!(KeySize::from_integer(127), None);
        assert_eq!(KeySize::from_integer(128).unwrap(), KeySize::Size128);
        assert_eq!(KeySize::from_integer(129), None);
        assert_eq!(KeySize::from_integer(191), None);
        assert_eq!(KeySize::from_integer(192).unwrap(), KeySize::Size192);
        assert_eq!(KeySize::from_integer(193), None);
        assert_eq!(KeySize::from_integer(255), None);
        assert_eq!(KeySize::from_integer(256).unwrap(), KeySize::Size256);
        assert_eq!(KeySize::from_integer(257), None);
    }
}
