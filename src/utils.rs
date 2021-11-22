use anyhow::{Result, anyhow};
use multihash:: Multihash;
use rust_base58::{ToBase58, FromBase58};
use web3::ethabi::Token;

#[derive(Debug)]
pub struct IPFSMultihash{
    pub hash_code: [u8; 2],
    pub size: u8,
    pub digest: [u8; 32],

}
impl IPFSMultihash{
    pub fn new(cid: &str)-> Result<Self>{
        let hash = cid.from_base58().map_err(anyhow::Error::msg)?;
        let multi_hash = Multihash::from_bytes(&hash)?;

        let mut digest = [0_u8; 32];
        let mut hash_code = [0_u8; 2];

        digest.copy_from_slice(multi_hash.digest());
        hash_code.copy_from_slice(&multi_hash.code().to_be_bytes()[6..]);

        Ok(IPFSMultihash{
            hash_code: hash_code,
            size: multi_hash.size(),
            digest: digest
        })
    }
}

pub fn multi_hash_token_to_cid(token: Token) -> Result<String>{
    let function_name = "multi_hash_token_to_cid";
    let mut hash = [0_u8; 34]; 
    if let Token::Tuple(tokens) = token{
        if tokens.len() != 3 {
            return Err(anyhow!("Error in {:?} tokens must be of size 3 (digest, hash code and size)", function_name))
        }

        for (_, token) in tokens.iter().enumerate(){
            match token {
                Token::FixedBytes(x)=>{
                    if x.len() == 32 {
                        //Digest
                        hash[2..].copy_from_slice(&x)
                    }
                    else if x.len() == 2 {
                        //Hash code
                        hash[0] = x[1];
                    }
                    else{
                        return Err(anyhow!("Error in {:?} Error digest should be 32 bytes and code 2 bytes", function_name)) 
                    }
                },
                Token::Uint(size) => hash[1] = size.byte(0), //Size
                _ => return Err(anyhow!("Error in {:?} Error tokens can only have 2 types FixedBytes and Uint", function_name))
            }
        }
        return Ok(hash.to_base58());        
    }

    Err(anyhow!("Error in {:?} Error tokens must be in a tuple token", function_name))
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_ipfs_multihash_new() {

        let cid_b58 = "QmTJd6JnxTGrLgJqLfnhMHXaytrSaBHvos4ECeVTvqwHdi";
        let multi_hash = IPFSMultihash::new(&cid_b58).unwrap();
        let result = (multi_hash.digest, multi_hash.hash_code, multi_hash.size);

        let expected_digest: [u8; 32] = 
            [73, 197, 113, 196, 240, 65, 205, 146, 
            12, 137, 174, 231, 110, 152, 122, 213, 
            137, 129, 35, 3, 24, 171, 244, 195, 
            151, 160, 208, 0, 39, 253, 35, 161];
        
        let expected_hash_code: [u8; 2] = [0, 18];

        let expected_size: u8 = 32;

        assert_eq!(expected_digest, result.0);
        assert_eq!(expected_hash_code, result.1);
        assert_eq!(expected_size, result.2);
    }

    #[test]
    fn test_multi_hash_token_to_cid() {
        
        let cid_b58_expected = "QmTJd6JnxTGrLgJqLfnhMHXaytrSaBHvos4ECeVTvqwHdi";
        let digest: [u8; 32] = 
            [73, 197, 113, 196, 240, 65, 205, 146, 
            12, 137, 174, 231, 110, 152, 122, 213, 
            137, 129, 35, 3, 24, 171, 244, 195, 
            151, 160, 208, 0, 39, 253, 35, 161];
        
        let hash_code: [u8; 2] = [0, 18];

        let size: u8 = 32;

        let token_digest_input = Token::FixedBytes(digest.to_vec());
        let token_hash_code_input = Token::FixedBytes(hash_code.to_vec());
        let token_size_input = Token::Uint(size.into());
        let token_tuple_input = Token::Tuple(vec![token_digest_input.clone(), token_hash_code_input.clone(), token_size_input.clone()]);
        let cid_b58_result = multi_hash_token_to_cid(token_tuple_input).unwrap();

        assert_eq!(cid_b58_expected, cid_b58_result);

    }
}