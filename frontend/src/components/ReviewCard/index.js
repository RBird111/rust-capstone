import { NavLink, useHistory } from "react-router-dom";

import "./ReviewCard.scss";
import StarRatingBar from "../FormElements/StarRatingBar";
import ProfileIcon from "../FormElements/ProfileIcon";

const ReviewCard = ({ review }) => {
  const history = useHistory();

  const imageUrl = Object.values(review.images)[0].url;

  return (
    <div className="review-card">
      <div
        className="background"
        style={{
          backgroundImage: `url(${imageUrl})`,
        }}
      />

      <div className="wrap">
        <div className="user-info">
          <ProfileIcon />
          <div className="p-div">
            <p className="name">
              {review.user.first_name} {review.user.last_name[0]}.
            </p>
            <p className="wrote">Wrote a review</p>
          </div>
        </div>

        <div className="star-rating-bar">
          <NavLink to={`/business/${review.business.id}`}>
            {review.business.name}
          </NavLink>
          <StarRatingBar rating={review.rating} />
        </div>
      </div>

      <div className="r-wrap">
        <div className="review-body">
          <p className="body">{review.body}</p>
          <p
            className="continue"
            onClick={() => history.push(`/business/${review.business.id}`)}
          >
            Continue reading
          </p>
        </div>

        <div className="icon-bar">
          {[
            "fa-regular fa-lightbulb fa-xl",
            "fa-regular fa-face-laugh fa-xl",
            "fa-regular fa-face-sad-cry fa-xl",
          ].map((className, idx) => (
            <div className="icon" key={idx}>
              <i
                onClick={() => alert("Feature not yet implemented")}
                className={className}
              />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default ReviewCard;
