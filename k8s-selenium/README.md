# SeleniumBase Kubernetes Setup

This directory contains Kustomize scripts for running SeleniumBase in a Kubernetes cluster. The setup includes:

- SeleniumBase running in CDP mode
- External access to Selenium port (4444) and VNC web interface (7900)
- Sample scraping script

## Directory Structure

```
k8s-selenium/
├── base/                  # Base Kustomize configuration
│   ├── deployment.yaml    # SeleniumBase deployment
│   ├── kustomization.yaml # Base kustomization
│   ├── namespace.yaml     # Selenium namespace
│   └── service.yaml       # Service definition
├── overlays/              # Environment-specific overlays
│   └── local/             # Local deployment overlay
│       ├── kustomization.yaml # Local kustomization
│       └── service-patch.yaml # Service patch for NodePort
└── samples/               # Sample scripts
    └── sample_scraper.py  # Sample scraping script
```

## Deployment

To deploy the SeleniumBase setup to your Kubernetes cluster:

```bash
# Apply the local overlay
kubectl apply -k k8s-selenium/overlays/local
```

## Accessing SeleniumBase

After deployment, you can access:

- Selenium server at `http://<node-ip>:30444`
- VNC web interface at `http://<node-ip>:30900`

Replace `<node-ip>` with the IP address of any node in your Kubernetes cluster.

## Running the Sample Scraper

The sample scraper demonstrates how to use SeleniumBase in CDP mode to scrape web content:

```bash
# Set the Selenium URL environment variable (if different from default)
export SELENIUM_URL="http://<node-ip>:30444/wd/hub"

# Run the sample scraper
python k8s-selenium/samples/sample_scraper.py
```

## SeleniumBase CDP Mode

This setup uses SeleniumBase in CDP (Chrome DevTools Protocol) mode, which provides:

- Better browser automation capabilities
- Improved handling of modern web applications
- Enhanced scraping capabilities

The CDP mode is enabled in the deployment configuration with the environment variable `SE_CDP_MODE=true`.

## Customization

You can customize the setup by:

1. Modifying resource limits in `base/deployment.yaml`
2. Changing port mappings in `overlays/local/service-patch.yaml`
3. Adding additional configuration options to the deployment