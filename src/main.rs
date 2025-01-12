use anyhow::Result;
use headless_chrome::{Browser, Element, LaunchOptionsBuilder, Tab};
use std::fs::File;
use std::io::{copy, Cursor};
use std::time::Duration;
use std::{sync::Arc, thread, time};

// const url: &str = "https://www.spglobal.com/commodityinsights/PlattsContent/_assets/_files/en/our-methodology/methodology-specifications/faq-singapore-implied.pdf";

// async fn download_image_to(file_name: &str) -> Result<()> {
//     // Send an HTTP GET request to the URL
//     let response = reqwest::get(url).await?;
//     // Create a new file to write the downloaded image to
//     let mut file = File::create(file_name)?;
    
//     // Create a cursor that wraps the response body
//     let mut content =  Cursor::new(response.bytes().await?);
//     // Copy the content from the cursor to the file
//     copy(&mut content, &mut file)?;

//     Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     // let image_url = "https://www.rust-lang.org/static/images/rust-logo-blk.svg";
//     let file_name = "1.pdf";
//     match download_image_to(file_name).await {
//         Ok(_) => println!("image saved successfully"),
//         Err(e) => println!("error while downloading image: {}", e),
//     }
//     Ok(())
// }

fn main() {
    let mut commodites: Vec<Commodity> = Vec::new();

    let browser = browser();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to("https://www.spglobal.com/commodityinsights/en/our-methodology/methodology-specifications/oil/")
        .unwrap();

    let html_commodities = tab
        .wait_for_elements("#CommodityList > div.filterable-list__row")
        .unwrap();

    for html_commodity in html_commodities {
        let html_filterable_list_cell = html_commodity
            .wait_for_elements(".filterable-list__cell")
            .unwrap();

        if html_filterable_list_cell.len() < 3 {
            continue;
        }

        // for (i, x) in html_filterable_list_cell.iter().enumerate() {
        // }

        let commodity = html_filterable_list_cell[0].get_inner_text().unwrap();
        let title = html_filterable_list_cell[1].get_inner_text().unwrap();
        let published = html_filterable_list_cell[2].get_inner_text().unwrap();
        let href = html_filterable_list_cell[1]
            .find_element("a")
            .unwrap()
            .get_attribute_value("href")
            .unwrap()
            .unwrap();

        let commodity = Commodity {
            commodity,
            title,
            published,
            href,
        };

        commodites.push(commodity);
    }

    for c in commodites.iter() {
        if !c.href.is_empty() {
            thread::sleep(time::Duration::from_secs(2));
            open_tab(c);

            break;
        }
    }

    create_screenshort(tab, "screenshot_main.png".to_string());
}

fn browser() -> Browser {
    Browser::new(
        LaunchOptionsBuilder::default()
            .headless(false)
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn open_tab(commodity: &Commodity) {
    println!("Open tab: {}", commodity.href);

    let browser = browser();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to(&commodity.href).unwrap();

    let _ = tab.wait_for_element(".intro-copy");

    let html_tag_a = tab.wait_for_elements("a.link").unwrap();

    let mut file_names: Vec<String> = Vec::new();

    for a in html_tag_a {
        let href_attr = a.get_attribute_value("href").unwrap().unwrap();
        
        let href_split_path = &href_attr.split("/");
        let href_split_dot = href_split_path.clone().last().unwrap().split(".");
        
        if href_split_dot.clone().last() == Some(&"pdf")
            || href_split_dot.clone().last() == Some(&"xlsx")
        {
            // let file_name = href_split_path.clone().last().unwrap();

            if !file_names.contains(&href_attr){
                file_names.push(href_attr);

                let _ = excute_js(a);
            }

        //     // match save_file(&href, file_name) {
        //     //     Ok(_) => println!("image saved successfully"),
        //     //     Err(e) => println!("error while downloading image: {}", e),
        //     // }
        }
    }

    create_screenshort(tab, commodity.title.clone() + ".png");
}

fn excute_js(elem: Element<'_>) -> Result<(), Box<dyn std::error::Error>> {
    // Run JavaScript in the page
    let remote_object = elem.call_js_fn(r#"
        function getIdTwice () {
            // `this` is always the element that you called `call_js_fn` on
            
            const path = this.href.split("/");

            this.setAttribute("download", path[path.length-1]);  
            this.click();
            
            //return true;
        }
    "#, vec![], false)?;
    // match remote_object.value {
    //     Some(returned_string) => {
    //         dbg!(&returned_string);
    //         // assert_eq!(returned_string, "firstHeadingfirstHeading".to_string());
    //     }
    //     _ => unreachable!()
    // };
    Ok(())
}

fn save_file(target: &str, file_name: &str) -> Result<()> {
    let _ = File::create(file_name)?;

    let client = reqwest::blocking::Client::builder()
    .timeout(Duration::from_secs(10))
    .build()?;
    let  _ = client.get(target);//.copy_to(&mut file);
    //let mut file_len = reqwest::blocking::get(target)?.copy_to(&mut file)?;

    Ok(())
}

fn create_screenshort(tab: Arc<Tab>, path: String) {
    let screenshot_data = tab
        .capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
            None,
            None,
            true,
        )
        .unwrap();

    // write the screenshot data to the output file
    std::fs::write(path, &screenshot_data).unwrap();
}

struct Commodity {
    commodity: String,
    title: String,
    published: String,
    href: String,
}

struct Files {
    description: String,
    date: String,
    extension: String,
    file_path: String,
}
