import "./Footer.scss";

const Footer = () => {
  return (
    <div className="footer">
      <div className="personal-info">
        <p className="name">Roosevelt Burden</p>

        <a target="_blank" rel="noreferrer" href="https://github.com/RBird111">
          <i className="fa-brands fa-github fa-lg" />
        </a>

        <a
          target="_blank"
          rel="noreferrer"
          href="https://www.linkedin.com/in/roosevelt-burden-83982026b"
        >
          <i className="fa-brands fa-linkedin fa-lg" />
        </a>
      </div>

      <div className="repo-link">
        <a
          target="_blank"
          rel="noreferrer"
          href="https://github.com/RBird111/rust-capstone"
        >
          <i className="fa-brands fa-python fa-lg" />
          <i className="fa-brands fa-css3 fa-lg" />
          GitHub Repo
          <i className="fa-brands fa-html5 fa-lg" />
          <i className="fa-brands fa-js fa-lg" />
        </a>
      </div>

      <div>
        <p>@2023</p>
      </div>
    </div>
  );
};

export default Footer;
