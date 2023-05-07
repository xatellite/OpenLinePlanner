<div id="top"></div>
<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/othneildrew/Best-README-Template">
    <img src="doc/logo.svg" alt="Logo" width="80" height="80">
  </a>

  <h2 align="center">OpenLinePlanner</h3>

  <p align="center">
    Fast and Easy public transport network prototyping
    <br />
    <a href="https://openlineplanner.xatellite.space/"><strong>Check out the Demo »</strong></a>
    <br />
    <br />
    <a href="https://github.com/TheNewCivilian/OpenLinePlanner/issues">Report Bug</a>
    ·
    <a href="https://github.com/TheNewCivilian/OpenLinePlanner/issues">Request Feature</a>
    ·
    <a href="mailto:hey@xatellite.space?subject=%5BOpenlineplanner%5D">Send Feedback</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#usage">Usage</a>
      <ul>
        <li><a href="#calculation-methods">Calculation Methods</a></li>
      </ul>
    </li>
    <li>
      <a href="#setting-up-developing-environment">Setting up Developing Environment</a>
      <ul>
        <li><a href="#backend-setup">Backend Setup</a></li>
        <li><a href="#frontend-setup">Frontend Setup</a></li>
      </ul>
    </li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

![OpenLinePlanner Screen Shot][product-screenshot]

Its main purpose is to prototype public transport lines on the open field, analyse the station coverage and find the optimal station position.

The tool allows to save configurations fast and export all results to a printable pdf.

It is build to be extendable and easy to adjust, to be applicable in different scenarios.

<p align="right">(<a href="#top">back to top</a>)</p>

### Built With

This project was build with:

[![Vue][Vue.js]][Vue-url]
[![Vite][Vite]][Vite-url]
[![Rust][Rust]][Rust-url]
[![Matomo][Matomo]][Matomo-url]

And deployed with:

[![Docker][Docker]][Docker-url]

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

## Usage

OpenLinePlanner allows you to

... draw **schematic transportation** lines on map <br>
... name lines and stations <br>
... give a custom color to each line <br>
... **analyse** coverage area of stations <br>
... **export** map and analysis as PDF <br>
... automatically **locate new stations**, based on predicted demand <br>

### Calculation Methods

By default OpenLinePlanner is configured to show the **absolute** amount of residences living in the coverage area of each station.
Every person in the influence radius of the station (default 500m) is rated with the weight of 1.
i.e. if 200 people life in the influence are the result is a score of 200

Alternatively a relative measurement can be applied (Settings). This approach takes the distance between station and the home of a potential passenger into account:

```js
// Calculation methods:
absolute: 1;
relative: 1 / sqrt(distance);
```

If your scenario requires adjusted parameters please reach out! ([Contact via Email](mailto:openlineplanner@xatellite.space))
Further adjustment dialogs are planned to be implemented if requested.

<p align="right">(<a href="#top">back to top</a>)</p>

## Screenshots

![OpenLinePlanner coverage area visualization][station-info-screenshot]
![OpenLinePlanner coverage area visualization][coverage-screenshot]
![OpenLinePlanner coverage area visualization][coverage-station-screenshot]
![OpenLinePlanner coverage area visualization][data-screenshot]
![OpenLinePlanner coverage area visualization][timetable-screenshot]

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Setting up Developing Environment

This is a short guide to setup your own development environment of OpenLinePlanner.

1. To start of clone the Repo:
   ```sh
   $ git clone https://github.com/TheNewCivilian/OpenLinePlanner.git
   ```

### Backend Setup

The backend can be build running

2. Build backend

   ```sh
   $ cd openlineplanner-backend
   $ cargo build --release
   ```

3. Gather data files from [OpenPopulationEstimator](https://github.com/TheNewCivilian/OpenPopulationEstimator) (inhabitants geojson) and e.g. [Protomaps](https://app.protomaps.com/downloads/osm) (pbf file of region)

4. Add ./settings/Settings.toml e.g.

   ```toml
    [data]
    residence = "./data/residence.geojson"
    osm = "./data/Wien_Donaustadt.osm.pbf"
   ```

5. Install backend binaries
   ```sh
   $ cargo install --path .
   ```
6. Startup Backend (Takes some minutes on first startup)
   ```sh
   $ openlineplanner-backend
   ```

### Frontend Setup

All environment variables are automatically loaded by VITE from the available .env files. If you need to adjust ports or endpoints check these files.

4. Install all NPM packages
   ```sh
   $ yarn
   ```
5. Start development server
   ```js
   $ yarn dev
   ```

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make OpenLinePlanner better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
If this project brings you any value or inspires you don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- LICENSE -->

## License

Distributed under the GNU GPL V3 License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Email: [openlineplanner@xatellite.space](mailto:openlineplanner@xatellite.space)

Project Link: [https://github.com/TheNewCivilian/OpenLinePlanner](https://github.com/TheNewCivilian/OpenLinePlanner)

<p align="right">(<a href="#top">back to top</a>)</p>

## Contributors

[TheNewCivilian](https://github.com/TheNewCivilian), [zdmx](https://github.com/zandemax)

<!-- ACKNOWLEDGMENTS -->

## Acknowledgments

The following pages and resources have been very helpful in the creation of the project:

- [README Template - @OthneilDrew](https://github.com/othneildrew/Best-README-Template)
- [Choose an Open Source License - @ChooseaLicense](https://choosealicense.com)
- [Img Shields - @ShieldIO](https://shields.io)

This project was created as part of the interdisciplinary project of the master class Rail Technology and Management of Railway Systems @FH-St.Pölten.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/TheNewCivilian/OpenLinePlanner.svg?style=for-the-badge
[contributors-url]: https://github.com/TheNewCivilian/OpenLinePlanner/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/TheNewCivilian/OpenLinePlanner.svg?style=for-the-badge
[forks-url]: https://github.com/TheNewCivilian/OpenLinePlanner/network/members
[stars-shield]: https://img.shields.io/github/stars/TheNewCivilian/OpenLinePlanner.svg?style=for-the-badge
[stars-url]: https://github.com/TheNewCivilian/OpenLinePlanner/stargazers
[issues-shield]: https://img.shields.io/github/issues/TheNewCivilian/OpenLinePlanner.svg?style=for-the-badge
[issues-url]: https://github.com/TheNewCivilian/OpenLinePlanner/issues
[license-shield]: https://img.shields.io/github/license/TheNewCivilian/OpenLinePlanner.svg?style=for-the-badge
[license-url]: https://github.com/TheNewCivilian/OpenLinePlanner/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[product-screenshot]: ./doc/images/plain.png
[coverage-screenshot]: ./doc/images/coverage.png
[station-info-screenshot]: ./doc/images/station_info.png
[coverage-station-screenshot]: ./doc/images/coverage_station.png
[data-screenshot]: ./doc/images/data.png
[timetable-screenshot]: ./doc/images/timetable.png
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[Vite]: https://img.shields.io/badge/Vite-35495E?style=for-the-badge&logo=Vite&logoColor=646CFF
[Vite-url]: https://vitejs.dev/
[Docker]: https://img.shields.io/badge/Docker-35495E?style=for-the-badge&logo=Docker&logoColor=2496ED
[Docker-url]: https://www.docker.com/
[Rust]: https://img.shields.io/badge/Rust-35495E?style=for-the-badge&logo=Rust&logoColor=000000
[Rust-url]: https://www.rust-lang.org/
[Matomo]: https://img.shields.io/badge/Matomo-35495E?style=for-the-badge&logo=Matomo&logoColor=3152A0
[Matomo-url]: https://matomo.org/
