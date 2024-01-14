/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  darkMode: "class",
  theme: {
    colors: {
      light: "#FAF9FA",
      "light-accent": "#8AB0B8",
      main: "#E7BF00",
      dark: "#4f5861", //"#282835",
      "dark-accent": "#C9453E",
      white: "#EEEEEE",
      "white-dim": "#FFFFFF80",
      black: "#08222d", //"#080808", blue #08222d
      "cli-black": "002221",
      "cli-red": "#EA3431",
      "cli-green": "#00B6B6",
      "cli-yellow": "#E7BF00",
      "cli-blue": "#4894FD",
      "cli-magenta": "#E01DCA",
      "cli-cyan": "#1AB2AD",
      "cli-white": "#FFFFFF",
    },
    extend: {
      fontFamily: {
        sans: ["JetBrains Mono", "monospace"],
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
