# sofar-mqtt

![Rust Version](https://img.shields.io/badge/rust-1.69+-orange.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ceski23/sofar-mqtt/blob/main/LICENSE)

**sofar-mqtt** is a Rust-based project that enables the reception and parsing of data frames from the Sofar photovoltaic (PV) inverter and facilitates sending them to an MQTT broker. This lightweight and efficient tool aims to simplify the integration of Sofar inverters with MQTT-based home automation systems like Home Assistant, energy monitoring platforms, or any other applications requiring real-time PV data.

## Features

- Connects to a Sofar PV inverter via TCP connection to dedicated data logger (LSW-3),
- Receives data frames emitted by the inverter in a specific protocol format,
- Parses the received frames to extract relevant data such as current power output, voltage, and other parameters,
- Sends the parsed data to an MQTT broker for further processing or integration with other systems,
- Implements error handling and logging to ensure reliable operation.

## Installation

1. Ensure you have Rust and Cargo installed. If not, follow the instructions provided at the [Rust website](https://www.rust-lang.org/tools/install).

2. Clone the **sofar-mqtt** repository:

   ```shell
   git clone https://github.com/ceski23/sofar-mqtt.git
   ```

3. Change to the project directory:

   ```shell
   cd sofar-mqtt
   ```

4. Build the project using Cargo:

   ```shell
   cargo build --release
   ```

## Usage

To run **sofar-mqtt**, use the following command:

```shell
cargo run --release
```

Alternatively run built executable directly:

```shell
./target/[TARGET_ARCHITECTURE]/release/sofar-mqtt
```

## Configuration

**sofar-mqtt** can be configured using environmental variables. The available configuration options include:

- `MQTT_HOST`: Specify the MQTT broker's address to which the parsed data will be sent (Default: `localhost`)
- `MQTT_PORT`: Specify the MQTT broker's port to which the parsed data will be sent (Default: `1883`)
- `MQTT_USER`: Specify the username used when connecting to MQTT
- `MQTT_PASSWORD`: Specify the username used when connecting to MQTT
- `TCP_PORT`: Specify the TCP port used to connect to the Sofar Data Logger (Default: `8080`)

To set these environmental variables, you can either export them in your shell environment, write them in `.env` file or specify them when running the Docker container.

## Using the Docker Image

Alternatively, you can use the provided Docker image to run **sofar-mqtt** without having to install Rust and its dependencies manually. The Docker image ensures a consistent and isolated environment for running the application.

> **Warning**
> For now, only image for ARM64 architecture is provided

To use the Docker image, follow these steps:

1. Pull the Docker image from the repository:

   ```shell
   docker pull ghcr.io/ceski23/sofar-mqtt:master
   ```

2. Run the Docker container with the necessary configuration options:

   ```shell
   docker run --name sofar-mqtt -d -p 3000:8080 ghcr.io/ceski23/sofar-mqtt:master
   ```

> **Note**
> For more information on available options and configuration, please refer to the [configuration](#configuration) section.

## Building the Docker Image

If you prefer to build the Docker image locally, follow these steps:

1. Clone the **sofar-mqtt** repository:

   ```shell
   git clone https://github.com/ceski23/sofar-mqtt.git
   ```

2. Change to the project directory:

   ```shell
   cd sofar-mqtt
   ```

3. Build the Docker image using the provided Dockerfile:

   ```shell
   docker build -t sofar-mqtt .
   ```

4. Proceed to use the Docker image as described in the previous section.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgments

- decoded Sofar data frames courtesy of [serek4/node-red-sofar-inverter](https://github.com/serek4/node-red-sofar-inverter)