import { NavLink } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";

import "./Navigation.scss";
import { useModal } from "../../context/Modal";
import { login } from "../../store/session";
import LoginFormPage from "../LoginFormPage";
import Logo from "../FormElements/Logo";
import OpenModalButton from "../OpenModalButton";
import ProfileButton from "./ProfileButton";
import SignupFormPage from "../SignupFormPage";
import BusinessForm from "../BusinessForm";
import SearchBar from "./SearchBar";
import { useEffect } from "react";

function Navigation({ isLoaded }) {
  const dispatch = useDispatch();

  const { setModalContent } = useModal();

  const sessionUser = useSelector((state) => state.session.user);

  useEffect(() => {}, [sessionUser]);

  const loginDemo = async (e) => {
    e.preventDefault();
    const demo = await (await fetch(`/api/user/1`)).json();
    await dispatch(login({ credential: demo.username, password: "password" }));
  };

  return (
    <div className="nav-bar-div">
      <NavLink className="nav-logo" exact to="/">
        {/* whelp... */}
        <Logo />
      </NavLink>

      <SearchBar />

      {isLoaded && sessionUser ? (
        <div className="profile-div">
          <p
            className="add-business"
            onClick={() => setModalContent(<BusinessForm />)}
          >
            Add a Business
          </p>
          <ProfileButton user={sessionUser} />
        </div>
      ) : (
        <div className="logged-out">
          <p onClick={loginDemo}>Log In as Demo User</p>

          <OpenModalButton
            buttonText={"Log In"}
            modalComponent={<LoginFormPage />}
            color={"white"}
          />

          <OpenModalButton
            buttonText={"Sign Up"}
            modalComponent={<SignupFormPage />}
          />
        </div>
      )}
    </div>
  );
}

export default Navigation;
