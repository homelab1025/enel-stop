import os
import time

from selenium import webdriver
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.common.by import By
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.support.ui import WebDriverWait
from xvfbwrapper import Xvfb

with Xvfb() as xvfb:
    # Set up Chrome options for downloading
    download_dir = os.path.join(os.getcwd(), "rss_downloads")  # Create a subfolder for downloads
    if not os.path.exists(download_dir):
        os.makedirs(download_dir)

    chrome_options = Options()
    # chrome_options.add_argument("--headless=new") # if you need explicit headless mode beyond xvfb
    chrome_options.add_argument("--no-sandbox")
    chrome_options.add_argument("--disable-dev-shm-usage")
    chrome_options.add_argument("--window-size=1920,1080")

    prefs = {
        "download.default_directory": download_dir,
        "download.prompt_for_download": False,  # Automatically download files
        "download.directory_upgrade": True,
        "safeBrowse.enabled": True
    }
    chrome_options.add_experimental_option("prefs", prefs)

    driver = webdriver.Chrome(options=chrome_options)

    try:
        # 1. Navigate to the main page
        print("Navigating to https://www.reteleelectrice.ro/intreruperi/programate/")
        driver.get("https://www.reteleelectrice.ro/intreruperi/programate/")

        # Set a fixed window size for headless mode
        driver.set_window_size(1920, 1080)
        time.sleep(2)  # Give some time for initial load and elements to settle

        # --- NEW: Reject cookie consent using the specified ID ---
        print("Attempting to reject all cookies (onetrust-reject-all-handler)...")
        try:
            reject_cookies_button = WebDriverWait(driver, 10).until(
                EC.element_to_be_clickable((By.ID, "onetrust-reject-all-handler"))
            )
            reject_cookies_button.click()
            print("Successfully clicked 'Reject All' cookies.")
            time.sleep(3)  # Give time for the banner to disappear and page to settle
        except Exception as e:
            print(f"Could not find or click 'onetrust-reject-all-handler' button: {e}")
            print("Continuing without rejecting cookies, if banner is not present or handled differently.")
            pass  # Continue if the button isn't found or an error occurs

        # 2. Simulate scrolling down the page to view more content
        print("Scrolling down the page...")
        driver.execute_script("window.scrollBy(0, 500);")
        time.sleep(2)

        driver.execute_script("window.scrollBy(0, 500);")
        time.sleep(2)

        # 5. Locate and click the specific A href by its href attribute
        target_href = "https://www.reteleelectrice.ro/rss-outages.xml"
        print(f"Locating the A href by its href attribute: {target_href}")

        try:
            # Using CSS selector to find an 'a' tag with a specific 'href' attribute
            specific_link = WebDriverWait(driver, 15).until(
                EC.element_to_be_clickable((By.CSS_SELECTOR, f'a[href="{target_href}"]'))
            )
            print("Element found. Attempting to scroll into view...")
            driver.execute_script("arguments[0].scrollIntoView({block: 'center'});", specific_link)
            time.sleep(1)

            link_href = specific_link.get_attribute("href")
            print(f"Found link with href: {link_href}. Attempting click to trigger download...")

            specific_link.click()  # Now we actually click to trigger the download
            print("Clicked the specific A href. Download should be initiated by ChromeDriver.")

            # --- Important: Wait for the download to complete ---
            # This is tricky in headless. You'd typically poll the download directory.
            downloaded_file_path = os.path.join(download_dir, "rss-outages.xml")  # Assuming the default filename
            timeout = 30
            start_time = time.time()
            while not os.path.exists(downloaded_file_path) and (time.time() - start_time) < timeout:
                time.sleep(1)

            if os.path.exists(downloaded_file_path):
                print(f"File downloaded to: {downloaded_file_path}")
            else:
                print(f"File not found in download directory after {timeout} seconds.")


        except Exception as e:
            print(f"Error during file download attempt: {e}")


    except Exception as e:
        print(f"An unhandled error occurred during simulation: {e}")

    finally:
        driver.quit()
        print("Browser closed.")
