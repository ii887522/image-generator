docker run --rm --name image-generator_deployer -w C:\image-generator -v %CD%:C:\image-generator image-generator-custom-node node deploy.js %1 %2
