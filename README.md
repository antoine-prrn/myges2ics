# MyGES 2 ICS

MyGES2ICS is a Rust project that allows you to subscribe to your schedule using a webcal link. You can subscribe to your schedule from your native iOS, MacOS, Android, or other calendar apps.

To achieve this, you need to subscribe to the server's link using ***basic-auth*** if your calendar app supports it. If your calendar app does not support basic-auth, you can provide your credentials using query parameters.

***
## How to subscribe

You can subscribe to your calendar using the server **https://myges2ics.aprrn.fr** that I provide or host your own one like on the instructions below.

**Use your own server, there is a lot of cheap VPS for students, the server that I provide can be shut down.**
#### Using a calendar app that supports basic-auth
[ICSx5](https://f-droid.org/fr/packages/at.bitfire.icsdroid/) on android supports basic-auth. Then your calendar is available any app like Google Calendar or [Simple Calendar Pro](https://f-droid.org/fr/packages/com.simplemobiletools.calendar.pro/).

To subscribe using apps that support basic authentication, use the following URL: ```https://myges2ics.aprrn.fr``` and provide your user/password of myges.fr.

#### Using a calendar app that does not support basic-auth
Most calendar apps, such as the native ones on iOS, macOS, or Google Calendar, do not support basic authentication. To subscribe in such cases, you can include your username and password in the query parameters of the URL like this:

```https://myges2ics.aprrn.fr/?user=USER&password=PASSWORD```

Please make sure to replace "USER" and "PASSWORD" with your actual myges.fr credentials.


***
## How to run

You can run this project with docker or by launching the binary.
**If you want to host it I recommend to host it behind a reverse proxy like *Caddy* to have use HTTPS**

**Docker :**

A sample ```docker-compose.yml``` is provided in the root folder. To launch the app with Docker, follow these steps:
1. Clone the repository
2. Build and launch with the following commands:
```bash
docker compose build && docker compose up -d
```

Your application is now accessible on the exported port in the ```docker-compose.yml``` file wich is by default ```8080```.

**Build and launch the binary**

To build the project you need to install the [rust toolchain](https://rustup.rs/). After installing Rust, clone the repository and execute the following commands:
1. Build the project:
```bash
cargo build --release
```

2. Launch it

```bash
./target/release/myges2ics
```
If you want to edit the default config values you can copy the ```.env.example``` file:
```bash
cp .env.example .env
```
Then you can edit the values to suit your needs