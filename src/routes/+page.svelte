<script lang="ts">
  // Components
  import ProjectCard from "$lib/ProjectCard.svelte";
  import Tabs from "$lib/Tabs.svelte";
  import SvelteSeo from "svelte-seo";

  const title = "Thierry Kühni | tey.sh";
  const description =
    "Hi there! You might have just found my website... Come on, get in :)";
  const keywords = "Thierry Kühni, Portfolio, Terminal, Developer";
</script>

<SvelteSeo
  {title}
  {description}
  {keywords}
  jsonLd={{
    "@context": "https://schema.org",
    "@type": "WebPage",
    name: title,
    description,
    keywords,
  }}
  openGraph={{ title, description, type: "website", site_name: "tey.sh" }}
  twitter={{
    card: "summary",
    title,
    description,
  }}
  nositelinkssearchbox={false}
  notranslate={false}
/>

<section>
  <h1>Hi, I'm Thierry</h1>
  <p>
    Better known online under my alias "TeyKey1". I'm an electrical engineer
    specializing in software and everything concerning bits and bytes. I have
    experience in a wide range of software-related fields, from bare-metal
    embedded systems to web apps and many things in between.
  </p>
</section>

<Tabs
  label="Skills"
  items={["Embedded", "Systems Programming", "Web"]}
>
  {#snippet selectedItemSlot(selectedItem)}
    {#if selectedItem === "Embedded"}
      <p>
        As an electrical engineer, I feel at home in embedded systems. I can
        accompany you, starting from the hardware design (KiCAD, VHDL) up to the
        development and testing of embedded software (C, Rust, Assembly).
      </p>
    {:else if selectedItem === "Systems Programming"}
      <p>
        With a background in industrial automation, I have experience in creating
        software with tight timing requirements running on B&R PLCs with real-time
        operating systems.
      </p>
      <p>
        I can also help you implement device drivers or other related programs
        using Rust.
      </p>
    {:else if selectedItem === "Web"}
      <p>
        I'm generally familiar with the essential web technologies. If you need
        someone to implement Webservers (Rust, NodeJS, TypeScript) or create
        websites (Vue, Svelte, Tailwind CSS), you've come to the right place :)
      </p>
      <p>
        While I can work with designers to create websites, I do not consider
        myself one. I'm also familiar with general system administration tasks
        encountered on web infrastructure.
      </p>
    {/if}
  {/snippet}
</Tabs>

<section>
  <h1>Projects</h1>
  <p>
    The following projects are all production-ready / actively used in
    production. For even more projects, feel free to check out my GitHub <a
      href="https://github.com/TeyKey1">profile</a
    >.
  </p>

  <ProjectCard
    title="Probe-rs Hive"
    tech="open-source, Rust, Vue, PCB and hardware design"
  >
    {#snippet desc()}
      <a href="https://probe.rs">probe-rs</a> is a modern open-source embedded debugging
      toolkit and an alternative to OpenOCD. Hive is a hardware-based test rack that
      can automatically test the probe-rs software against many combinations of debug
      probes and microcontroller targets.
    {/snippet}

    {#snippet more()}
      <div class="project-more">
        <p>
          This project was my bachelor's thesis, for which I received the top
          mark. The probe-rs debugger software can communicate with many debug
          probes and microcontroller targets. However, there was no way to
          reliably test if new probe-rs releases would work correctly on the many
          possible combinations of probes and targets.
        </p>

        <p>
          The goal of this project was to design and manufacture a hardware test
          rack, which should be able to test the probe-rs software automatically
          against many combinations of debug probes and microcontrollers and
          integrate seamlessly into the probe-rs rust codebase and its existing
          GitHub CI/CD pipeline. The probe-rs developers previously designed a
          hardware test rack but never manufactured it or wrote any software for
          it. This design was large and not portable. The client wanted a new
          hardware design which should be portable so all interested probe-rs
          developers would be able to manufacture a Hive test rack for themselves.
        </p>

        <p>
          The resulting test rack is designed as a combination of modular and
          compact Raspberry Pi shields. The testrack can connect up to four debug
          probes with up to 32 microcontrollers in all possible combinations. The
          Raspberry Pi runs the test server, which controls the whole test rack
          and communicates with the debug probes and the outside world.
        </p>

        <p>
          The Hive test harness integrates into the standard rust integration test
          environment, which allows developers to write the test functions for
          Hive in a very similar way to normal integration tests. Custom Rust
          macros alter the test functions so that the test runner on the server
          can run them directly on the hardware.
        </p>

        <p>
          With the Hive CLI tool, the developer (or a server) can connect to the
          Hive test rack over the internet and run the tests on the hardware. The
          test server runs the tests according to the instructions received from
          the CLI and reports the results back. To prevent malicious code from
          doing any damage to the test rack server, all remote code (probe-rs test
          candidate and test functions) is executed in a bubblewrap sandbox with
          Linux seccomp.
        </p>

        <p>
          Administration and maintenance of the test rack can be done comfortably
          using a Vue-based web frontend. Due to the general software architecture
          Hive is easily integratable into any CI/CD system.
        </p>

        <h3>Links</h3>
        <p>
          Repo: <a href="https://github.com/probe-rs/hive">Hardware</a>
          <a href="https://github.com/probe-rs/hive-software">Software</a>
          Docs: <a href="https://github.com/probe-rs/hive/wiki">Hardware</a>
          <a href="https://github.com/probe-rs/hive-software/wiki">Software</a>
        </p>
      </div>
    {/snippet}
  </ProjectCard>

  <ProjectCard
    title="FliegerWeb.com"
    tech="SvelteKit, Rust, TypeScript, Directus, Meilisearch, MySQL"
  >
    {#snippet desc()}
      <a href="https://fliegerweb.com">FliegerWeb</a> is one of the largest aviation
      news and information websites in the German-speaking region since 2008.
    {/snippet}

    {#snippet more()}
      <div class="project-more">
        <p>
          In 2022, I took over the administration and development of FliegerWeb.
          At the time the website was fully utilizing Java for the frontend and
          backend code (Struts 2, Tiles, Bootstrap) in combination with a MySQL
          database.
        </p>
        <p>
          I improved the SEO aspects of this application, but over time it became
          apparent that it would be a lot of effort to port the code from 2014 to
          today's standards in terms of dependency versions, general usability of
          the website, such as search, filtering, and usability on mobile devices.
        </p>
        <p>
          Given the custom backend also started to show some age with various
          smaller unfixed bugs and quirky workarounds, as well as the customer
          demanding extensive improvements over the current state, I decided to
          fully rebuild FliegerWeb using modern standards.
        </p>
        <p>
          The backend was built using Directus CMS, which was a massive time-saver
          as the general API and editing interface are working out of the box
          without the need to create a full-fledged CMS by hand. Most of the time
          was required to build scripts to fix a plethora of data inconsistencies
          (broken links, Images, wrong formatting, etc.) in the existing MySQL
          database due to previous reworks of FliegerWeb and bugs.
        </p>
        <p>
          The basic search engine used in the Java app was very limited in
          functionality and usability and thus replaced by Meilisearch, which
          massively improved the search experience and results on the site.
        </p>
        <p>
          Previously, FliegerWeb used a tag-based recommender system that relied
          on the authors of the articles correctly using tags to function
          properly. I built a new recommender system using Rust utilizing a TF-IDF
          algorithm as a base metric for the similarity of articles. Additionally,
          the system is able to incorporate a wide range of other metrics into its
          calculations such as tags, dates, article categories, etc.
        </p>
        <p>
          The front end was built using Svelte + SvelteKit with Tailwind CSS. The
          design and routing was kept as close as possible to the predecessor
          while moving to a fully responsive design suitable for mobile devices.
          The redesign focused on modernizing the look and feel of the site as
          well as giving the user more possibilities to easily filter and search
          the vast amount of content available, which was not possible in the
          previous version of the site.
        </p>

        <h3>Links</h3>
        <p>
          Live: <a href="https://fliegerweb.com">FliegerWeb.com</a>
          GitHub: <a href="https://github.com/Fliegerweb">FliegerWeb Repo</a> This
          is a closed-source project. Most repositories/code are not public.
        </p>
      </div>
    {/snippet}
  </ProjectCard>

  <ProjectCard title="svelte-konva" tech="open-source, Svelte, TypeScript">
    {#snippet desc()}
      svelte-konva is the official Svelte wrapper for the <a href="https://konvajs.org/">Konva 2 HTML5 canvas
      library</a>.
    {/snippet}

    {#snippet more()}
      <div class="project-more">
        <p>
          Konva allows users to easily draw shapes and listen to events on HTML5
          canvas using JavaScript. svelte-konva combines and leverages Svelte's
          reactivity system with the simplicity of Konva to enable users to create
          reactive HTML5 canvas applications easily in Svelte. The wrapper is well
          tested using Vitest, ensuring continuously high software quality. Being
          written in Typescript, users of the wrapper also benefit from
          well-working autocompletion and documentation of the Svelte components.
        </p>

        <h3>Links</h3>
        <p>
          Repo: <a href="https://github.com/konvajs/svelte-konva">svelte-konva</a>
          Docs: <a href="https://konvajs.org/docs/svelte/">svelte-konva</a>
        </p>
      </div>
    {/snippet}
  </ProjectCard>

  <ProjectCard title="PCA9535 embedded-hal driver" tech="open-source, Rust">
    {#snippet desc()}
      A Rust embedded-hal compatible driver for the PCA9535 IO-Expander chip.
    {/snippet}

    {#snippet more()}
      <div class="project-more">
        <p>
          The driver is fully compatible with the Rust embedded-hal, allowing it
          to be used with many different target systems. The driver is
          automatically tested against the real hardware using a Raspberry Pi,
          roughly similar to HIL-Testing.
        </p>

        <h3>Links</h3>
        <p>
          Repo: <a href="https://github.com/TeyKey1/pca9535">PCA9535</a>
          Docs: <a href="https://docs.rs/pca9535/latest/pca9535">PCA9535</a>
        </p>
      </div>
    {/snippet}
  </ProjectCard>
  <p></p>
</section>

<section>
  <h1>About me</h1>
  <p>
    My passion for programming was originally sparked at the age of 15 when I
    started creating modifications (like <a
      href="https://dev.bukkit.org/projects/theteleporter">this one</a
    >) for the video game Minecraft. At the time, those mods were written in
    Java, which I taught myself using online resources. Quickly followed the
    need to create custom websites for various gaming-related things. First
    using basic HTML and CSS, later using WordPress.
  </p>
  <p>
    After finishing mandatory school, I completed an apprenticeship as an
    automation technician, where I learned about various industrial-grade
    electronic equipment and technology such as PLCs and manufacturing robots.
    During the last two years of the apprenticeship, I was part of a team of
    engineers at General Electric, planning and coordinating the installation of
    electrical retrofits and upgrades for gas turbine power plants worldwide.
  </p>
  <p>
    Due to my general interest in computers and electronics, I chose to pursue a
    BSc in electrical engineering at the university of applied sciences <a
      href="https://www.fhnw.ch/en/about-fhnw">FHNW</a
    > where I specialized in embedded systems design.
  </p>
  <p>
    I had a fair share of education in languages such as Java, C, Python,
    Assembly, and VHDL during my studies. As I'm also interested in anything
    involving the web, I dived into technologies such as Node JS servers and Vue
    frontends for personal projects to connect embedded systems with beautiful
    web-based frontends.
  </p>
  <p>
    Out of personal interest (and growing necessity), I have spent considerable
    time learning and using various technologies and methodologies of software
    testing and CI/CD pipelines. Ever since this topic has been close to my
    heart as it has saved me a lot of time and worries :)
  </p>
  <p>
    Due to my BSc thesis (probe-rs Hive), I got pulled into open-source software
    development. Since then, I have contributed to and maintain several
    open-source projects. I have knowingly or unknowingly used and benefited
    from tons of (mostly free) open-source projects in my day-to-day life, and
    it is important to me to give something back to the best of my abilities.
  </p>
  <p>
    When I'm not programming, I might be <a
      href="https://pcpartpicker.com/b/7LBcCJ">building fancy computers</a
    > or just generally hand-crafting or repairing something in my workshop. I do
    bodyweight training to stay in shape. To kill time, I play video games or read
    books. I also listen to a lot of music and recently started to collect my favorite
    tracks as vinyl records.
  </p>
</section>

<style>
  .project-more {
    display: flex;
    flex-direction: column;
  }

  .project-more p {
    margin-top: 0px;
  }

  .project-more h3 {
    margin-top: 0px;
  }
</style>
