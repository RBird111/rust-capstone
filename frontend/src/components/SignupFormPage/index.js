import { useEffect, useState } from "react";
import { useDispatch } from "react-redux";
import { signUp } from "../../store/session";
import { useModal } from "../../context/Modal";

import "./SignupForm.scss";
import FormInput, { handleErrors, toInput } from "../FormElements/FormInput";
import DefaultButton from "../FormElements/DefaultButton";

function SignupFormPage() {
  const dispatch = useDispatch();
  const { closeModal } = useModal();

  const [email, setEmail] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [first_name, setFirstName] = useState("");
  const [last_name, setLastName] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [errors, setErrors] = useState({});
  const [isSubmitted, setIsSubmitted] = useState(false);

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

    if (!password) errorsObj.password = "Password required";
    else if (password.length > 50)
      errorsObj.password = "Password must be less than 50 characters.";

    if (!confirmPassword) errorsObj.confirmPassword = "Must confirm Password";

    if (password !== confirmPassword) {
      errorsObj.password = "Confirm Password must be same as Password";
      errorsObj.confirmPassword = "Confirm Password must be same as Password";
    }

    setErrors(errorsObj);
  }, [confirmPassword, email, first_name, last_name, password, username]);

  const handleSubmit = async (e) => {
    e.preventDefault();

    setIsSubmitted(true);

    if (Object.values(errors).length === 0) {
      const user = {
        email,
        username,
        password,
        first_name,
        last_name,
      };

      const data = await dispatch(signUp(user));

      if (data) {
        const errorsObj = {};

        for (const error of data) {
          const [name, message] = error.split(" : ");
          errorsObj[name] = message;
        }

        return setErrors(errorsObj);
      }

      closeModal();
    }
  };

  return (
    <div className="signup-form">
      <h1>
        Sign Up for&nbsp;<span>W</span>help...
      </h1>

      <form onSubmit={handleSubmit}>
        <FormInput
          input={toInput("First Name", first_name, setFirstName)}
          handleErrors={handleErrors(isSubmitted, errors.first_name)}
        />

        <FormInput
          input={toInput("Last Name", last_name, setLastName)}
          handleErrors={handleErrors(isSubmitted, errors.last_name)}
        />

        <FormInput
          input={toInput("Email", email, setEmail)}
          handleErrors={handleErrors(isSubmitted, errors.email)}
        />

        <FormInput
          input={toInput("Username", username, setUsername)}
          handleErrors={handleErrors(isSubmitted, errors.username)}
        />

        <FormInput
          input={toInput("Password", password, setPassword, "password")}
          handleErrors={handleErrors(isSubmitted, errors.password)}
        />

        <FormInput
          input={toInput(
            "Confirm Password",
            confirmPassword,
            setConfirmPassword,
            "password"
          )}
          handleErrors={handleErrors(isSubmitted, errors.confirmPassword)}
        />

        <DefaultButton text={"Sign Up"} />
      </form>
    </div>
  );
}

export default SignupFormPage;
