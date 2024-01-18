import { useHistory } from "react-router-dom";
import { useModal } from "../../context/Modal";
import ProfileIcon from "../FormElements/ProfileIcon";
import StarRatingBar from "../FormElements/StarRatingBar";
import ReviewForm from "../ReviewForm";
import "./ReviewFeedItem.scss";

const ReviewFeedItem = ({ review, userEmail, business, aboutMe }) => {
  const history = useHistory();

  const { user, rating, body } = review;
  const { setModalContent } = useModal();

  const userOwned = userEmail === user.email;

  const style = () => {
    if (!userOwned) return {};
    if (aboutMe) return { outline: "2px solid #4990e2" };
    return {
      outline: "2px solid #00ac82",
    };
  };

  return (
    <div className="review-feed-item" style={{ ...style() }}>
      <div className="name-tag">
        {!aboutMe && <ProfileIcon />}

        {userOwned && aboutMe ? (
          <p
            onClick={() => history.push(`/business/${business.id}`)}
            style={{
              cursor: "pointer",
              marginLeft: "0",
              color: "#00ac82",
              fontSize: "28px",
              fontFamily: `"Amatic SC", cursive`,
            }}
          >
            {business.name}
          </p>
        ) : userOwned ? (
          <p>You</p>
        ) : (
          <p>
            {user.first_name} {user.last_name[0]}.
          </p>
        )}

        {userOwned && (
          <div
            className="edit-icon"
            title="Click to edit your review"
            onClick={() =>
              setModalContent(
                <ReviewForm business={business} review={review} />
              )
            }
          >
            <i className="fa-solid fa-pen-to-square fa-lg" />
          </div>
        )}
      </div>

      <StarRatingBar rating={rating} />

      <p className="body">{body}</p>
    </div>
  );
};

export default ReviewFeedItem;
