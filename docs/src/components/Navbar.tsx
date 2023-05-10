import Link from "next/link";
import { useState } from "react";

export default function Navbar() {
  const [toggled, setToggled] = useState(false);

  const handleToggle = () => {
    setToggled(!toggled);
  };

  return (
    <>
      <div className="lg:flex hidden flex-row items-center justify-between mt-4 fixed top-0 w-full px-10">
        <div className="w-[25%]">
          <img className="h-28" src="/nautilus-icon.jpg" alt="Nautilus Logo" />
        </div>

        <div className="flex flex-row gap-x-4 items-center w-[60%] justify-center">
          <Link
            className="text-slate-300 font-semibold text-lg hover:text-white duration-500"
            href=""
          >
            How it Works
          </Link>
          <Link
            className="text-slate-300 font-semibold text-lg hover:text-white duration-500"
            href="https://github.com/nautilus-project/nautilus"
          >
            GitHub
          </Link>
          <Link
            className="text-slate-300 font-semibold text-lg hover:text-white duration-500"
            href="/docs/what-is-nautilus"
          >
            Documentation
          </Link>
        </div>

        <div className="w-[25%]" />
      </div>

      <div
        className={
          toggled
            ? "lg:hidden flex flex-col w-full px-2 transition-all duration-300 fixed z-50 drop-shadow-lg"
            : "lg:hidden flex flex-col absolute w-full px-2 transition-all duration-300"
        }
        style={
          toggled
            ? {
                backgroundColor: "hsla(var(--b1) / var(--tw-bg-opacity, 1))",
              }
            : {}
        }
      >
        <div className="flex flex-row items-center justify-between mt-4">
          <div className="w-1/2">
            <img
              className="h-[5.5rem]"
              src="/nautilus-icon.jpg"
              alt="Nautilus Logo"
            />
          </div>

          <div className="w-1/2 flex justify-end mr-2">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              strokeWidth={1.5}
              stroke="currentColor"
              className="w-10 h-10"
              onClick={handleToggle}
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="M3.75 6.75h16.5M3.75 12h16.5M12 17.25h8.25"
              />
            </svg>
          </div>
        </div>

        {toggled && (
          <>
            <div className="px-5 py-4">
              <div className="flex flex-col gap-y-2 w-full">
                <Link
                  className="text-slate-300 font-semibold text-base hover:text-white duration-500"
                  href=""
                >
                  How it Works
                </Link>
                <Link
                  className="text-slate-300 font-semibold text-base hover:text-white duration-500"
                  href="https://github.com/nautilus-project/nautilus"
                >
                  GitHub
                </Link>
                <Link
                  className="text-slate-300 font-semibold text-base hover:text-white duration-500"
                  href="/docs/what-is-nautilus"
                >
                  Documentation
                </Link>
              </div>
            </div>
          </>
        )}
      </div>
    </>
  );
}
