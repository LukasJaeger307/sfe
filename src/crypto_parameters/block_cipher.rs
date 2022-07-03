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
    pub fn to_integer(&self) -> u32 {
        match self {
            BlockCipher::Aes => 1,
            BlockCipher::Camellia => 2,
        }
    }

    pub fn from_integer(integer : u32) -> Option<BlockCipher> {
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
