import "./ProfileIcon.scss";

const ProfileIcon = ({ props }) => {
  return (
    <div {...props} className="icon-div">
      <i className="profile-icon fa-solid fa-user" />
    </div>
  );
};

export default ProfileIcon;
