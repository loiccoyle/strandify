import React, { useState } from "react";
import { Sun, Moon } from "lucide-react";
import { useTheme } from "../contexts/ThemeContext";

const Header: React.FC = () => {
  const { theme, toggleTheme } = useTheme();
  const [showTooltip, setShowTooltip] = useState(false);

  const handleGitHubClick = () => {
    window.open("https://github.com/loiccoyle/strandify", "_blank");
  };

  return (
    <header className="mb-4">
      <div className="flex items-center mb-2">
        <div className="flex flex-col md:flex-row items-center justify-around gap-2 w-full cursor-default">
          <h1 className="text-5xl font-extrabold tracking-tight dark:text-white text-gray-800">
            Strandify
          </h1>
          <p className="text-base text-center grow sm:text-lg text-gray-600 dark:text-gray-300 italic relative">
            <span className="sm:inline hidden">Effortless string art, </span>
            <span className="sm:hidden inline">String art,</span>
            <span
              className="underline relative"
              onClick={() => setShowTooltip(!showTooltip)}
              onMouseEnter={() => setShowTooltip(true)}
              onMouseLeave={() => setShowTooltip(false)}
            >
              no strings attached
              <span
                className={`absolute left-1/2 transform -translate-x-1/2 mt-6 p-2 w-64 text-xs text-gray-800 dark:text-gray-200 bg-white dark:bg-black rounded-lg shadow-lg z-10 transition-opacity duration-300 ${
                  showTooltip ? "opacity-100 visible" : "opacity-0 invisible"
                }`}
              >
                Free and{" "}
                <a
                  href="https://github.com/loiccoyle/strandify/tree/gh-pages"
                  className="underline font-bold"
                  target="_blank"
                >
                  open-source
                </a>
                . All processing is done locally. Your photos never leave your
                device.
              </span>
            </span>
          </p>
        </div>
        <div className="flex justify-end mb-2 gap-2">
          <button
            onClick={handleGitHubClick}
            className="p-2 rounded-full bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
            aria-label="View GitHub repository"
          >
            <svg
              className="w-6 h-6"
              viewBox="0 0 24 24"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="M12 2C6.477 2 2 6.477 2 12c0 4.419 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.238-.008-.866-.013-1.699-2.782.603-3.369-1.342-3.369-1.342-.454-1.155-1.11-1.462-1.11-1.462-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.831.092-.647.35-1.088.636-1.338-2.22-.252-4.555-1.111-4.555-4.943 0-1.091.39-1.984 1.029-2.688-.103-.253-.446-1.27.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.026 2.747-1.026.546 1.38.202 2.398.1 2.651.64.704 1.028 1.597 1.028 2.688 0 3.848-2.339 4.687-4.566 4.935.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12c0-5.523-4.477-10-10-10z"
                fill="currentColor"
              />
            </svg>
          </button>
          <button
            onClick={toggleTheme}
            className="p-2 rounded-full bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
            aria-label="Toggle dark mode"
          >
            {theme === "light" ? (
              <Moon className="w-6 h-6" />
            ) : (
              <Sun className="w-6 h-6" />
            )}
          </button>
        </div>
      </div>
    </header>
  );
};

export default Header;
