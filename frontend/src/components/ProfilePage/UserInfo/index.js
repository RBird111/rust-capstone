import { useEffect, useRef, useState } from "react";
import { useDispatch } from "react-redux";

import "./UserInfo.scss";
import { updateUser } from "../../../store/session";
import Error from "../../FormElements/Error";

const UserInput = ({ value, label, onChange, user, form }) => {
  const [edit, setEdit] = useState(false);

  return (
    <div className="field">
      <p className="label">
        {label
          .split("_")
          .map((word) => word[0].toUpperCase() + word.slice(1))
          .join(" ")}
        :
      </p>
      <div className="disp-ele">
        {!edit ? (
          <p className="value">
            {value}
            <i
              className="fa-solid fa-pen-to-square"
              onClick={() => setEdit(true)}
            />
          </p>
        ) : (
          <>
            <input
              type="text"
              value={value}
              onChange={(e) => onChange(e.target.value)}
            />
            <button className="submit">
              <i
                className="fa-solid fa-check"
                onClick={() => {
                  setEdit(false);
                  form.current.dispatchEvent(
                    new Event("submit", { cancelable: true, bubbles: true })
                  );
                }}
              />
            </button>
            <i
              className="fa-sharp fa-solid fa-xmark"
              onClick={() => {
                onChange(user[label]);
                setEdit(false);
              }}
            />
          </>
        )}
      </div>
    </div>
  );
};

const UserInfo = ({ user }) => {
  const dispatch = useDispatch();

  const formRef = useRef();

  const [first_name, setFirstName] = useState(user.first_name);
  const [last_name, setLastName] = useState(user.last_name);
  const [username, setUsername] = useState(user.username);
  const [email, setEmail] = useState(user.email);

  const [errors, setErrors] = useState({});

  // Validations
  useEffect(() => {
    setErrors({});
    const errorsObj = {};

    if (!first_name) errorsObj.first_name = "First Name required";
    else if (first_name.length > 40)
      errorsObj.first_name = "First Name must be less than 40 characters";

    if (!last_name) errorsObj.last_name = "Last Name required";
    else if (last_name.length > 40)
      errorsObj.last_name = "Last Name must be less than 40 characters";

    if (!username) errorsObj.username = "Username required";
    else if (username.length > 40)
      errorsObj.username = "Username must be less than 40 characters";

    if (!email) errorsObj.email = "Email required";
    else if (email.length > 40)
      errorsObj.email = "Email must be less than 40 characters";
    else if (email && !email.match(/[\w\-_$@!#%;^&?]+@\w+\.\w+/))
      errorsObj.email = "Email must be valid";

    setErrors(errorsObj);
  }, [email, first_name, last_name, username]);

  const handleSubmit = async (e) => {
    e.preventDefault();

    if (Object.values(errors).length === 0) {
      const updatedUser = {
        user_id: user.id,
        email,
        username,
        first_name,
        last_name,
      };

      const data = await dispatch(updateUser(updatedUser));

      if (data) {
        const errorsObj = {};

        for (const error of data) {
          const [name, message] = error.split(" : ");
          errorsObj[name] = message;
        }

        return setErrors(errorsObj);
      }
    }
  };

  return (
    <div className="user-info-form">
      <form className="update-user" onSubmit={handleSubmit} ref={formRef}>
        {[
          [first_name, "first_name", setFirstName],
          [last_name, "last_name", setLastName],
          [username, "username", setUsername],
          [email, "email", setEmail],
        ].map(([value, label, onChange], idx) => (
          <div key={idx}>
            {errors[label] ? (
              <Error error={errors[label]} />
            ) : (
              <p style={{ height: "17px" }}></p>
            )}
            <UserInput
              value={value}
              label={label}
              onChange={onChange}
              user={user}
              form={formRef}
            />
          </div>
        ))}
      </form>
    </div>
  );
};

export default UserInfo;
