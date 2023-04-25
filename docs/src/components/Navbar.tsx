import { FC } from "react";

const Navbar: FC = () => {
  return (
    <div className="navbar mx-auto backdrop-blur-lg max-w-6xl rounded-xl">
      <div className="flex-1">
        <a className="btn btn-ghost normal-case text-xl">Nautilus</a>
      </div>
      <div className="flex-none">
        <ul className="menu menu-horizontal px-1">
          <li>
            <a className="text-slate-50 font-semibold">How It Works</a>
          </li>
          <li>
            <a
              className="text-slate-50 font-semibold"
              href="https://github.com/nautilus-project/nautilus"
            >
              GitHub
            </a>
          </li>
          <li>
            <a className="text-slate-50 font-semibold">Documentation</a>
          </li>
        </ul>
      </div>
    </div>
  );
};

export default Navbar;
