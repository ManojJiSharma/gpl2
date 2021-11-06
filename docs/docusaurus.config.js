module.exports = {
  title: "Gemachain Program Library Docs",
  tagline:
    "Gemachain is an open source project implementing a new, high-performance, permissionless blockchain.",
  url: "https://gpl.gemachain.com",
  baseUrl: "/",
  favicon: "img/favicon.ico",
  organizationName: "gemachain-labs", // Usually your GitHub org/user name.
  projectName: "gemachain-program-library", // Usually your repo name.
  themeConfig: {
    navbar: {
      logo: {
        alt: "Gemachain Logo",
        src: "img/logo-horizontal.svg",
        srcDark: "img/logo-horizontal-dark.svg",
      },
      links: [
        {
          href: "https://docs.gemachain.com/",
          label: "Docs »",
          position: "left",
        },
        {
          href: "https://discordapp.com/invite/pquxPsq",
          label: "Chat",
          position: "right",
        },

        {
          href: "https://github.com/gemacoin/gemachain.git",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Community",
          items: [
            {
              label: "Discord",
              href: "https://discordapp.com/invite/pquxPsq",
            },
            {
              label: "Twitter",
              href: "https://twitter.com/gemachain",
            },
            {
              label: "Forums",
              href: "https://forums.gemachain.com",
            },
          ],
        },
        {
          title: "More",
          items: [
            {
              label: "GitHub",
              href: "https://github.com/gemacoin/gemachain.git",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Gemachain Foundation`,
    },
  },
  plugins: [require.resolve('docusaurus-lunr-search')],
  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          path: "src",
          routeBasePath: "/",
          homePageId: "introduction",
          sidebarPath: require.resolve("./sidebars.js"),
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      },
    ],
  ],
};
