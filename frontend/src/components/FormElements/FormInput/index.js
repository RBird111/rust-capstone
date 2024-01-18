import "./FormInput.scss";
import Error from "../Error";

export const toInput = (label, value, onChange, type = "text") => ({
  label,
  value,
  onChange,
  type,
});

// Optional: to add error handling
export const handleErrors = (isSubmitted, error) => ({
  isSubmitted,
  error,
});

const FormInput = ({ input, handleErrors }) => {
  const { label, value, onChange, type } = input;
  const { isSubmitted, error } = handleErrors ? handleErrors : {};

  return (
    <div className="form-input">
      {isSubmitted && error && <Error error={error} />}

      <input
        type={type}
        name={label}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={label}
      />
    </div>
  );
};

export default FormInput;
