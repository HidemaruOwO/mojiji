// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import starlightThemeRapide from "starlight-theme-rapide";

// https://astro.build/config
export default defineConfig({
  integrations: [
    starlight({
      plugins: [starlightThemeRapide()],
      title: "Mojiji Docs",
      social: {
        github: "https://github.com/HidemaruOwO/mojiji",
      },
      sidebar: [
        {
          label: "Guides",
          items: [
            // Each item here is one entry in the navigation menu.
            { label: "Quick Start", slug: "guides/usage" },
            { label: "Build from Source Code", slug: "guides/build" },
            { label: "Deploy to your server", slug: "guides/deploy" },
          ],
        },
        {
          label: "Reference",
          autogenerate: { directory: "reference" },
        },
      ],
    }),
  ],
});
