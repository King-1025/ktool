mod tool;
mod fmt;

use self::fmt::{
    pick_data
};

//use std::env;
use self::tool::{
    sign,
    stime,
    x3des
};

use std::io::{
//    stdout, 
      Read, 
//    Write
};

use curl::easy::{
    Easy,
    List
};

use shell_command::run_shell_command;

pub fn glive_request() -> String {
//    run_shell_command("/termux/self/bin/gff json").unwrap()
    run_shell_command(env!("GLIVE_EXEC")).unwrap()
}

pub fn doinai_request(page : &str) -> String {
    let mut easy = Easy::new();
    let mut res = Vec::new();

    let data = format!("uid=-1&p={}", page);
    let mut data = data.as_bytes();

    easy.url("https://www.orq652.com/api/public/?service=Video.getVideoList").unwrap();
    easy.post(true).unwrap();
    easy.http_headers(get_headers()).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    {
      let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.write_function(|data| {
        res.extend_from_slice(data);
 //       stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
      transfer.perform().unwrap();
    }
    pick_data(&String::from_utf8(res).unwrap_or_else(|_| "".into()))
}

fn get_headers() -> List {
   let mut list = List::new();
   let (ts, tsx) = stime();
   list.append("Accept-Charset: */*").unwrap();
   list.append("http_referer: www.51dounai.com").unwrap();
   list.append("User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/63.0.3239.132 Safari/537.36").unwrap();
   list.append("http_app_version: 1.1.0").unwrap();
   list.append("safe: dounaisafecouncil").unwrap();
   list.append("Accept-Language: zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5").unwrap();
   list.append("deviceid: 865982020825298").unwrap();
   list.append("Content-Type: application/x-www-form-urlencoded").unwrap();
   list.append("http_user_agent: dounaivod").unwrap();
   list.append(&format!("signature: {}", sign(&format!("865982020825298Xiaomi1.1.0{}zfQPeEB6LfVfysW7zA7cc5fbuGd3qzAE", ts)))).unwrap();
   list.append(&format!("timestamp: {}", x3des("1a0dcc06af4585e83a1c4967", "yr3OMKu8", &tsx))).unwrap();
   return list;
}
