// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface Platform {}

    interface MdsvexFile {
      default: import("svelte/internal").SvelteComponent;
      metadata: Record<string, string>;
    }

    type MdsvexResolver = () => Promise<MdsvexFile>;

    interface TilPost {
      slug: string;
      title: string;
      author: string;
      description: string;
      date: string;
      published: boolean;
    }
  }
}

export {};
