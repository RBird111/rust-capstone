import StarRatingBar from "../../FormElements/StarRatingBar";
import "./BusinessFeedItem.scss";

const BusinessFeedItem = ({ business }) => {
  const { name, avg_rating, description, reviews } = business;
  const numReviews = Object.values(reviews).length;

  return (
    <div className="b-feed-item">
      <div className="name-rating">
        <p className="name">{name}</p>

        <div className="rating">
          <StarRatingBar rating={Math.round(Number(avg_rating))} />
          <p>
            {numReviews} review{numReviews === 1 ? "" : "s"}
          </p>
        </div>
      </div>

      <div className="description">
        <p className="d-title">Business description:</p>

        <p className="d-message">{description}</p>
      </div>
    </div>
  );
};

export default BusinessFeedItem;
