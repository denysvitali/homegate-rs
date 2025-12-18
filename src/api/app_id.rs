
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::api::{SECRET, USER_AGENT};

type HmacSha256 = Hmac<Sha256>;

#[tracing::instrument(level = "debug")]
fn calculate_hmac(s: &str) -> String {
    tracing::debug!("Calculating HMAC for authentication");
    let mut h: HmacSha256 = HmacSha256::new_from_slice(&SECRET)
        .expect("SECRET is valid");
    h.update(s.as_bytes());
    let result = h.finalize().into_bytes();


    let b = result[result.len() - 1] & 0xF;
    let mut buffer: Vec<u8> = vec![0; 4];
    let mut j = 0;
    loop {
        let i = j;
        j += 1;
        buffer[i] = result[i + b as usize];
        if j > 3 {
            buffer[0] &= 0xFF;
            let mut rdr = Cursor::new(buffer);
            let n = rdr.read_i32::<BigEndian>()
                .expect("buffer contains valid i32");
            return format!("{}", n);
        }
    }
}

#[tracing::instrument(level = "debug")]
pub fn calculate_app_id(time: &chrono::NaiveDateTime) -> String {
    tracing::debug!("Generating App ID for authentication");
    let time_millis = time.and_utc().timestamp_millis() as u64;
    let ceil = (f64::from((time_millis / 1000) as u32) / 60.0).ceil();
    let s = format!("{}{}{}", USER_AGENT, app_version(), ceil);

    calculate_hmac(&s)
}

pub fn app_version() -> String {
    let sdk_version = 30;
    format!("Homegate/12.6.0/12060003/Android/{}", sdk_version)
}

#[cfg(test)]
mod test {
    
    
    

    use crate::api::app_id::calculate_app_id;

    #[test]
    fn test_app_id() -> Result<(), std::io::Error> {
        assert_eq!("1926888397", calculate_app_id(
            &chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd_opt(2022, 1, 25).unwrap(),
                chrono::NaiveTime::from_hms_opt(1, 30, 56).unwrap()),
        ));
        Ok(())
    }
}