# OpenResolvium - `res/vnc.json` Documentation

This document provides an overview of the `res/vnc.json` file used in the OpenResolvium project.

## Purpose of `res/vnc.json`

The `res/vnc.json` file serves as a configuration and data storage file for the project. It contains information about VNC servers, including their IP addresses, connection statuses, and any associated screenshots.

## Structure of `vnc.json`

The file is structured as a JSON object with the following format:

- **`list`**: A container for all VNC server entries.
    - **`vnc1`, `vnc2`, ...**: Unique identifiers for each VNC server.
        - **`ip`**: The IP address of the VNC server.
        - **`status`**: The connection status of the server (e.g., `Down`, `Up`).
        - **`img`**: A placeholder for a screenshot or image associated with the server (e.g., `null` or a file path).
        - **`password`**: The password of the VNC server.

### Example `vnc.json`

Below is a sample `vnc.json` file:

```json
{
    "list": {
        "vnc1": {
            "ip": "Test IP",
            "status": null,
            "img": null,
            "password": "abc123"
        },
        "vnc2": {
            "ip": "Test IP 2",
            "status": null,
            "img": null,
            "password": null
        }
    }
}
```

## Usage

1. **Adding VNC Servers**: Add new entries under the `list` object with unique keys (e.g., `vnc3`, `vnc4`).
2. **Updating Status**: Modify the `status` field to reflect the connection state of the server.
3. **Adding Screenshots**: Update the `img` field with the path to a screenshot file if available.