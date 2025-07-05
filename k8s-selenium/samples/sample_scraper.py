#!/usr/bin/env python
"""
Sample web scraper using SeleniumBase in CDP mode.
This script demonstrates how to connect to a remote Selenium server
and perform web scraping operations.
"""

from seleniumbase import Driver
import time
import json
import os

# Configuration
SELENIUM_URL = os.environ.get("SELENIUM_URL", "http://localhost:30444/wd/hub")

def scrape_website():
    """
    Sample function to scrape a website using SeleniumBase in CDP mode.
    """
    print("Starting web scraping with SeleniumBase in CDP mode")
    
    # Initialize the driver with CDP mode enabled
    driver = Driver(
        uc=True,  # Use undetected-chromedriver
        cdp=True,  # Enable CDP mode
        remote_address=SELENIUM_URL,
        headless=False,
    )
    
    try:
        # Navigate to a website
        driver.get("https://news.ycombinator.com/")
        print(f"Page title: {driver.title}")
        
        # Wait for the page to load
        time.sleep(2)
        
        # Extract news items
        news_items = []
        elements = driver.find_elements("css selector", ".titleline > a")
        scores = driver.find_elements("css selector", ".score")
        
        for i, (element, score) in enumerate(zip(elements, scores)):
            if i >= 10:  # Limit to 10 items
                break
                
            title = element.text
            url = element.get_attribute("href")
            score_text = score.text if i < len(scores) else "0 points"
            
            news_items.append({
                "title": title,
                "url": url,
                "score": score_text
            })
        
        # Print the results
        print(json.dumps(news_items, indent=2))
        
        # Take a screenshot
        driver.save_screenshot("hacker_news.png")
        print("Screenshot saved as hacker_news.png")
        
        return news_items
        
    finally:
        # Close the browser
        driver.quit()

if __name__ == "__main__":
    scrape_website()