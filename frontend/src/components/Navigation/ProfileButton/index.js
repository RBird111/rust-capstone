import { useState, useEffect, useRef } from "react";
import { useDispatch } from "react-redux";
import { useHistory } from "react-router-dom";

import "./ProfileButton.scss";
import { logout } from "../../../store/session";
import ProfileIcon from "../../FormElements/ProfileIcon";

function ProfileButton({ user }) {
  const dispatch = useDispatch();
  const history = useHistory();

  const [showMenu, setShowMenu] = useState(false);
  const ulRef = useRef();

  const openMenu = () => {
    if (showMenu) return;
    setShowMenu(true);
  };

  useEffect(() => {
    if (!showMenu) return;

    const closeMenu = (e) => {
      if (!ulRef.current.contains(e.target)) {
        setShowMenu(false);
      }
    };

    document.addEventListener("click", closeMenu);

    return () => document.removeEventListener("click", closeMenu);
  }, [showMenu]);

  const handleLogout = (e) => {
    e.preventDefault();
    history.push("/");
    dispatch(logout());
  };

  const divClassName = "profile-dropdown" + (showMenu ? "" : " hidden");

  return (
    <div className="profile-button-div">
      <ProfileIcon props={{ onClick: openMenu }} />

      <div className={divClassName} ref={ulRef}>
        {user && (
          <>
            <div className="header">
              <p>
                {user.first_name} {user.last_name[0]}.
              </p>
            </div>

            <p
              className="about"
              onClick={() => {
                setShowMenu(false);
                history.push("/profile");
              }}
            >
              <i className="fa-regular fa-circle-user" />
              About Me
            </p>

            <p className="logout" onClick={handleLogout}>
              <i className="fa-solid fa-arrow-right-from-bracket" />
              Log Out
            </p>
          </>
        )}
      </div>
    </div>
  );
}

export default ProfileButton;
