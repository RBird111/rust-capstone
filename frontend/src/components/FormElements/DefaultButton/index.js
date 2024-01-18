import "./DefaultButton.scss";

const DefaultButton = ({ text, onClick, color = "red" }) => {
  return (
    <button className={`default-btn ${color}`} type="submit" onClick={onClick}>
      {text}
    </button>
  );
};

export default DefaultButton;
