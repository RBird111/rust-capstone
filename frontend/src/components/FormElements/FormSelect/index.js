import "./FormSelect.scss";
import Error from "../Error";

export const toSelectInput = (value, onChange, options) => ({
  value,
  onChange,
  options,
});

// Optional: to add error handling
export const handleErrors = (isSubmitted, error) => ({
  isSubmitted,
  error,
});

const FormSelect = ({ input, handleErrors }) => {
  const { value, onChange, options } = input;
  const { isSubmitted, error } = handleErrors;

  return (
    <div className="form-select">
      {isSubmitted && error && <Error error={error} />}

      <select value={value} onChange={(e) => onChange(e.target.value)}>
        <option value={""}>Select a Category</option>
        {options.map((opt, idx) => (
          <option key={idx} value={opt}>
            {opt[0].toUpperCase() + opt.slice(1)}
          </option>
        ))}
      </select>
    </div>
  );
};

export default FormSelect;
