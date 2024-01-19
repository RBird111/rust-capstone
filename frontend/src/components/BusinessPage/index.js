import { useDispatch, useSelector } from "react-redux";
import { useEffect, useState } from "react";
import { useHistory, useParams } from "react-router-dom";

import "./BusinessPage.scss";
import { useModal } from "../../context/Modal";
import { getBusiness, updateBusiness } from "../../store/business";
import StarRatingBar from "../FormElements/StarRatingBar";
import LoadingIcon from "../FormElements/LoadingIcon";
import ReviewForm from "../ReviewForm";
import LoginFormPage from "../LoginFormPage";
import BusinessForm from "../BusinessForm";
import ReviewFeedItem from "../ReviewFeedItem";
import UploadImages from "../UploadImages";

// Expects reviews to be normalized
const alreadyReviewed = (user, reviews) => {
  reviews = Object.values(reviews);
  for (const review of reviews) {
    if (review.user.email === user.email) {
      return review;
    }
  }
  return false;
};

const BusinessPage = () => {
  const dispatch = useDispatch();
  const history = useHistory();

  const { user } = useSelector((state) => state.session);

  const { setModalContent } = useModal();

  const { businessId } = useParams();

  const business = useSelector((state) => state.business.currBusiness);
  const { name, description, category, location, reviews, avg_rating, images } =
    business;

  const [isLoaded, setIsLoaded] = useState(false);

  const hasReview = user && reviews ? alreadyReviewed(user, reviews) : null;

  useEffect(() => {
    dispatch(getBusiness(businessId)).then((res) => {
      if (res.errors) history.push("/");
      else setIsLoaded(true);
    });
  }, [businessId, dispatch, history]);

  if (!isLoaded) return <LoadingIcon />;

  const iconClass = {
    restaurant: "fa-utensils",
    shopping: "fa-cart-shopping",
    automotive: "fa-car",
    "home services": "fa-house",
  };

  const claim = async (e) => {
    e.preventDefault();

    const businessData = {
      id: business.id,
      name: business.name,
      description: business.description,
      category: business.category,
      location_id: business.location_id,
      owner_id: user.id,
      address: business.location.address,
      state: business.location.state,
      city: business.location.city,
    };

    await dispatch(updateBusiness(businessData));
    await dispatch(getBusiness(business.id));
  };

  const style = () => {
    if (Object.values(images).length === 0)
      return { backgroundColor: "#9b3838" };

    return {
      backgroundImage: `linear-gradient(90deg, black, transparent), url(${
        Object.values(images)[
          Math.floor(Math.random() * Object.values(images).length)
        ].url
      })`,
    };
  };

  return (
    <div className="business-page">
      <div className="top-bar" style={{ ...style() }}>
        <div className="title">
          <div className="name">
            <h1>{name}</h1>
            {user && user.id === business.owner_id ? (
              <p
                onClick={() =>
                  setModalContent(<BusinessForm business={business} />)
                }
              >
                Update Business
              </p>
            ) : user && !business.owner_id ? (
              <p onClick={claim}>Claim Business!</p>
            ) : (
              <p className="claimed" style={{ cursor: "default" }}>
                Claimed <i className="fa-regular fa-circle-check" />
              </p>
            )}
          </div>

          {user && hasReview ? (
            <button
              className="add-review"
              onMouseEnter={(e) => {
                const star = document.querySelector(".fa-star");
                star.className = "fa-solid fa-star";
              }}
              onMouseLeave={(e) => {
                const star = document.querySelector(".fa-star");
                star.className = "fa-regular fa-star";
              }}
              onClick={() =>
                setModalContent(
                  <ReviewForm business={business} review={hasReview} />
                )
              }
            >
              <i className="fa-regular fa-star" />
              Update your review!
            </button>
          ) : user ? (
            <button
              className="add-review"
              onMouseEnter={(e) => {
                const star = document.querySelector(".fa-star");
                star.className = "fa-solid fa-star";
              }}
              onMouseLeave={(e) => {
                const star = document.querySelector(".fa-star");
                star.className = "fa-regular fa-star";
              }}
              onClick={() =>
                setModalContent(<ReviewForm business={business} />)
              }
            >
              <i className="fa-regular fa-star" />
              Write a review!
            </button>
          ) : (
            <button onClick={() => setModalContent(<LoginFormPage />)}>
              Log in to review!
            </button>
          )}
        </div>

        <div className="avg-rating-top">
          <StarRatingBar rating={Math.round(Number(avg_rating))} />

          <p className="reviews">
            {Object.values(reviews).length} review
            {Object.values(reviews).length === 1 ? "" : "s"}
          </p>
        </div>

        <p className="category">
          <i className={`fa-solid ${iconClass[category]}`} />
          {category[0].toUpperCase() + category.slice(1)}
        </p>

        <div className="btm-bit">
          <div className="location">
            {location.address}
            <p>
              {location.city}, {location.state}
            </p>
          </div>

          <div
            className="upload-image-btn"
            style={{ visibility: user ? "visible" : "hidden" }}
            onClick={() => {
              if (user)
                setModalContent(<UploadImages businessId={businessId} />);
            }}
          >
            <i className="fa-solid fa-camera" />
            <p>Upload Images!</p>
          </div>
        </div>
      </div>

      <div className="about">
        <div className="details">
          <p className="title">About this business:</p>
          <div className="description">{description}</div>
        </div>
      </div>

      <div className="featured-reviews">
        <div className="r-wrap">
          <p className="title">Featured Reviews:</p>

          <div className="reviews-frame">
            {isLoaded &&
              Object.values(reviews)
                .reverse()
                .map((review) => (
                  <ReviewFeedItem
                    key={review.id}
                    review={review}
                    userEmail={user?.email}
                    business={business}
                  />
                ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default BusinessPage;
