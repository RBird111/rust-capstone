import { useHistory } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";
import { useEffect, useState } from "react";

import "./ProfilePage.scss";
import LoadingIcon from "../FormElements/LoadingIcon";
import UserInfo from "./UserInfo";
import ReviewFeedItem from "../ReviewFeedItem";
import { getUserReviews } from "../../store/reviews";
import UserImages from "./UserImages";

const ProfilePage = () => {
  const dispatch = useDispatch();
  const history = useHistory();

  const user = useSelector((state) => state.session.user);
  const userReviews = useSelector((state) => state.reviews.userReviews);

  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    if (!user) history.push("/");
    else dispatch(getUserReviews()).then(() => setIsLoaded(true));
  }, [dispatch, history, user]);

  if (!isLoaded) return <LoadingIcon />;

  return (
    <div className="profile-page">
      <div className="acc-imgs">
        <div className="account">
          <h1>Account Information</h1>
          <UserInfo user={user} />
        </div>

        <div className="imgs">
          <h1>My Images</h1>
          <UserImages user={user} />
        </div>
      </div>

      <div className="personal-reviews">
        <h1>My Reviews</h1>
        <div className="user-review-items">
          {Object.values(userReviews).map((review) => (
            <ReviewFeedItem
              key={review.id}
              review={review}
              userEmail={user.email}
              business={review.business}
              aboutMe={true}
            />
          ))}
        </div>
      </div>
    </div>
  );
};

export default ProfilePage;
