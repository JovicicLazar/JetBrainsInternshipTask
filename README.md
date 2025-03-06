# JetBrainsInternshipTask
Creation of a client that will handle propper data transfer from a buggy server

# Task description
Here is a glitchy HTTP server written in Python. The server sends some randomized data on every GET request, but often it doesn’t send the whole data, but just a part of the data. Luckily, the server supports the HTTP header “Range”.

Run the server in a terminal.

You need to write a client application that downloads the binary data from the glitchy server. To ensure that the downloaded data is correct, check the SHA-256 hash of the downloaded data. The hash must be the same as the HTTP server writes into the terminal.

Write the client app either in Kotlin+JVM or in Rust.

If possible, avoid using external libraries.
