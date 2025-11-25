/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  darkMode: "class",
  theme: {
    colors: {
      // Dark theme
      base: "#08222d",
      main: "#E7BF00",
      accent: "#4f5861",
      "dark-error": "#C9453E",
      // CLI colors / terminal colors
      "cli-black": "002221",
      "cli-red": "#EA3431",
      "cli-green": "#00B6B6",
      "cli-yellow": "#E7BF00",
      "cli-blue": "#4894FD",
      "cli-magenta": "#E01DCA",
      "cli-cyan": "#1AB2AD",
      "cli-white": "#FFFFFF",
      // Text colors
      white: "#EEEEEE",
      "white-dim": "#FFFFFF80",
    },
    extend: {
      fontFamily: {
        sans: ["JetBrains Mono", "monospace"],
      },
      typography: {
        DEFAULT: {
          css: {
            code: {
              backgroundColor: "#203c48",
              fontWeight: "400",
              padding: "0.25rem 0.25rem 0 0.25rem",
              "border-radius": "0.1rem",
            },
            "code::before": {
              content: '""',
            },
            "code::after": {
              content: '""',
            },
          },
        },
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
