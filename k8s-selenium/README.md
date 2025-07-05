# Selenium for Kubernetes

This directory contains Kustomize configurations for deploying Selenium in a Kubernetes cluster. The setup includes a standalone Selenium Chrome instance that can be used for web scraping and browser automation.

## Features

- Standalone Selenium Chrome server
- Exposed WebDriver API (port 4444) for CDP (Chrome DevTools Protocol) requests
- Exposed VNC interface (port 7900) for visual debugging
- Sample Python script for scraping using seleniumbase CDP

## Directory Structure

```
k8s-selenium/
├── base/                      # Base Kustomize configuration
│   ├── kustomization.yaml     # Base kustomization file
│   ├── selenium-deployment.yaml  # Selenium deployment
│   └── selenium-service.yaml  # Selenium service (ClusterIP)
├── overlays/                  # Kustomize overlays
│   └── local/                 # Local development overlay
│       ├── kustomization.yaml # Local kustomization file
│       └── service-patch.yaml # Patch to change service to NodePort
└── sample_scraper.py          # Sample Python script for scraping
```

## Deployment

### Prerequisites

- Kubernetes cluster (e.g., k3s, minikube, or a cloud provider)
- kubectl installed and configured
- kustomize installed (or use kubectl with built-in kustomize)

### Deploying to a Local Cluster

To deploy Selenium to your local Kubernetes cluster:

```bash
kubectl apply -k k8s-selenium/overlays/local
```

This will create:
- A deployment running the Selenium Chrome container
- A NodePort service exposing:
  - WebDriver API on port 30444 (internal port 4444)
  - VNC interface on port 30900 (internal port 7900)

### Accessing Selenium

- WebDriver API: `http://<node-ip>:30444/wd/hub`
- VNC interface: `http://<node-ip>:30900` (password: `secret`)

Replace `<node-ip>` with the IP address of your Kubernetes node.

## Using the Sample Scraper

The `sample_scraper.py` script demonstrates how to use seleniumbase with CDP to connect to the Selenium service and perform web scraping.

### Prerequisites

```bash
pip install seleniumbase
```

### Running the Sample

```bash
python k8s-selenium/sample_scraper.py --selenium-url http://<node-ip>:30444/wd/hub --url https://example.com
```

## Customization

You can customize the deployment by:

1. Modifying the base configuration in `k8s-selenium/base/`
2. Creating new overlays for different environments in `k8s-selenium/overlays/`
3. Adjusting resource limits in `selenium-deployment.yaml`