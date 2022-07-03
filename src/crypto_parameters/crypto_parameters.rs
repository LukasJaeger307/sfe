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
use crate::crypto_parameters::block_cipher::BlockCipher;
use crate::crypto_parameters::kdf::Kdf;
use crate::crypto_parameters::key_size::KeySize;
use crate::crypto_parameters::operation_mode::OperationMode;

fn u32_to_byte_array(integer : u32) -> [u8; 4] {
    let mut byte_array : [u8; 4] = [0xFF ; 4];
    for i in 0..4 {
        byte_array[i] = ((integer >> (3 - i) * 8) & 0xFF) as u8;
    }
    byte_array
}

fn byte_array_to_u32(byte_array : &[u8; 4]) -> u32 {
    let mut integer : u32 = 0x00000000;
    for i in 0..4 {
        integer |= (byte_array[i] as u32) << ((3 - i) * 8);
    }
    integer
}

#[cfg(test)]
mod utility_function_tests {
    use super::*;

    #[test]
    fn test_u32_to_byte_array() {
        let to_convert : u32 = 0x02AB1D00;
        let expected_array : [u8; 4] = [0x02, 0xAB, 0x1D, 0x00];
        let actual_array: [u8; 4] = u32_to_byte_array(to_convert);
        assert_eq!(expected_array, actual_array);
    }

    #[test]
    fn test_byte_array_to_u32() {
        let to_convert : [u8; 4] = [0x02, 0xAB, 0x1D, 0x00];
        let expected_integer : u32 = 0x02AB1D00;
        let actual_integer: u32 = byte_array_to_u32(&to_convert);
        assert_eq!(expected_integer, actual_integer);
    }
}

#[derive(PartialEq, Debug)]
pub struct CryptoParameters {
    block_cipher : BlockCipher,
    kdf : Kdf,
    key_size : KeySize,
    operation_mode : OperationMode,
}

impl CryptoParameters {
    pub fn to_byte_buffer(&self) -> [u8; 16] {
        let mut byte_buffer : [u8; 16] = [0xFF; 16];
        
        let buffers : [[u8; 4]; 4] = [
            self.get_block_cipher_byte_buffer(),
            self.get_kdf_byte_buffer(),
            self.get_key_size_byte_buffer(),
            self.get_operation_mode_byte_buffer(),
        ];

        for i in 0..4 {
            for j in 0..4 {
                byte_buffer[j + i * 4] = buffers[i][j];
            }
        }
        
        byte_buffer
    }

    fn get_block_cipher_byte_buffer(&self) -> [u8;4] {
        let block_cipher_integer = self.block_cipher.to_integer();
        u32_to_byte_array(block_cipher_integer)
    }

    fn get_kdf_byte_buffer(&self) -> [u8; 4] {
        let kdf_integer = self.kdf.to_integer();
        u32_to_byte_array(kdf_integer)
    }

    fn get_key_size_byte_buffer(&self) -> [u8; 4] {
        let key_size_integer = self.key_size.to_integer();
        u32_to_byte_array(key_size_integer)
    }

    fn get_operation_mode_byte_buffer(&self) -> [u8; 4] {
        let operation_mode_integer = self.operation_mode.to_integer();
        u32_to_byte_array(operation_mode_integer)
    }

    pub fn from_byte_buffer(byte_buffer : &[u8; 16]) -> Option<CryptoParameters> {
        let block_cipher_integer : u32 = Self::get_sub_array_at_offset_as_u32(&byte_buffer, 0);
        let block_cipher : Option<BlockCipher> = 
            BlockCipher::from_integer(block_cipher_integer);
        if block_cipher == None {
            return None;
        }

        let kdf_integer : u32 = Self::get_sub_array_at_offset_as_u32(byte_buffer, 4);
        let kdf : Option<Kdf> = 
            Kdf::from_integer(kdf_integer);
        if kdf == None {
            return None;
        }

        let key_size_integer : u32 = Self::get_sub_array_at_offset_as_u32(byte_buffer, 8);
        let key_size : Option<KeySize> = 
            KeySize::from_integer(key_size_integer);
        if key_size == None {
            return None;
        }
        
        let operation_mode_integer : u32 = Self::get_sub_array_at_offset_as_u32(byte_buffer, 12);
        let operation_mode : Option<OperationMode> = 
            OperationMode::from_integer(operation_mode_integer);
        if operation_mode == None {
            return None;
        }

        Some(CryptoParameters{
            block_cipher : block_cipher.unwrap(),
            kdf : kdf.unwrap(),
            key_size : key_size.unwrap(),
            operation_mode : operation_mode.unwrap(),
        })
    }

    fn get_sub_array_at_offset_as_u32(array : &[u8; 16], offset : usize) -> u32 {
        let mut sub_array : [u8; 4] = [0xFF; 4];
        for i in 0..4 {
            sub_array[i] = array[i + offset];
        }
        byte_array_to_u32(&sub_array)
    }
}

#[cfg(test)]
mod crypto_parameters_tests {
    use super::*;

    #[test]
    fn test_crypto_parameters_to_byte_buffer() {
        let parameters : CryptoParameters = CryptoParameters{
            block_cipher : BlockCipher::Camellia,
            kdf : Kdf::Pbkdf2,
            key_size : KeySize::Size256,
            operation_mode : OperationMode::Gcm,
        };
        let expected_array : [u8; 16] = 
            [0x00, 0x00, 0x00, 0x02
            ,0x00, 0x00, 0x00, 0x01
            ,0x00, 0x00, 0x01, 0x00
            ,0x00, 0x00, 0x00, 0x01];
        let actual_array : [u8; 16] = parameters.to_byte_buffer();
        assert_eq!(expected_array, actual_array);
    }

    #[test]
    fn test_crypto_parameters_from_byte_buffer_wrong_block_cipher() {
        let byte_buffer : [u8; 16] = 
            [0x00, 0x00, 0x00, 0x00
            ,0x00, 0x00, 0x00, 0x01
            ,0x00, 0x00, 0x01, 0x00
            ,0x00, 0x00, 0x00, 0x01];
        assert_eq!(CryptoParameters::from_byte_buffer(&byte_buffer), None);
    }
    
    #[test]
    fn test_crypto_parameters_from_byte_buffer_wrong_kdf() {
        let byte_buffer : [u8; 16] = 
            [0x00, 0x00, 0x00, 0x02
            ,0x00, 0x00, 0x00, 0x00
            ,0x00, 0x00, 0x01, 0x00
            ,0x00, 0x00, 0x00, 0x01];
        assert_eq!(CryptoParameters::from_byte_buffer(&byte_buffer), None);
    }
    
    #[test]
    fn test_crypto_parameters_from_byte_buffer_wrong_key_size() {
        let byte_buffer : [u8; 16] = 
            [0x00, 0x00, 0x00, 0x02
            ,0x00, 0x00, 0x00, 0x01
            ,0x00, 0x00, 0x00, 0xFF
            ,0x00, 0x00, 0x00, 0x01];
        assert_eq!(CryptoParameters::from_byte_buffer(&byte_buffer), None);
    }
    
    #[test]
    fn test_crypto_parameters_from_byte_buffer_wrong_operation_mode() {
        let byte_buffer : [u8; 16] = 
            [0x00, 0x00, 0x00, 0x02
            ,0x00, 0x00, 0x00, 0x01
            ,0x00, 0x00, 0x01, 0x00
            ,0x00, 0x00, 0x00, 0x00];
        assert_eq!(CryptoParameters::from_byte_buffer(&byte_buffer), None);
    }
    
    #[test]
    fn test_crypto_parameters_from_byte_buffer() {
        let byte_buffer : [u8; 16] = 
            [0x00, 0x00, 0x00, 0x02
            ,0x00, 0x00, 0x00, 0x01
            ,0x00, 0x00, 0x01, 0x00
            ,0x00, 0x00, 0x00, 0x01];
        let parameters : CryptoParameters = 
            CryptoParameters::from_byte_buffer(&byte_buffer).unwrap();
        assert_eq!(parameters.block_cipher, BlockCipher::Camellia);
        assert_eq!(parameters.kdf, Kdf::Pbkdf2);
        assert_eq!(parameters.key_size, KeySize::Size256);
        assert_eq!(parameters.operation_mode, OperationMode::Gcm);
    }
}
