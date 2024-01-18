import "./LoadingIcon.scss";

const LoadingIcon = () => {
  return (
    <div className="loading-div">
      <p>Loading...</p>
      <div className="loading-icon">
        <div className="inner-loading-icon">
          <div className="inner-inner-loading-icon" />
        </div>
      </div>
    </div>
  );
};

export default LoadingIcon;
