interface Section {
  title: string;
  description: string;
  previous: string;
  previousLink: string;
  next: string;
  nextLink: string;
  content: string;
}

const sections = [
  {
    title: "🚢 The Basics",
    subsections: [
      { title: "► What is Nautilus?", slug: "what-is-nautilus" },
      { title: "► How It Works", slug: "how-it-works" },
      { title: "► Installation", slug: "installation" },
    ],
  },
  {
    title: "⚛️ Core Concepts",
    subsections: [
      { title: "► Getting Started", slug: "getting-started" },
      { title: "► Wallets", slug: "wallets" },
      { title: "► Tokens", slug: "tokens" },
      { title: "► Tables", slug: "tables" },
      { title: "► State", slug: "state" },
    ],
  },
  {
    title: "🧰 Tools",
    subsections: [
      { title: "► Javascript SDK", slug: "javascript-sdk" },
      { title: "► Python SDK", slug: "python-sdk" },
    ],
  },
];

export default function DocumentationLayout({
  title,
  description,
  previous,
  previousLink,
  next,
  nextLink,
  content,
}: Section) {
  return (
    <div>
      <header className="sticky top-0 z-50 flex items-center justify-between px-3 py-2 border-b shadow-lg bg-white backdrop-blur-sm border-slate-400/40">
        <div className="flex items-center flex-grow basis-0">
          <img
            className="h-10 w-auto mr-2"
            src="/favicon.ico"
            alt="Nautilus Logo"
          />
          <a
            href="/"
            className="text-xl font-semibold tracking-tight text-slate-900 mr-4"
          >
            Nautilus
          </a>
          <code className="bg-slate-200 rounded-md text-md text-slate-600 px-2">
            0.0.1
          </code>
        </div>

        <form action="https://duckduckgo.com/" className="md:w-80 lg:w-96">
          <span className="relative flex items-center group">
            <svg
              aria-hidden="true"
              viewBox="0 0 20 20"
              className="absolute w-4 h-4 ml-3 fill-slate-400 group-hover:fill-slate-500 group-focus:fill-slate-500"
            >
              <path d="M16.293 17.707a1 1 0 0 0 1.414-1.414l-1.414 1.414ZM9 14a5 5 0 0 1-5-5H2a7 7 0 0 0 7 7v-2ZM4 9a5 5 0 0 1 5-5V2a7 7 0 0 0-7 7h2Zm5-5a5 5 0 0 1 5 5h2a7 7 0 0 0-7-7v2Zm8.707 12.293-3.757-3.757-1.414 1.414 3.757 3.757 1.414-1.414ZM14 9a4.98 4.98 0 0 1-1.464 3.536l1.414 1.414A6.98 6.98 0 0 0 16 9h-2Zm-1.464 3.536A4.98 4.98 0 0 1 9 14v2a6.98 6.98 0 0 0 4.95-2.05l-1.414-1.414Z"></path>
            </svg>
            <input
              type="text"
              name="q"
              placeholder="Search docs…"
              className="w-full py-2 pl-10 pr-2 border rounded bg-slate-100 placeholder-slate-400 text-slate-800 border-slate-100 outline outline-offset-2 outline-2 outline-transparent hover:border-slate-200 focus:border-slate-200 focus:outline-slate-600"
            />
          </span>
          <input type="hidden" name="sites" />
          <input type="submit" value="Search" className="sr-only" />
        </form>

        <div className="items-center justify-end flex-grow hidden basis-0 md:flex">
          <a
            href="https://github.com/nautilus-project/nautilus"
            className="px-4 py-2 text-sm font-semibold rounded bg-sky-500 text-slate-50 transition ease-in-out delay-75 hover:scale-105 duration-200"
          >
            GitHub
          </a>
        </div>
      </header>

      <main className="relative flex justify-center mx-auto max-w-8xl sm:px-2 lg:px-8 xl:px-12">
        <label className="fixed bottom-0 left-0 z-50 flex items-center justify-center w-12 h-12 mb-4 ml-4 bg-white border rounded-full shadow-lg cursor-pointer text-slate-600 border-slate-300 lg:hidden transition duration-200 ease-in-out active:scale-95">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="w-6 h-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth="2"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M4 8h16M4 16h16"
            />
          </svg>
        </label>

        <input
          type="checkbox"
          name="navigation"
          id="navigation"
          className="hidden peer"
        />
        <div className="fixed top-[3.5rem] h-screen shadow-xl px-4 left-0 hidden peer-checked:block lg:relative lg:top-0 lg:h-auto lg:px-0 lg:block lg:flex-none lg:shadow-none">
          <div className="absolute inset-y-0 right-0 w-full lg:w-[50vw] bg-white lg:bg-white"></div>

          <nav className="sticky top-[4.5rem] w-64 pr-8 text-base lg:text-sm xl:w-72 xl:pr-16">
            <ul
              role="list"
              className="-ml-0.5 h-[calc(100vh-4.5rem)] overflow-y-auto py-7 pl-0.5 space-y-8"
            >
              {sections.map((section) => (
                <li key={section.title}>
                  <h3 className="text-lg font-bold tracking-tight text-slate-900">
                    {section.title}
                  </h3>

                  <ul role="list" className="pl-3 mt-3 space-y-2">
                    {section.subsections.map((subsection) => (
                      <li key={subsection.title}>
                        <a
                          href={`/docs/${subsection.slug}`}
                          className="text-slate-600 hover:text-slate-800"
                        >
                          {subsection.title}
                        </a>
                      </li>
                    ))}
                  </ul>
                </li>
              ))}
            </ul>
          </nav>
        </div>

        <div className="flex-auto bg-white max-w-2xl min-w-0 px-4 py-10 lg:max-w-none lg:pr-0 lg:pl-8 xl:px-16">
          <h1 className="text-4xl font-bold tracking-tight mt-8 text-black">
            {title}
          </h1>
          <h3 className="text-xl font-semibold tracking-tight mt-4 mb-8 text-slate-400">
            {description}
          </h3>
          <div
            className="[&_code]:font-semibold [&_code]:text-black [&_h2]:font-bold [&_h2]:text-black [&_h2]:text-3xl [&_h2]:mt-8 [&_h2]:mb-4 [&_hr]:mb-4 [&_p]:text-slate-900 [&_b]:text-black [&_a]:text-sky-500 [&_a]:font-semibold [&_a]:text-lg [&_li]:list-disc [&_li]:text-slate-900"
            dangerouslySetInnerHTML={{ __html: content }}
          />
          <dl className="flex pt-6 mt-6 border-t border-slate-200">
            {previous && previousLink ? (
              <div className="mr-auto text-left">
                <dt className="text-sm font-normal tracking-tight text-slate-600">
                  Previous
                </dt>

                <dd className="mt-1">
                  <a
                    href={previousLink}
                    className="text-base font-semibold text-slate-900 hover:underline"
                  >
                    {previous}
                  </a>
                </dd>
              </div>
            ) : null}

            {next && nextLink ? (
              <div className="ml-auto text-right">
                <dt className="text-sm font-normal tracking-tight text-slate-600">
                  Next
                </dt>

                <dd className="mt-1">
                  <a
                    href={nextLink}
                    className="text-base font-semibold text-slate-900 hover:underline"
                  >
                    {next}
                  </a>
                </dd>
              </div>
            ) : null}
          </dl>
        </div>
      </main>
    </div>
  );
}
