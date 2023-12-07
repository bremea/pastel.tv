**Note:** Pastel is still under development, so the production site may go down and source code is subject to change at any time.

This repository contains the entire source code for [pastel.tv](https://pastel.tv). The production website is run on Vercel, with the backend being hosted on Google Cloud and the database in Planetscale. However, the code is platform agnostic, and should be able to run anywhere.

# About
pastel.tv is a online service that provides access to an instance of Android TV running in the cloud. This cloud TV can be easily moved between servers, allowing it to run in any region supported by the cloud provider. It exposes a WebRTC stream, allowing users to view and share content live.

# Tech
pastel.tv is built with the following tech:
- Rust (all backend services)
- Typescript & SvelteKit (website client)
- MySQL (database)
- WebRTC (streaming)

In production, pastel is run with the following services:
- Google Cloud (backend hosting, CDN)
- Planetscale (database)
- Vercel (website frontend)
- Resend (automated email)

# Why open source?
This is a fun project I decided to make so my friends and I can watch content together, even when continents apart. I have no reason to keep it proprietary, and making pastel open source is a nice way to pad out my GitHub as I apply for jobs ([which I am actively looking for](https://bremea.com)).

Please keep in mind that since this is a hobby project I work on in my free time, there is little in the way of documentation. If you are interested in running pastel yourself, it's mostly up to you to figure out any kinks in the system that might bubble up. If you have questions, feel free to shoot me a message on Discord or [email](mailto:jade@bremea.com)

# License
pastel.tv is free and open-source software, licensed under the GNU Affero General Public License, version 3 (AGPLv3). The full license is in the `LICENSE` file, in the root of the repository. Please note that the AGPLv3 stipulates that any modifications must be under the same license and have their source code available, even when distributing across a network (like a SaaS). For questions about licensing/copyright please [send an email](mailto:jade@bremea.com). Copyright notice is below.

```
pastel.tv
Copyright (C) 2023  Jade Meadows

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
