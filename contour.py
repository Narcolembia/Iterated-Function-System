import argparse
import cv2
import numpy as np

parser = argparse.ArgumentParser()
parser.add_argument("image", type=str)
parser.add_argument("-s", "--step", type=int, default=10)
parser.add_argument("-b", "--blur", type=int, default=50)
args = parser.parse_args()

# Make blur odd
if args.blur % 2 == 0:
    args.blur += 1

# Split image name and extension
image_name, image_extension = args.image.split(".")

# Read image
image = cv2.imread(args.image)

# Blur the image
image = cv2.GaussianBlur(image, (args.blur, args.blur), 0)

# Create a blank image with the same shape
contour_image = np.zeros_like(image)

for i in range(0, 255, args.step):
    # Convert image to grayscale
    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)

    # Apply threshold
    _, thresh = cv2.threshold(gray, i, 255, cv2.THRESH_BINARY)

    # Find contour lines
    contours, _ = cv2.findContours(thresh, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)

    # Draw contour lines
    cv2.drawContours(contour_image, contours, -1, (250, 200,200), 2)

cv2.imwrite(f"{image_name}-pp.jpg", contour_image)