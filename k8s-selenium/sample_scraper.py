#!/usr/bin/env python3
"""
Sample script for scraping using seleniumbase CDP with the Selenium service.
This script demonstrates how to connect to the Selenium service running in Kubernetes
and perform basic scraping operations using CDP (Chrome DevTools Protocol).
"""

from seleniumbase import Driver
import time
import json
import argparse

def parse_arguments():
    parser = argparse.ArgumentParser(description='Sample Selenium CDP scraper')
    parser.add_argument('--selenium-url', type=str, default='http://192.168.23.107:30661/wd/hub',
                        help='URL of the Selenium service')
    parser.add_argument('--url', type=str, default='https://www.reteleelectrice.ro/intreruperi/programate/',
                        help='URL to scrape')
    return parser.parse_args()

def main():
    args = parse_arguments()

    print(f"Connecting to Selenium at {args.selenium_url}")
    print(f"Scraping URL: {args.url}")

    # Connect to the Selenium service
    driver = Driver(
        uc=True,  # Use undetected-chromedriver mode
        host=args.selenium_url
    )

    try:
        # Navigate to the target URL
        driver.get(args.url)
        print(f"Page title: {driver.title}")

        # Wait for the page to load
        time.sleep(2)

        # Example: Use CDP to get all cookies
        cookies = driver.execute_cdp_cmd('Network.getAllCookies', {})
        print(f"Found {len(cookies['cookies'])} cookies")

        # Example: Use CDP to extract performance metrics
        performance_metrics = driver.execute_cdp_cmd('Performance.getMetrics', {})
        print("Performance metrics:")
        for metric in performance_metrics['metrics']:
            print(f"  {metric['name']}: {metric['value']}")

        # Example: Extract all links from the page
        links = driver.find_elements("tag name", "a")
        print(f"Found {len(links)} links on the page:")
        for i, link in enumerate(links[:5]):  # Print first 5 links
            print(f"  {i+1}. {link.text} -> {link.get_attribute('href')}")

        # Example: Take a screenshot
        driver.save_screenshot("screenshot.png")
        print("Screenshot saved as screenshot.png")

        # Example: Extract HTML content
        html_content = driver.execute_cdp_cmd('DOM.getOuterHTML', {'nodeId': 1})
        print(f"HTML content length: {len(html_content['outerHTML'])} characters")

    finally:
        # Close the browser
        driver.quit()
        print("Browser closed")

if __name__ == "__main__":
    main()
