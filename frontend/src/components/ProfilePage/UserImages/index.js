import { useDispatch, useSelector } from "react-redux";
import { useEffect, useState } from "react";

import "./UserImages.scss";
import { useModal } from "../../../context/Modal";
import { deleteImage, getUserImages } from "../../../store/images";

const GalleryModal = () => {
  const dispatch = useDispatch();

  let images = useSelector((state) => state.images.userImages);
  images = Object.values(images);

  const [outWidth, setOutWidth] = useState(0);
  const [loaded, setLoaded] = useState(false);
  const [waiting, setWaiting] = useState(false);

  useEffect(() => {
    dispatch(getUserImages()).then(() => setLoaded(true));
  }, [dispatch]);

  useEffect(() => {
    // Remove padding from top and bottom of modal container
    const modal = document.getElementById("modal-content");
    modal.style.padding = "0";
    modal.style.border = "3px solid #e29b49";

    const handleResize = () => {
      // Get width of gallery container
      const container = document.querySelector(".gallery-modal");
      const width = container.offsetWidth;
      setOutWidth(width);
    };

    if (loaded) handleResize();

    window.addEventListener("resize", handleResize);

    return () => window.removeEventListener("resize", handleResize);
  }, [loaded]);

  const handleDelete = (image) => async (e) => {
    e.preventDefault();

    setWaiting(true);
    await dispatch(deleteImage(image.id));
    await dispatch(getUserImages());
    setWaiting(false);
  };

  return (
    <div
      className="gallery-modal"
      style={waiting ? { justifyContent: "center", alignItems: "center" } : {}}
    >
      {waiting ? (
        <div className="loader" />
      ) : (
        images.map((image, idx) => (
          <div
            key={idx}
            className="gal-img"
            style={{
              width: `${(outWidth - 1) / 2 - 9}px`,
              height: `${(((outWidth - 1) / 2 - 9) * 2) / 3}px`,
            }}
            onClick={handleDelete(image)}
          >
            {<img src={image.url} alt="gallery item" />}
          </div>
        ))
      )}
    </div>
  );
};

const UserImages = () => {
  const dispatch = useDispatch();

  const { setModalContent } = useModal();

  let images = useSelector((state) => state.images.userImages);
  images = Object.values(images);
  const length = images.length;

  const [idx, setIdx] = useState(0);
  const [isLoaded, setIsLoaded] = useState(false);
  const [fade, setFade] = useState(null);

  useEffect(() => {
    dispatch(getUserImages()).then(() => setIsLoaded(true));
  }, [dispatch]);

  useEffect(() => {
    const interval = isLoaded
      ? setInterval(() => {
          setFade("fade");
          setIdx((idx + 1) % length);
          setTimeout(() => {
            setFade(null);
          }, 0);
        }, 4000)
      : null;
    return () => clearInterval(interval);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [images]);

  if (!isLoaded) return <>Loading...</>;

  return (
    <div
      className="user-images"
      onClick={() => setModalContent(<GalleryModal />)}
    >
      {length ? (
        <img className={fade ?? "img"} src={images[idx]?.url} alt="carousel" />
      ) : (
        <p>You haven't uploaded any images</p>
      )}
    </div>
  );
};

export default UserImages;
