
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
Each USB device is required to implement the Default Control Pipe, at least. They are host
software-initiated

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

### Endpoint zero requirements
- Needs to support Control Transfer as a bare minimum
- Needs to be able to be reset at full-speed
- Respond successfully to requests set_address(), set_configuration(), get_descriptor() and respond
the appropriate information


### Non Endpoint zero requirements
Functions (functions seem to be the implementations of a USB device, it's like the collection of
design that makes a USB Device or Host) can have additional endpoints, and low-speed functions can
only support up to 2 additional endpoints, other than the 2 required to implement the Default Control
Pipe.

Full-speed devices can have as many endpoints as needed, only limited by the protocol definition,
which specifies a max of 15 input and 15 output endpoints. Endpoints other than the Default control
Pipe are disabled until configuration is established.


## Pipes
They are sort of the way of moving data between the Host and the Endpoint on the Device. There are 2
mutually exclusive pipe communication modes:
- Stream: data moving through this pipe have no USB-defined structure
- Message: data moving through this pipe has some USB-defined structure

The USB does not interpret the data being moved through the pipe, it's up to the Device and the Host
to make sense of the information.

Pipes can also have the following associated with them:
- A claim on USB access and bandwidth usage
- Transfer type
- The associated endpoint's characteristics, like the directionality and maximum data payload sizes.

There is a pipe that consists on the two endpoint number zero (for both directions, since endpoints
are simplex), and that pipe is called Default Control Pipe. DCP is always available once the device
is powered and received the reset signal. It's not limited to only being used at the beginning of
the USB connection, it can also be used in any other moment during said connection.

The USB system claims ownership of the Default Control Pipe and mediates its usage with software
that attempts to configure the Device.

## Stream Pipes
They are unidirectional, simplex communication transfers. These pipes work as FIFO. It's bound to a
single endpoint of the device, meaning that if an outgoing Stream Pipe bound to Endpoint number 5,
EP5 will be communicating in an outgoing manner.

Stream Pipes support Bulk, Isochronous and Interrupt Transfer Types.

## Message Pipes
These are duplex comm pipes, even though communications might only be predominant in a single
direction. Default Control Pipe is a Message Pipe. Requests to a Message Pipe are sent one at a time.
Multiple software clients might send several requests to the Device through Message Pipes, but the
USB System Software ensures a single request is sent and processed at a time, in a FIFO manner.

Message Pipes require that an associated Endpoint has both the IN and OUT directions linked to it,
for example for Endpoint Zero, there has to be an Endpoint Zero IN and an Endpoint Zero Out set up.
They both have to be linked to the same number, but two different Endpoints, one for each direction.

Message Pipes support the Control Transfer Type.


## Frames and Microframes
A frame is a unit of time defined by the USB protocol that varies depending on the USB mode. For full
and low speed, this Frame is 1ms (1KHz) and for High Speed, the Frame is called Microframe and is
125us (8KHz).

A Microframe can contain several transactions, and the amount and type of transactions supported is defined
by the type of Transfer .

LEFT IN PAGE 72


 */