
/*

4 types of transfer in USB (page 64)
- Control
- Isochronous
- Interrupt
- Bulk

## Control transfers
Used to access different functions, and control different parts of the device. Supports
configuration, command, status type communication flows between the client software and its function/
Control transfers are used to configure the device at attach time. Lossless data delivery
Each USB device is required to implement the Default Control Pipe, at least

## Isochronous transfers
These type of transfers make use of a prenegotiated delivery latency and bandwidth. These are
real-time transfers

## Interrupt transfers
Used for timely but reliable delivery of data. Might be event notifications for example or
coordinates from a pointing device. They use limited latency transfer

## Bulk transfers
Used to send large bursts of data between device and host, sequential and reliable. Need to use
error detection


## Device endpoints (page 61)
It's a portion of the USB device that is the terminus of the communication flow between the host
and the device. The USB device is composed by a collection of endpoints. Each endpoint is given a
unique endpoint number, but the USB address is assigned to the device itself by the host, and all
the endpoints in a device share that same address. Each endpoint is a simplex connection (supports
data transfer in a single direction, either Host -> Device, or Device -> Host).

An endpoint is described by:
- Bus access frequency/latency requirements
- Bandwidth requirement
- Endpoint number
- Error handling behaviour requirements
- Maximum packet size capable of sending or receiving
- Transfer type for said endpoint (Control, Isochronous, Interrupt or Bulk)
- Direction of the data between endpoint and Host

Endpoints that are not the endpoint zero are in the Unknown state before configuration

 */