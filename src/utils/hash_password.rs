use sha2::{Digest, Sha256};

pub fn hash_password(username: &str, password: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(username.as_bytes());
    hasher.update(password.as_bytes());
    hasher.finalize().to_vec()
}

pub fn check_password(username: &str, password: &str, db_password: &[u8]) -> bool {
	db_password == hash_password(username, password)
}
