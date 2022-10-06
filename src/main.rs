mod profile;
use core::panic;
mod app_err;
mod profile_fetcher;
use app_err::Result;
use reqwest::header::{HeaderMap, HeaderValue};
mod http_client;

#[tokio::main]
async fn main() -> Result<()> {
    let builder = reqwest::ClientBuilder::new();
    let headers = get_default_header();

    let client = builder.default_headers(headers).build().unwrap();
    let res = client.get("https://www.tiktok.com/api/user/detail/?aid=1988&app_language=en&app_name=tiktok_web&browser_language=en-US&browser_name=Mozilla&browser_online=true&browser_platform=Linux%20x86_64&browser_version=5.0%20%28X11%29&channel=tiktok_web&cookie_enabled=true&device_id=7151405184076678658&device_platform=web_pc&focus_state=true&from_page=user&history_len=4&is_fullscreen=false&is_page_visible=true&language=en&os=linux&priority_region=&referer=&region=VN&screen_height=1080&screen_width=1920&secUid=&tz_name=Asia%2FHo_Chi_Minh&uniqueId=therock&verifyFp=verify_l8x5iraf_LXxZfIT6_MiXs_4jep_8rnY_PcoGDF2x2XNN&webcast_language=en&msToken=_St_1yBtqkdqj8jFZcB_Rwanwv7RvoRGlNOEmom9sI-ooZ-jbUYGAH95WNmLKL2SFSTCuoZQwPDxBg6wCw752d-QNVsy5rVyBgL2fyaajhiMIbhbdwrD6ISNOv6bKSVradlKjmUnlmfM&X-Bogus=DFSzsIVL9fhANaIASKYiNBGgimrH&_signature=_02B4Z6wo00001M9WkEwAAIDBQDcpDomsIMjPV5TAAFDwea").send().await;

    match res {
        Ok(res) => {
            if res.status().is_client_error() || res.status().is_server_error() {
                println!("{}", res.status());
                return Ok(());
            }
            let text = res.text().await?;
            println!("{}", text);
        }
        Err(_) => {
            panic!();
        }
    };

    Ok(())
}

fn get_default_header() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        "User-Agent",
        HeaderValue::from_static(
            "Mozilla/5.0 (X11; Linux x86_64; rv:105.0) Gecko/20100101 Firefox/105.0",
        ),
    );

    headers
}
