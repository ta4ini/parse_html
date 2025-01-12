use anyhow::Result;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::fs::File;
use std::{sync::Arc, thread, time};

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

    for a in html_tag_a {
        let href = a.get_attribute_value("href").unwrap().unwrap();
        let mut href_split_dot = &href.split('.');
        if href_split_dot.clone().last() == Some(&"pdf")
            || href_split_dot.clone().last() == Some(&"xlsx")
        {
            let href_split_path = href.split("/");
            let file_name = href_split_path.last().unwrap();

            match save_file(&href, file_name) {
                Ok(_) => println!("image saved successfully"),
                Err(e) => println!("error while downloading image: {}", e),
            }
        }
    }

    create_screenshort(tab, commodity.title.clone() + ".png");
}

fn save_file(target: &str, file_name: &str) -> Result<()> {
    // let tmp_dir = Builder::new().prefix("example").tempdir()?;
    // let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let mut file = File::create(file_name)?;
    let mut file_len = reqwest::blocking::get(target)?.copy_to(&mut file)?;
    // let file_name = response
    //         .url()
    //         .path_segments()
    //         .and_then(|segments| segments.last())
    //         .and_then(|name| if name.is_empty() { None } else { Some(name) })
    //         .unwrap_or("tmp.bin");
    // Create a new file to write the downloaded image to

    // Copy the contents of the response to the file
    //copy(&mut response, &mut file)?;

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
