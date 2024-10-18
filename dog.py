import argparse
import skimage
import numpy as np


parser = argparse.ArgumentParser()
parser.add_argument("image", type=str)
parser.add_argument("-s", "--small", type=float)
parser.add_argument("-l", "--large", type=float)
args = parser.parse_args()
image_name, image_extension = args.image.split(".")
image = skimage.io.imread(args.image)
low_sigma = args.small
high_sigma = args.large
image = image[:,:,0:3]
image = skimage.color.rgb2gray(image)
#image = skimage.restoration.denoise_bilateral(image,sigma_color = 0.1)
image = skimage.filters.difference_of_gaussians(image,low_sigma,high_sigma,channel_axis=2)
image = image < skimage.filters.threshold_triangle(image)
print(skimage.filters.threshold_triangle(image))

skimage.io.imsave(image_name + "_pp_dog" + "." +image_extension,skimage.util.img_as_ubyte(image))