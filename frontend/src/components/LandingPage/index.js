import { useDispatch, useSelector } from "react-redux";
import { useEffect, useState } from "react";
import { getRandomReviews } from "../../store/reviews";

import "./LandingPage.scss";
import ReviewCard from "../ReviewCard";
import CategoryCard from "../CategoryCard";
import LoadingIcon from "../FormElements/LoadingIcon";

const LandingPage = () => {
  const dispatch = useDispatch();

  let reviews = useSelector((state) => state.reviews.randomReviews);
  reviews = Object.values(reviews);

  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    dispatch(getRandomReviews(10)).then(() => setIsLoaded(true));
  }, [dispatch]);

  if (!isLoaded) return <LoadingIcon />;

  // Images for category buttons
  const ctgImages = reviews.map(
    (review) => Object.values(review.images)[0]?.url
  );

  return (
    <div className="landing-page">
      {/* Categories */}
      <h1 className="title">Categories</h1>
      <div className="categories">
        {["automotive", "home services", "restaurant", "shopping"].map(
          (service, idx) => (
            <CategoryCard key={idx} category={service} url={ctgImages[idx]} />
          )
        )}
      </div>

      {/* Recent Activity */}
      <h1 className="title">Recent Activity</h1>
      <div className="reviews">
        {reviews.slice(0, 6).map((review) => (
          <ReviewCard key={review.id} review={review} />
        ))}
      </div>
    </div>
  );
};

export default LandingPage;
