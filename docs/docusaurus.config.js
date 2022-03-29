// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require("prism-react-renderer/themes/github");
const darkCodeTheme = require("prism-react-renderer/themes/dracula");

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "Auto Traffic Control",
  tagline: "A Video Game for Programmers",
  url: "https://autotrafficcontrol.com",
  baseUrl: "/",
  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",
  favicon: "img/logo.svg",
  organizationName: "jdno", // Usually your GitHub org/user name.
  projectName: "atc", // Usually your repo name.

  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          editUrl: "https://github.com/jdno/atc/tree/main/docs",
        },
        blog: {
          showReadingTime: true,
          editUrl: "https://github.com/jdno/atc/tree/main/docs",
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      navbar: {
        title: "Auto Traffic Control",
        logo: {
          alt: "Auto Traffic Control Logo",
          src: "img/logo.svg",
        },
        items: [
          {
            href: "https://github.com/jdno/atc",
            label: "GitHub",
            position: "right",
          },
        ],
      },
      footer: {
        style: "dark",
        links: [
          {
            title: "Auto Traffic Control",
            items: [
              {
                label: "Documentation",
                to: "/",
              },
              {
                label: "Repository",
                href: "https://github.com/jdno/atc",
              },
            ],
          },
          {
            title: "jdno",
            items: [
              {
                label: "Blog",
                href: "https://jdno.dev",
              },
              {
                label: "GitHub",
                href: "https://github.com/jdno",
              },
              {
                label: "Twitter",
                href: "https://twitter.com/jdno_dev",
              },
              {
                label: "Twitch",
                href: "https://twitch.com/jdno_dev",
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Jan David Nose`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
      },
    }),
};

module.exports = config;
