use crate::command::Command;
use crate::db::Db;
use crate::resp::RespData;

impl Command {
    pub fn set(&self, storage: &mut Db) -> RespData {
        let key = self.args[0].clone().unpack_str();
        let value = self.args[1].clone().unpack_str();
        let expires = self.calculate_expires();

        if let Err(err) = expires {
            return RespData::Error(err);
        }

        storage.set(key, value, expires.unwrap());
        RespData::SimpleString("OK".to_string())
    }

    fn calculate_expires(&self) -> Result<u128, String> {
        if self.args.len() < 4 {
            return Ok(0);
        }

        let expiry_mode = self.args[2].clone().unpack_str();
        match expiry_mode.to_lowercase().as_str() {
            "px" => self.args[3]
                .clone()
                .unpack_str()
                .parse::<u128>()
                .map_err(|_| "ERR invalid expiry value".into()),
            _ => Err("ERR invalid expiry mode".into()),
        }
    }
}
