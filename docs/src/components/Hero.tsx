import styles from "@/styles/Transitions.module.css";
import Link from "next/link";

export default function Hero() {
  return (
    <div className="mx-auto max-w-2xl lg:py-32 py-28">
      <div className={styles.fadeInUp}>
        <div className="flex justify-center">
          <img src="/nautilus-icon.jpg" alt="Nautilus Logo" />
        </div>
        <div className="text-center">
          <h1 className="text-6xl font-bold tracking-tight text-slate-50 drop-shadow-lg sm:text-6xl">
            Set Sail With{" "}
            <span className="text-6xl font-bold tracking-tight text-slate-50 drop-shadow-lg underline decoration-sky-500 sm:text-6xl">
              Nautilus
            </span>
          </h1>
          <p className="lg:mt-2 mt-5 text-lg lg:leading-8 text-slate-200 leading-[24px]">
            An object-oriented, SQL-native Solana programming framework
          </p>
          <div className="mt-4 mb-12 flex items-center justify-center gap-x-6">
            <button className="mt-2 btn btn-primary border-none rounded-md bg-sky-500 outline-none hover:bg-sky-600 focus:ring-cyan-400">
              Get Started
            </button>
            <Link className="mt-2" href="/docs/what-is-nautilus">
              <button className="btn btn-ghost font-semibold text-slate-50">
                Docs{" "}
                <span className="text-slate-50 ml-2" aria-hidden="true">
                  {"  "}→
                </span>
              </button>
            </Link>
          </div>
        </div>
      </div>
    </div>
  );
}
