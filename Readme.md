# Owlnigma

This project is a client-server application written in Rust that implements encryption and decryption of transmitted data using the AES-GCM algorithm. The application allows the client to send requests to the server, and the server responds with encrypted messages. The project uses the tokio library for asynchronous task execution and aes-gcm for encryption.

## Features
 - Data Encryption: All data between the client and server is encrypted using the AES-GCM algorithm with a 256-bit key.
 - Asynchronous Execution: The project uses the tokio library, allowing it to handle multiple connections simultaneously.
 - Command Handling: The server supports client commands and responds accordingly, implemented using an enum.
- Secure Key Handling: The encryption key is securely stored in a constant and passed as an argument to ensure data security during transmission.

## Installation

1. Make sure you have Rust installed.
2. Clone the repository.
   ```bash
   git clone 

   ```
3. Navigate to the project dirictory:

   ```bash
   cd owlnigma
   ```

4. cargo build 