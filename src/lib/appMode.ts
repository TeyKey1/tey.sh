/**
 * The app can either display the content as a simple html&css page without the need of any JS (HTML-Mode) or
 * it displays the content in a more fancy fashion as a terminal using xterm and JS
 */

/**
 * App mode context key
 */
export const APP_MODE = "appmode";

export enum AppMode {
  Html,
  Terminal,
}
