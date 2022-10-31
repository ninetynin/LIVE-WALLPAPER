import cv2;
import numpy as np
import os;

vid_path = "../../modify-assets/sample.mp4"

cam = cv2.VideoCapture(vid_path)

if not os.path.exists('images'):
    os.makedirs('images')
    # DONT NEED THIS IN MAIN CODE
    
index = 000
while(True):
    ret, frame = cam.read()
    if not ret:
        break
    name = './images/frame' + str(index) + '.jpg'
    print('Creating...' + name)
    cv2.imwrite(name, frame)
    index += 1