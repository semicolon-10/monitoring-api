# Monitoring API

This repository contains instructions and resources for setting up Monitoring for Rust applications using Prometheus.

## Getting Started

Follow these steps to set up and run the Monitoring

1. **Clone the Repository**

    ```bash
    git clone git@github.com:semicolon-10/monitoring-api.git
    ```

2. **Docker Compose Setup**

    This repository includes a Docker Compose file to easily spin up Prometheus UI. Navigate to the repository directory and run the following command:

    ```bash
    cd monitoring-api
    docker-compose up -d
    ```

    This will start Prometheus UI, and you can access it via [http://localhost:9091/](http://localhost:9091/), Also Grafana on [http://localhost:3000/](http://localhost:3000/)

3. **Running the Dummy Web Server**

    To spin up the web server execute the following command:

    ```bash
    cargo run
    ```

# Subscribe to my youtube Channel 🎥

[Semicolon](https://www.youtube.com/@Semicolon10)
