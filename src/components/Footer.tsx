import React from "react";

const Footer: React.FC = () => {
  return (
    <footer className="text-center text-gray-600 dark:text-gray-300 text-xs">
      <p>
        Made with ❤️ by{" "}
        <a
          href="https://loiccoyle.com"
          className="underline font-bold"
          target="_blank"
        >
          Loïc Coyle
        </a>
      </p>
      It broke? Open an{" "}
      <a
        href="https://github.com/loiccoyle/strandify/issues/new"
        className="underline font-bold"
        target="_blank"
      >
        issue
      </a>
    </footer>
  );
};

export default Footer;
