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
  trailingSlash: false,

  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          editUrl:
            "https://github.com/jdno/auto-traffic-control/tree/main/docs",
        },
        blog: {
          showReadingTime: true,
          editUrl:
            "https://github.com/jdno/auto-traffic-control/tree/main/docs",
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
            type: "doc",
            docId: "getting-started",
            position: "left",
            label: "Docs",
          },
          {
            type: "doc",
            docId: "api/introduction",
            position: "left",
            label: "API",
          },

          {
            href: "https://jdno.itch.io/auto-traffic-control",
            label: "itch.io",
            position: "right",
          },
          {
            href: "https://github.com/jdno/auto-traffic-control",
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
                label: "Download",
                href: "https://jdno.itch.io/auto-traffic-control",
              },
              {
                label: "Repository",
                href: "https://github.com/jdno/auto-traffic-control",
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
        additionalLanguages: ["protobuf", "rust"],
      },
    }),
};

module.exports = config;
