#!/usr/bin/env python3
"""
Sample script for scraping using seleniumbase CDP locally.
This script demonstrates how to use seleniumbase for local browser automation
and perform basic scraping operations using CDP (Chrome DevTools Protocol).
"""

from seleniumbase import Driver
import time
import requests
import argparse

def parse_arguments():
    parser = argparse.ArgumentParser(description='Sample Selenium CDP scraper')
    parser.add_argument('--url', type=str, default='https://www.reteleelectrice.ro/intreruperi/programate/',
                        help='URL to scrape')
    return parser.parse_args()

def main():
    args = parse_arguments()

    print("Starting local seleniumbase browser")
    print(f"Scraping URL: {args.url}")

    # Initialize local seleniumbase driver
    driver = Driver(
        uc=True  # Use undetected-chromedriver mode
    )

    try:
        # Navigate to the target URL
        driver.get(args.url)
        print(f"Page title: {driver.title}")

        # Wait for the page to load
        time.sleep(2)

        # Find the RSS link using the provided XPath
        xpath = "//*[@id=\"page-wrap\"]/div/div/div/div/div[4]/div/a"
        try:
            rss_link_element = driver.find_element("xpath", xpath)

            if rss_link_element:
                rss_url = rss_link_element.get_attribute('href')
                print(f"Found RSS link: {rss_url}")

                # Use requests to get the RSS content directly
                # This avoids issues with the browser trying to render XML
                print("Fetching RSS content using requests...")

                # Add headers to mimic a browser request
                headers = {
                    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36',
                    'Referer': 'https://www.reteleelectrice.ro/intreruperi/programate/',
                    'Accept': 'application/rss+xml, application/xml, text/xml, */*'
                }

                response = requests.get(rss_url, headers=headers)
                if response.status_code == 200:
                    print("\n--- RSS Content ---")
                    print(response.text)
                    print("--- End of RSS Content ---")
                else:
                    print(f"Failed to fetch RSS content: {response.status_code}")
                    print("Attempting to use the browser's cookies for authentication...")

                    # Get cookies from the browser
                    cookies = driver.get_cookies()
                    cookies_dict = {cookie['name']: cookie['value'] for cookie in cookies}

                    # Try again with cookies
                    response = requests.get(rss_url, headers=headers, cookies=cookies_dict)
                    if response.status_code == 200:
                        print("\n--- RSS Content (with cookies) ---")
                        print(response.text)
                        print("--- End of RSS Content ---")
                    else:
                        print(f"Still failed to fetch RSS content: {response.status_code}")
            else:
                print("RSS link not found using the provided XPath")
        except Exception as e:
            print(f"Error finding or processing RSS link: {e}")

    finally:
        # Close the browser
        driver.quit()
        print("Browser closed")

if __name__ == "__main__":
    main()
