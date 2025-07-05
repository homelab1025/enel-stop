# Selenium Grid for Kubernetes

This directory contains Kustomize configurations for deploying Selenium Grid in a Kubernetes cluster. The setup includes a Selenium Hub and Chrome Node that can be used for web scraping, browser automation, and parallel test execution.

## Features

- Selenium Grid architecture with Hub and Node components
- Chrome Node with multiple browser sessions support
- Exposed Hub WebDriver API (port 4444) for test distribution
- Exposed Node VNC interface (port 7900) for visual debugging
- Sample Python script for scraping using seleniumbase CDP

## Directory Structure

```
k8s-selenium/
├── base/                          # Base Kustomize configuration
│   ├── kustomization.yaml         # Base kustomization file
│   ├── selenium-hub-deployment.yaml  # Selenium Hub deployment
│   ├── selenium-hub-service.yaml  # Selenium Hub service (ClusterIP)
│   ├── selenium-deployment.yaml   # Chrome Node deployment
│   ├── selenium-service.yaml      # Chrome Node service (ClusterIP)
│   └── namespace.yaml             # Selenium namespace
├── overlays/                      # Kustomize overlays
│   └── local/                     # Local development overlay
│       ├── kustomization.yaml     # Local kustomization file
│       └── service-patch.yaml     # Patch to change services to NodePort
└── sample_scraper.py              # Sample Python script for scraping
```

## Deployment

### Prerequisites

- Kubernetes cluster (e.g., k3s, minikube, or a cloud provider)
- kubectl installed and configured
- kustomize installed (or use kubectl with built-in kustomize)

### Deploying to a Local Cluster

To deploy Selenium Grid to your local Kubernetes cluster:

```bash
kubectl apply -k k8s-selenium/overlays/local
```

This will create:
- A Selenium Hub deployment that manages test distribution
- A Chrome Node deployment that runs browser instances
- NodePort services exposing:
  - Hub WebDriver API on port 30444 (internal port 4444)
  - Hub HTTPS on port 30443 (internal port 4443)
  - Hub Publish on port 30442 (internal port 4442)
  - Node port on port 30555 (internal port 5555)
  - Node VNC interface on port 30900 (internal port 7900)

### Accessing Selenium Grid

- Hub WebDriver API: `http://<node-ip>:30444/wd/hub` (for test execution)
- Hub Grid Console: `http://<node-ip>:30444/ui` (for Grid management)
- Node VNC interface: `http://<node-ip>:30900` (password: `secret`, for visual debugging)

Replace `<node-ip>` with the IP address of your Kubernetes node.

## Using the Sample Scraper

The `sample_scraper.py` script demonstrates how to use seleniumbase with CDP to connect to the Selenium Grid Hub and perform web scraping.

### Prerequisites

```bash
pip install seleniumbase
```

### Running the Sample

```bash
python k8s-selenium/sample_scraper.py --selenium-url http://<node-ip>:30444/wd/hub --url https://example.com
```

The script connects to the Selenium Grid Hub, which automatically distributes the test to an available Chrome Node.

## Customization

You can customize the deployment by:

1. Modifying the base configuration in `k8s-selenium/base/`
2. Creating new overlays for different environments in `k8s-selenium/overlays/`
3. Adjusting resource limits in the deployment files
4. Adding more nodes or different browser types (e.g., Firefox, Edge)

### Adding More Chrome Nodes

To scale the number of Chrome Nodes, you can modify the `replicas` field in `selenium-deployment.yaml`:

```yaml
spec:
  replicas: 3  # Change from 1 to desired number
```

### Adding Firefox Nodes

To add Firefox Nodes to your Grid, create a new deployment file similar to the Chrome Node deployment, but using the `selenium/node-firefox:latest` image.
