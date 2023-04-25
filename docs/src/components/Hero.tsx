import { FC } from "react";
import Navbar from "./Navbar";
import styles from "@/styles/Transitions.module.css";

const Hero: FC = () => {
  return (
    <div className="bg-[url(https://i.imgur.com/Fq6zXIb.jpg)] backdrop-blur-md bg-no-repeat bg-cover bg-center bg-fixed">
      <div className="bg-cover bg-fixed backdrop-blur-md pt-2">
        <div className={styles.fadeInUp}>
          <Navbar />
        </div>
        <div className="mx-auto max-w-2xl py-32">
          <div className={styles.fadeInUp}>
            <div className="hidden sm:mb-8 sm:flex sm:justify-center">
              <img src="https://i.imgur.com/BwfvrEt.png" alt="Nautilus Logo" />
            </div>
            <div className="text-center">
              <h1 className="text-6xl font-bold tracking-tight text-slate-50 drop-shadow-lg sm:text-6xl">
                Set Sail With{" "}
                <span className="text-6xl font-bold tracking-tight text-slate-50 drop-shadow-lg underline decoration-sky-500 sm:text-6xl">
                  Nautilus
                </span>
              </h1>
              <p className="mt-6 text-lg leading-8 text-slate-200">
                An object-oriented, SQL-native Solana programming framework
              </p>
              <div className="mt-10 mb-12 flex items-center justify-center gap-x-6">
                <button className="btn btn-primary rounded-md bg-sky-500 outline-none hover:bg-sky-600 focus:ring-cyan-400">
                  Get Started
                </button>
                <button className="btn btn-ghost font-semibold text-slate-50">
                  Docs{" "}
                  <span className="text-slate-50" aria-hidden="true">
                    {" "}
                     →
                  </span>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Hero;
