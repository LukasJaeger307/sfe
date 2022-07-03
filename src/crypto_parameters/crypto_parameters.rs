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

fn byte_array_to_u32(byte_array : [u8; 4]) -> u32 {
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
        let actual_integer: u32 = byte_array_to_u32(to_convert);
        assert_eq!(expected_integer, actual_integer);
    }
}

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
}
