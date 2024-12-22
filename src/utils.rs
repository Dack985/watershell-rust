use anyhow::Result;

pub fn calculate_checksum(data: &[u8]) -> u16 {
    let sum: u32 = data.chunks(2).map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]) as u32).sum();
    let carry = (sum >> 16) as u16;
    !((sum as u16) + carry)
}

pub fn get_interface_name() -> Result<String> {
    Ok("eth0".to_string())
}
