IMAGE_NAME="youodf/sample-job"
IMAGE_TAG="1.0.0"

docker build -t $IMAGE_NAME:$IMAGE_TAG .
docker push $IMAGE_NAME:$IMAGE_TAG