use crate::command::Command;
use crate::db::Db;
use crate::resp::RespData;

impl Command {
    pub fn set(&self, storage: &mut Db) -> RespData {
        let key = self.args[0].clone().unpack_str();
        let value = self.args[1].clone().unpack_str();

        let expires: u128 = if self.args.len() < 4 {
            0
        } else {
            let expiry_mode = self.args[2].clone().unpack_str();
            match expiry_mode.to_lowercase().as_str() {
                "px" => self.args[3].clone().unpack_str().parse::<u128>().unwrap(),
                _ => return RespData::Error("ERR invalid expiry mode".into()),
            }
        };

        println!("Setting key: {key}, value: {value}, expires: {expires}");
        storage.set(key, value, expires);
        RespData::SimpleString("OK".to_string())
    }
}
